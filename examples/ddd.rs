use std::fmt::{Display, Error, Formatter};

use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use totinko::ddd::aggregate::Aggregate;
use totinko::ddd::command::{new_command, Command};
use totinko::ddd::context::{new_aggregate_ctx, AggregateContext};
use totinko::ddd::event::{new_event, Event};

//
// Errors

#[derive(Debug)]
struct OrderError(String);

impl Display for OrderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for OrderError {}

//
// Command payloads

#[derive(Debug, Serialize, Deserialize)]
struct CreateOrderPayload {
    order_id: String,
}

//
// Event payloads

#[derive(Debug, Serialize, Deserialize)]
struct OrderCreatedPayload {
    order_id: String,
    timestamp: i64,
}

//
// Aggregate Root

#[derive(Debug)]
struct OrderAR {
    _context: AggregateContext,
    order_id: Option<String>,
    payment_id: Option<String>,
}

impl OrderAR {
    pub fn new(id: &str) -> Self {
        Self {
            _context: new_aggregate_ctx(id, "OrderAR"),
            order_id: Some(id.to_string()),
            payment_id: None,
        }
    }
}

#[async_trait]
impl Aggregate for OrderAR {
    type Error = OrderError;
    type Services = (); // to services used for this simple example

    async fn handle(
        &self,
        command: Command,
        _service: Self::Services,
    ) -> Result<Vec<Event>, Self::Error> {
        let name = command.entity.name.as_str();
        match name {
            "CreateOrder" => {
                // get typed payload from command
                let command_payload: CreateOrderPayload =
                    serde_json::from_value(command.payload).unwrap();

                // create event payload returned events as result
                let event_payload = OrderCreatedPayload {
                    order_id: command_payload.order_id,
                    timestamp: Utc::now().timestamp(),
                };
                // event(s) generated as result of processing command
                let events = vec![new_event("OrderCreated", json!(event_payload))];

                Ok(events)
            }
            _ => return Err(OrderError("Unknown command".to_string())),
        }
    }

    fn apply(&mut self, event: Event) {
        let name = event.entity.name.as_str();

        // update AR state by applying events. no business logic
        match name {
            "OrderCreated" => {
                let event_payload: OrderCreatedPayload =
                    serde_json::from_value(event.payload).unwrap();
                self.order_id = Some(event_payload.order_id);
            }
            _ => panic!("unknown event"),
        }
    }
}

#[tokio::main]
async fn main() {
    let payload = CreateOrderPayload {
        order_id: "123".to_string(),
    };
    let command = new_command("CreateOrder", json!(payload));

    let ar = OrderAR::new("1");
    let events = ar.handle(command, ()).await;

    println!("{:?}", ar);
    println!("{:?}", events);
}
