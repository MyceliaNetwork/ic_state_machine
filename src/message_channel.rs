use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct MessageSender<T> {
    buffer: Arc<Mutex<VecDeque<T>>>,
}

impl<T> MessageSender<T> {
    pub fn try_send(&self, message: T) -> Result<(), ()> {
        match self.buffer.try_lock() {
            Ok(mut buffer) => {
                buffer.push_back(message);
                Ok(())
            }
            Err(_) => Err(())
        }
    }
}

pub struct MessageReceiver<T> {
    buffer: Arc<Mutex<VecDeque<T>>>,
}

impl<T> MessageReceiver<T> {
    pub fn try_receive(&self) -> Result<Option<T>, ()> {
        match self.buffer.try_lock() {
            Ok(mut buffer) => {
                Ok(buffer.pop_front())
            }
            Err(_) => Err(())
        }
    }
}


pub fn create_channel<T>() -> (MessageSender<T>, MessageReceiver<T>) {
    let buffer = Arc::new(Mutex::new(VecDeque::new()));
    let sender = MessageSender { buffer: buffer.clone() };
    let receiver = MessageReceiver { buffer };
    (sender, receiver)
}

#[cfg(test)]
mod test {
    use crate::message_channel::create_channel;

    #[test]
    pub fn it_sends_and_receives_a_message() {
        let (tx, rx) = create_channel::<u64>();

        tx.try_send(1).unwrap();
        assert_eq!(rx.try_receive().unwrap(), Some(1));
    }

    #[test]
    pub fn it_handles_multiple_senders() {
        let (tx, rx) = create_channel::<u64>();

        let tx_1 = tx.clone();

        tx.try_send(1).unwrap();
        assert_eq!(rx.try_receive().unwrap(), Some(1));

        tx_1.try_send(2).unwrap();
        tx.try_send(3).unwrap();
        assert_eq!(rx.try_receive().unwrap(), Some(2));
        assert_eq!(rx.try_receive().unwrap(), Some(3));
    }
}