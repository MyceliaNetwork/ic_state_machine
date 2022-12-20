#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::char::ToUppercase;
    use std::rc::Rc;
    use crate::state::{DeliveryStatus, NoMessage, State, StateMachineMessage, StateType, Transition};
    use crate::state::DeliveryStatus::Delivered;
    use crate::state::Transition::{Same, Terminal};
    use crate::state_machine_orchestrator::{SimpleMachineOrchestrator, StateMachineOrchestrator};

    #[derive(Debug, PartialEq)]
    pub struct Red {
        count: u64,
    }

    #[derive(Debug, PartialEq)]
    pub struct CommandStageOne {
    }

    #[derive(Debug, PartialEq)]
    pub struct CommandStageTwo {
    }

    #[derive(Debug, PartialEq)]
    pub struct CommandStageThree {
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Message {
        machine_id: String,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Commands {
        StartFoo {id : String},
        StartBar {id : String},
        StartBaz {id : String}
    }

    impl StateMachineMessage for Commands {
        fn id(&self) -> &String {
            match self {
                Commands::StartFoo { id, .. } => id,
                Commands::StartBar { id, .. } => id,
                Commands::StartBaz { id, .. } => id,
            }
        }

        fn unpack(self) -> Self {
            self
        }
    }

    impl StateMachineMessage for Message {
        fn id(&self) -> &String {
            &self.machine_id
        }

        fn unpack(self) -> Self {
            self
        }
    }

    pub struct Types {}
    pub struct TypesWithCommands {}

    impl StateType for Types {
        type In = Message;
        type Out = NoMessage;
    }

    impl StateType for TypesWithCommands {
        type In = Message;
        type Out = Commands;
    }

    impl State<Types> for Red {
        fn deliver(&mut self, message: Message) -> DeliveryStatus<Message, String> {
            self.count += 1;
            Delivered
        }

        fn advance(&self) -> Result<Transition<Types>, String> {
            Ok(Same)
        }
    }

    impl State<TypesWithCommands> for CommandStageOne {
        fn initialize(&self) -> Vec<Commands> {
            vec![Commands::StartFoo { id: "".to_string() }]
        }

        fn deliver(&mut self, message: Message) -> DeliveryStatus<Message, String> {
            Delivered
        }

        fn advance(&self) -> Result<Transition<TypesWithCommands>, String> {
            Ok(Transition::Next(Box::new(CommandStageTwo {})))

        }
    }

    impl State<TypesWithCommands> for CommandStageTwo {
        fn initialize(&self) -> Vec<Commands> {
            vec![Commands::StartBar { id: "".to_string() }]
        }

        fn deliver(&mut self, message: Message) -> DeliveryStatus<Message, String> {
            Delivered
        }

        fn advance(&self) -> Result<Transition<TypesWithCommands>, String> {
            Ok(Transition::Next(Box::new(CommandStageThree {})))
        }
    }

    impl State<TypesWithCommands> for CommandStageThree {
        fn initialize(&self) -> Vec<Commands> {
            vec![Commands::StartBaz { id: "".to_string() }]
        }

        fn deliver(&mut self, message: Message) -> DeliveryStatus<Message, String> {
            Delivered
        }

        fn advance(&self) -> Result<Transition<TypesWithCommands>, String> {
            Ok(Terminal)
        }
    }

    #[test]
    pub fn it_routes_messages() {
        let mut orchestrator = SimpleMachineOrchestrator::new(Box::new(|_| {}));

        let (id_one, _) = orchestrator.create_machine(
            Box::new(Red { count: 0 })
        );
        let (id_static, _) = orchestrator.create_machine(
            Box::new(Red { count: 0 })
        );

        orchestrator.handle_message(
            Message { machine_id: id_one.clone() }
        );
        orchestrator.handle_message(
            Message { machine_id: id_one.clone() }
        );

        let machine_one = orchestrator.get_state_machine(&id_one).unwrap();
        let machine_static = orchestrator.get_state_machine(&id_static).unwrap();

        assert_eq!(machine_one.downcast_state::<Red>().unwrap().count, 2);
        assert_eq!(machine_static.downcast_state::<Red>().unwrap().count, 0);
    }

    #[test]
    pub fn it_routes_passes_commands() {

        let mut commands = Rc::new(RefCell::new(vec![]));

        let mut handler_commands = commands.clone();
        let handler = move |v : Commands| {
            handler_commands.borrow_mut().push(v);
        };

        let handler = Box::new(handler);

        let mut orchestrator = SimpleMachineOrchestrator::new(handler);

        let (id_one, _) = orchestrator.create_machine(
            Box::new(CommandStageOne {})
        );

        orchestrator.step_machine(&id_one);

        assert_eq!(commands.borrow_mut().len(), 1);
        assert_eq!(commands.borrow_mut().pop(), Some(Commands::StartFoo { id: "".to_string() }));

        orchestrator.step_machine(&id_one);

        assert_eq!(commands.borrow_mut().len(), 1);
        assert_eq!(commands.borrow_mut().pop(), Some(Commands::StartBar { id: "".to_string() }));


        orchestrator.step_machine(&id_one);

        assert_eq!(commands.borrow_mut().len(), 1);
        assert_eq!(commands.borrow_mut().pop(), Some(Commands::StartBaz { id: "".to_string() }));

    }
}