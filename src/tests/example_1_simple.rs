use crate::state::{NoMessage, State, StateType, Transition};
// A Simple state machine that alternates between Red and Blue states.

// State Red
#[derive(Debug, PartialEq)]
pub struct Red {}

// State Blue
#[derive(Debug, PartialEq)]
pub struct Blue {}

pub struct MachineTypes {}

// Defines the incoming and outgoing messages for the state machine.
impl StateType for MachineTypes {
    type In = NoMessage;
    type Out = NoMessage;
}

impl Red {
    pub fn new() -> Self {
        Red {}
    }
}

impl State<MachineTypes> for Red {
    fn advance(&self) -> Result<Transition<MachineTypes>, String> {
        Ok(Transition::Next(Box::new(Blue {})))
    }
}

impl State<MachineTypes> for Blue {
    fn advance(&self) -> Result<Transition<MachineTypes>, String> {
        Ok(Transition::Next(Box::new(Red {})))
    }
}

#[cfg(test)]
mod test {
    use crate::tests::example_1_simple::{Blue, Red};
    use crate::message_channel::create_channel;
    use crate::state_machine::StateMachine;

    #[test]
    pub fn test() {
        let (sender, _) = create_channel();
        let (mut machine, _) = StateMachine::new("simple".to_string(), sender, Box::new(Red::new()));

        assert_eq!(machine.downcast_state::<Red>(), Some(&Red {}));
        machine.step();

        assert_eq!(machine.downcast_state::<Blue>(), Some(&Blue {}));
        machine.step();

        assert_eq!(machine.downcast_state::<Red>(), Some(&Red {}));
        machine.step();
    }
}