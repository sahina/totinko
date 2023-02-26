use async_trait::async_trait;
use std::error::Error;

use crate::func::decider::Decider;
use crate::func::event::computation::EventComputation;
use crate::func::event::repository::EventRepository;

#[async_trait]
pub trait Aggregate<C, S, E>: Decider<C, S, E> + EventRepository<E> {
    type AggregateError: Error;

    /// Handles the command of type `C`, and returns new persisted events.
    async fn handle(command: C) -> Result<Vec<E>, Self::AggregateError>;
}

#[derive(Debug)]
pub struct EventSourcingAggregate<C, S, E, D>
where
    D: Decider<C, S, E>,
{
    computation: EventComputation<C, S, E, D>,
}
