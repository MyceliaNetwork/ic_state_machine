use crate::state::{DeliveryStatus, NoMessage, State, StateMachineMessage, StateType, Transition};
use crate::state::Transition::{Same, Terminal};

// A Simple state machine that alternates between Red and Blue states.

// State Red
#[derive(Debug, PartialEq)]
pub struct RedMessageState {
    pub count: u64,
}

// State Blue
#[derive(Debug, PartialEq)]
pub struct BlueMessageState {
    pub count: u64,
}

#[derive(Clone, Debug)]
pub enum SimpleMessage {
    IncrementRed { machine_id: String },
    IncrementBlue { machine_id: String },
}

impl StateMachineMessage for SimpleMessage {
    fn id(&self) -> &String {
        match self {
            SimpleMessage::IncrementRed { machine_id } => { machine_id }
            SimpleMessage::IncrementBlue { machine_id } => { machine_id }
        }
    }

    fn unpack(self) -> Self {
        self
    }
}

pub struct MachineTypes {}

// Defines the incoming and outgoing messages for the state machine.
impl StateType for MachineTypes {
    type In = SimpleMessage;
    type Out = NoMessage;
}

impl RedMessageState {
    pub fn new() -> Self {
        RedMessageState { count: 0 }
    }
}

impl State<MachineTypes> for RedMessageState {
    fn deliver(&mut self, message: SimpleMessage) -> DeliveryStatus<SimpleMessage, String> {
        println!("{:?} {:?}", self, message);
        match message {
            SimpleMessage::IncrementRed { .. } => {
                self.count += 1;
                DeliveryStatus::Delivered
            }
            _ => DeliveryStatus::Unexpected(message)
        }
    }

    fn advance(&self) -> Result<Transition<MachineTypes>, String> {
        if self.count < 3 {
            return Ok(Same);
        }
        Ok(Transition::Next(Box::new(BlueMessageState { count: 0 })))
    }
}

impl State<MachineTypes> for BlueMessageState {
    fn deliver(&mut self, message: SimpleMessage) -> DeliveryStatus<SimpleMessage, String> {
        println!("{:?} {:?}", self, message);

        match message {
            SimpleMessage::IncrementBlue { machine_id } => {
                self.count += 1;
                print!("Blue count: {}", self.count);
                DeliveryStatus::Delivered
            }
            _ => DeliveryStatus::Unexpected(message)
        }
    }

    fn advance(&self) -> Result<Transition<MachineTypes>, String> {
        if self.count < 2 {
            return Ok(Same);
        }
        Ok(Terminal)
    }
}

#[cfg(test)]
mod test {
    use crate::tests::example_2_simple_inbound_messages::{BlueMessageState, RedMessageState, SimpleMessage};
    use crate::message_channel::create_channel;
    use crate::state_machine::StateMachine;
    use crate::state_machine::StepResult::Terminated;

    #[test]
    pub fn test() {
        let (sender, _) = create_channel();
        let (mut machine, sender) = StateMachine::new("simple".to_string(), sender, Box::new(RedMessageState::new()));

        // Init the state machine
        let _ = machine.step();
        println!("State: {:?}", machine.state());
        assert_eq!(machine.downcast_state::<RedMessageState>(), Some(&RedMessageState { count: 0 }));

        // Send a message before 0 after 1
        sender.send(SimpleMessage::IncrementRed { machine_id: "one".to_string() });
        let _ = machine.step();
        assert_eq!(machine.downcast_state::<RedMessageState>(), Some(&RedMessageState { count: 1 }));

        // Send a message before 1 after 2
        sender.send(SimpleMessage::IncrementRed { machine_id: "two".to_string() });
        let _ = machine.step();
        assert_eq!(machine.downcast_state::<RedMessageState>(), Some(&RedMessageState { count: 2 }));

        // Send a message before 2 after 3
        sender.send(SimpleMessage::IncrementRed { machine_id: "three".to_string() });
        let _ = machine.step();
        assert_eq!(machine.downcast_state::<BlueMessageState>(), Some(&BlueMessageState { count: 0 }));

        // Send another message and step the machine. before step : 2
        sender.send(SimpleMessage::IncrementBlue { machine_id: "one".to_string() });
        let _ = machine.step();
        assert_eq!(machine.downcast_state::<BlueMessageState>(), Some(&BlueMessageState { count: 1 }));

        sender.send(SimpleMessage::IncrementBlue { machine_id: "two".to_string() });
        let result = machine.step();
        assert_eq!(machine.downcast_state::<BlueMessageState>(), Some(&BlueMessageState { count: 2 }));
        assert_eq!(result, Ok(Terminated));
    }
}