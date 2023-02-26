use async_trait::async_trait;
use thiserror::Error;

use crate::ddd::core::{AggregateId, EventMetadata};
use crate::ddd::event::{AggregateEvent, Event};
use crate::es::event::EventApplier;

/// Event store errors.
#[derive(Debug, Error)]
pub enum StoreError {}

#[async_trait]
pub trait EventStore<A: EventApplier> {
    /// Load aggregate at current state
    async fn load_aggregate(&self, aggregate_id: AggregateId) -> Result<A, StoreError>;

    /// Load all events for a particular `aggregate_id`
    async fn load_events(
        &self,
        aggregate_id: AggregateId,
    ) -> Result<Vec<AggregateEvent>, StoreError>;

    /// Commit new events
    async fn commit(
        &self,
        events: Vec<Event>,
        metadata: EventMetadata,
    ) -> Result<Vec<AggregateEvent>, StoreError>;
}

// #[async_trait]
// pub trait EventStore {
//     async fn load_aggregate(&self, aggregate_id: AggregateId) -> Result<AggregateES, StoreError>;
//     async fn load_events(
//         &self,
//         aggregate_id: AggregateId,
//     ) -> Result<Vec<AggregateEvent>, StoreError>;
//     async fn commit(
//         &self,
//         events: Vec<Event>,
//         metadata: EventMetadata,
//     ) -> Result<Vec<AggregateEvent>, StoreError>;
// }
