use std::fmt::Debug;

use downcast_rs::{Downcast, impl_downcast};

pub type BoxedState<Types> = Box<dyn State<Types>>;

pub trait StateType: 'static {
    type In: StateMachineMessage;
    type Out: StateMachineMessage;
}

#[derive(Debug, Clone)]
pub struct NoMessage(String);

impl StateMachineMessage for NoMessage {
    fn id(&self) -> &String {
        &self.0
    }

    fn unpack(self) -> NoMessage {
        NoMessage("".to_string())
    }
}

pub trait StateMachineMessage: Debug + Send + Clone {
    fn id(&self) -> &String;
    fn unpack(self) -> Self;
}

// Result from an attempt to deliver a message to a state.
pub enum DeliveryStatus<M, E: Debug> {
    Delivered,
    Unexpected(M),
    Error(E),
}

pub enum Transition<M: StateType> {
    Same,
    Next(BoxedState<M>),
    Terminal,
}

pub trait State<Types: StateType>: Downcast + Debug
{
    /// Fired once when the state is first entered
    fn initialize(&self) -> Vec<Types::Out> {
        vec![]
    }

    /// Called when a message is delivered to the state
    fn deliver(&mut self, message: Types::In) -> DeliveryStatus<Types::In, String> {
        DeliveryStatus::Unexpected(message)
    }

    /// Called until transition or terminal is returned
    fn advance(&self) -> Result<Transition<Types>, String>;
}

impl_downcast!(State<Types> where Types: StateType);
