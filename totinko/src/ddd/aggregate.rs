use crate::ddd::core::{EventMetadata, EventPayload};
use crate::ddd::event::AggregateEvent;

pub trait Eventer {
    fn add_event(&mut self, name: &str, payload: EventPayload, metadata: Option<EventMetadata>);
    fn events(&self) -> &Vec<AggregateEvent>;
    fn clear_events(&mut self);
}

pub trait AggregateNamer {
    fn aggregate_name(&self) -> String;
}
