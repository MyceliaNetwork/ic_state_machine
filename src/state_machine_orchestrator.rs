use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::{channel, Receiver};

use crate::state::{State, StateMachineMessage, StateType};
use crate::state_machine::{StateMachine, StateMachineHandle, StateMachineId};

pub trait StateMachineOrchestrator<Types: StateType> {
    fn create_machine(&mut self, state: Box<dyn State<Types>>) -> (StateMachineId, StateMachineHandle<Types::In>);
    fn handle_message(&mut self, message: Types::In) -> ();
}

pub struct SimpleMachineOrchestrator<Types: StateType> {
    next_id: u64,
    machines: HashMap<String, (StateMachine<Types>, StateMachineHandle<Types::In>, Receiver<Types::Out>)>,
    commands : VecDeque<Types::Out>,
}

impl<Types: StateType> SimpleMachineOrchestrator<Types> {
    pub fn new() -> SimpleMachineOrchestrator<Types> {
        SimpleMachineOrchestrator {
            next_id: 0,
            machines: HashMap::new(),
            commands: Default::default()
        }
    }
}

impl<Types: StateType> StateMachineOrchestrator<Types> for SimpleMachineOrchestrator<Types> {
    fn create_machine(&mut self, state: Box<dyn State<Types>>) -> (StateMachineId, StateMachineHandle<Types::In>) {
        let machine_id = self.next_id.to_string();
        let (tx, rx) = channel::<Types::Out>();

        let (machine, inbound_channel) = StateMachine::new(
            machine_id.clone(),
            tx,
            state,
        );

        self.machines.insert(machine_id.clone(), (machine, inbound_channel.clone(), rx));
        self.next_id += 1;
        return (machine_id, inbound_channel.clone());
    }

    // Pass the message to the correct state machine
    // Invoke the state machine's step function
    fn handle_message(&mut self, message: Types::In) -> () {
        match self.machines.get_mut(message.id()) {
            None => {}
            Some((machine, handle, rx)) => {
                handle.send(message).unwrap();
                machine.step();
                while let Ok(command) = rx.try_recv() {
                    self.commands.push_back(command);
                }
            }
        }
    }
}

impl<Types: StateType> SimpleMachineOrchestrator<Types> {

}