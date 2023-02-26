use serde::Serialize;
use serde_json::{Map, Value};

// pub type EventMetadata = Map<&'static str, Value>;
pub type EventMetadata = Map<String, Value>;
pub type EventId = String;
// pub type EventPayload = Value;
pub type EntityId = String;
pub type AggregateId = String;
pub type CommandMetadata = Map<String, Value>;
pub type CommandId = String;
pub type CommandPayload = Value;

// #[derive(Derivative, Serialize)]
// #[derivative(Debug, Clone, Eq, PartialEq)]
#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct EventPayload(pub Value);

// impl PartialOrd for EventPayload {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         None
//     }
// }
//
// impl Ord for EventPayload {
//     fn cmp(&self, _other: &Self) -> Ordering {
//         Ordering::Equal
//     }
// }
