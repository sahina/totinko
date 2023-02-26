use thiserror::Error;

use crate::ddd::event::Event;

#[derive(Error, Debug)]
pub enum EventError {
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error("unknown event")]
    UnknownEvent,
}

pub trait EventHandler {
    fn handle(&self, event: Event) -> Vec<Event>;
}

pub trait EventApplier {
    fn apply(&mut self, event: Event);
}

pub trait EventCommitter {
    fn commit_events(&mut self);
}

pub trait VersionSetter {
    fn set_version(&mut self, version: u32);
}

pub trait Versioner {
    fn version(&self) -> u32;
    fn pending_version(&self) -> u32;
}
