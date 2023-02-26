use std::error::Error;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamId(String);

#[async_trait]
pub trait EventRepository<E> {
    type RepositoryError: Error;

    async fn fetch_events(&self, stream_id: StreamId) -> Result<Vec<E>, Self::RepositoryError>;
    async fn save(&self, event: E) -> Result<(), Self::RepositoryError>;
    async fn save_all(&self, events: Vec<E>) -> Result<(), Self::RepositoryError>;
}

// todo: EventStore for db interactions. EventRepository -> EventStore -> Db
