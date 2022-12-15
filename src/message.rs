struct StateMachineMessage<T> {
    state_machine_id: String,
    message: T,
}

impl<T> StateMachineMessage<T> {
    pub fn new(state_machine_id: String, message: T) -> Self {
        StateMachineMessage {
            state_machine_id,
            message,
        }
    }

    pub fn state_machine_id(&self) -> &str {
        &self.state_machine_id
    }

    pub fn unwrap(self) -> T {
        self.message
    }
}

#[cfg(test)]
mod test {
    use crate::message::StateMachineMessage;

    #[test]
    pub fn test() {
        let message = StateMachineMessage::new("test".to_string(), 1);
        assert_eq!(message.state_machine_id(), "test");
        assert_eq!(message.unwrap(), 1);
    }
}