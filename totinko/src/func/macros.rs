#[macro_export]
macro_rules! decider {
    ($decider_name:ident, {
        state: $state:ident,
        command: $command:ident,
        event: $event:ident
    }) => {
        #[derive(Debug)]
        pub struct $decider_name {
            decide_fn: fn($command, $state) -> Vec<$event>,
            evolve_fn: fn($state, $event) -> $state,
            initial_state: $state,
        }

        impl DeciderBase<$command, $state, $state, $event, $event> for $decider_name {
            fn decide(&self, command: $command, state: $state) -> Vec<$event> {
                (self.decide_fn)(command, state)
            }

            fn evolve(&self, state: $state, event: $event) -> $state {
                (self.evolve_fn)(state, event)
            }

            fn initial_state(&self) -> &$state {
                &self.initial_state
            }
        }

        impl Decider<$command, $state, $event> for $decider_name {
            fn new(
                decide: fn($command, $state) -> Vec<$event>,
                evolve: fn($state, $event) -> $state,
                initial_state: $state,
            ) -> Self {
                $decider_name {
                    decide_fn: decide,
                    evolve_fn: evolve,
                    initial_state,
                }
            }
        }
    };
}

#[cfg(test)]
mod macro_tests {
    use crate::decider;
    use crate::func::decider::{Decider, DeciderBase};

    #[derive(Default, Debug, Eq, PartialEq)]
    struct State;

    #[derive(Debug)]
    struct Command;

    #[derive(Debug)]
    struct Event;

    decider!(TestDecider, {
        state: State,
        command: Command,
        event: Event
    });

    #[test]
    fn test_new_decider() {
        let decide = |_command: Command, _state: State| vec![];
        let evolve = |_state: State, _event: Event| State {};
        let some_decider = TestDecider::new(decide, evolve, State::default());

        let events = some_decider.decide(Command {}, State {});

        assert_eq!(events.len(), 0);
    }
}
