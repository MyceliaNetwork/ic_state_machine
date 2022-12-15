use std::collections::VecDeque;
use crate::message_channel::{create_channel, MessageReceiver, MessageSender};
use crate::state::{BoxedState, DeliveryStatus, State, StateType, Transition};

pub type StateMachineId = String;

pub struct StateMachineHandle<IncomingMessages : Clone> {
    tx: MessageSender<IncomingMessages>,
}

impl <IncomingMessages : Clone> StateMachineHandle<IncomingMessages> {
    pub fn clone(&self) -> Self {
        StateMachineHandle {
            tx: self.tx.clone(),
        }
    }
}

impl<IncomingMessages : Clone> StateMachineHandle<IncomingMessages> {
    pub fn send(&self, message: IncomingMessages) -> Result<(), ()> {
        self.tx.try_send(message)
    }
}

#[derive(Debug, PartialEq)]
pub enum StepResult {
    Running,
    Terminated,
}

#[derive(Debug, PartialEq)]
pub struct StateMachineError {
    message : String,
}

pub struct StateMachine<Types: StateType> {
    state_machine_id: String,
    state: BoxedState<Types>,
    message_queue: VecDeque<Types::In>,
    is_state_initialized: bool,

    // Receives messages for states
    inbound_message_channel: MessageReceiver<Types::In>,
    // Sends messages from states
    outbound_message_channel: MessageSender<Types::Out>,
}

impl<Types> StateMachine<Types>
    where Types: 'static + StateType
{
    /// Create a new state machine with the given initial state.
    /// Return a StateMachine and StateMachineHandle that can be used to send messages to the state machine.
    pub fn new(state_machine_id: String, outbound_message_channel: MessageSender<Types::Out>, state: Box<dyn State<Types>>) -> (StateMachine<Types>, StateMachineHandle<Types::In>) {
        let (tx, inbound_message_channel) = create_channel::<Types::In>();

        (
            StateMachine {
                state_machine_id,
                state,
                message_queue: VecDeque::new(),
                is_state_initialized: false,
                inbound_message_channel,
                outbound_message_channel,
            },
            StateMachineHandle {
                tx
            }
        )
    }

    /// Return the current state of the machine
    pub fn state(&self) -> &dyn State<Types> {
        &*self.state
    }

    /// Attempt to return the current state of the machine downcast to the given type.
    pub fn downcast_state<T>(&self) -> Option<&T>
        where T: State<Types>
    {
        self.state.downcast_ref::<T>()
    }

    /// Drive the state machine forward by processing a messages in the queue and advancing the state.
    pub fn step(&mut self) -> Result<StepResult, StateMachineError> {
        // If the current state is not initialized do that first
        if !self.is_state_initialized {
            let messages = self.state.initialize();
            messages.into_iter().for_each(|message| self.outbound_message_channel.try_send(message).unwrap());

            self.is_state_initialized = true;
        }

        // Drain message channel
        loop {
            match self.inbound_message_channel.try_receive() {

                Ok(Some(message)) => {
                    self.message_queue.push_back(message);
                },
                _ => break
            }
        }

        while let Some(message) = self.message_queue.pop_front() {
            match self.state.deliver(message) {
                DeliveryStatus::Delivered => {}
                DeliveryStatus::Unexpected(message) => {
                    println!("Unexpected message: {:?}", message);
                    return Err(StateMachineError {
                        message: format!("Unexpected message: {:?}", message),
                    })
                }
                DeliveryStatus::Error(error) => {
                    print!("Error: {}", error);
                    return Err(StateMachineError{ message: error })
                }
            }
        }

        // Attempt to advance the state machine
        let advanced = self.state.advance().map_err(|e| StateMachineError { message: e })?;

        return match advanced {
            Transition::Same => {
                Ok(StepResult::Running)
            }
            Transition::Next(state) => {
                self.state = state;
                self.is_state_initialized = false;
                return Ok(StepResult::Running)
            }
            Transition::Terminal => {
                return Ok(StepResult::Terminated)
            }
        }
    }
}