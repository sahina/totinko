use crate::common::helpers::types::OrderEvent;
use async_trait::async_trait;
use std::fmt::Error;
use totinko::func::event::repository::{EventRepository, StreamId};

#[derive(Debug)]
pub struct OrderRepository {}

#[async_trait]
impl EventRepository<OrderEvent> for OrderRepository {
    type RepositoryError = Error;

    async fn fetch_events(
        &self,
        stream_id: StreamId,
    ) -> Result<Vec<OrderEvent>, Self::RepositoryError> {
        todo!()
    }

    async fn save(&self, event: OrderEvent) -> Result<(), Self::RepositoryError> {
        todo!()
    }

    async fn save_all(&self, events: Vec<OrderEvent>) -> Result<(), Self::RepositoryError> {
        todo!()
    }
}
