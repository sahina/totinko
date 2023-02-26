use serde::Serialize;

#[derive(Debug, Eq, PartialEq, Clone, Serialize)]
pub struct UserRegistrationData {
    pub email: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize)]
pub struct UserSignInData {
    pub email: String,
}

pub fn user_registration() -> UserRegistrationData {
    UserRegistrationData {
        email: "homer@simpsons.com".to_string(),
    }
}

pub fn user_sign_in() -> UserSignInData {
    UserSignInData {
        email: "homer@simpsons.com".to_string(),
    }
}

pub mod types {
    use totinko::decider;
    use totinko::func::decider::{Decider, DeciderBase};

    decider!(OrderDecider, {
        state: OrderState,
        command: OrderCommand,
        event: OrderEvent
    });

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct Product {
        pub product_id: String,
    }

    #[derive(Debug, Clone, Default, Eq, PartialEq)]
    pub struct OrderState {
        pub order_id: String,
        pub products: Vec<Product>,
        pub shipped: bool,
    }

    //
    // Events

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum OrderValidationEvent {
        OrderIdMissing,
        ProductIdMissing,
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum OrderEvent {
        Created(String),
        ProductAdded(Product),
        ProductRemoved(Product),
        Shipped,
        Invalid(OrderValidationEvent),
    }

    //
    // Commands

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum OrderCommand {
        Create(String),
        AddProduct(Product),
        RemoveProduct(Product),
        Ship,
    }

    //
    // decider implementations

    pub fn order_decide(command: OrderCommand, state: OrderState) -> Vec<OrderEvent> {
        match command {
            OrderCommand::Create(order) => {
                if order.is_empty() {
                    vec![OrderEvent::Invalid(OrderValidationEvent::OrderIdMissing)]
                } else {
                    vec![OrderEvent::Created(order)]
                }
            }
            OrderCommand::AddProduct(product) => {
                let product_id = product.product_id.trim();

                match product_id {
                    "" => {
                        vec![OrderEvent::Invalid(OrderValidationEvent::ProductIdMissing)]
                    }
                    _ => {
                        vec![OrderEvent::ProductAdded(product)]
                    }
                }
            }
            OrderCommand::RemoveProduct(product) => {
                if state.products.contains(&product) {
                    vec![OrderEvent::ProductRemoved(product)]
                } else {
                    vec![]
                }
            }
            OrderCommand::Ship => vec![OrderEvent::Shipped],
        }
    }

    pub fn order_evolve(mut state: OrderState, event: OrderEvent) -> OrderState {
        match event {
            OrderEvent::Created(order) => OrderState {
                order_id: order,
                products: vec![],
                shipped: false,
            },
            OrderEvent::ProductAdded(product) => {
                state.products.push(product);
                state
            }
            OrderEvent::ProductRemoved(product) => {
                let found = state
                    .products
                    .iter()
                    .position(|item| *item.product_id == product.product_id);

                if let Some(index) = found {
                    state.products.remove(index);
                    state
                } else {
                    state
                }
            }
            OrderEvent::Shipped => {
                state.shipped = true;
                state
            }
            OrderEvent::Invalid(_) => state,
        }
    }
}
