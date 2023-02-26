use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use totinko::es::context::ESAggregateContext;

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

#[derive(Debug, Serialize, Deserialize)]
struct AddProduct {
    product_id: String,
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
    _context: ESAggregateContext,
    order_id: Option<String>,
}

impl OrderAR {
    pub fn new(id: &str) -> Self {
        Self {
            _context: ESAggregateContext::new(id, "OrderAR"),
            order_id: Some(id.to_string()),
        }
    }
}

#[tokio::main]
async fn main() {
    let ar = OrderAR::new("123");

    dbg!(ar);
}
