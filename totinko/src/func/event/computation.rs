use std::marker::PhantomData;

use crate::func::decider::Decider;

pub trait EventComputer<C, S, E, D>
where
    D: Decider<C, S, E>,
{
    fn decider(&self) -> &D;
    fn compute_events(&self, events: Vec<E>, command: C, initial_state: S) -> Vec<E>;
}

#[derive(Debug)]
pub struct EventComputation<C, S, E, D>
where
    D: Decider<C, S, E>,
{
    decider: D,
    command: PhantomData<C>,
    event: PhantomData<E>,
    state: PhantomData<S>,
}

impl<C, S, E, D: Decider<C, S, E>> EventComputation<C, S, E, D> {
    pub fn new(decider: D) -> EventComputation<C, S, E, D> {
        Self {
            decider,
            command: Default::default(),
            event: Default::default(),
            state: Default::default(),
        }
    }
}

impl<C, S, E, D: Decider<C, S, E>> EventComputer<C, S, E, D> for EventComputation<C, S, E, D> {
    fn decider(&self) -> &D {
        &self.decider
    }

    fn compute_events(&self, events: Vec<E>, command: C, initial_state: S) -> Vec<E> {
        let current_state = events
            .into_iter()
            .fold(initial_state, |s, e| self.decider.evolve(s, e));

        self.decider.decide(command, current_state)
    }
}
