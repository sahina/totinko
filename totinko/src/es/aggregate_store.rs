use async_trait::async_trait;

use crate::ddd::entity::{TEntityId, TEntityName};
use crate::es::context::ESAggregateContext;
use crate::es::event::EventApplier;

#[async_trait]
pub trait TAggregateStore {
    type Error: std::error::Error;

    async fn load(&self, context: &mut ESAggregateContext) -> Result<(), Self::Error>;
    async fn save(&self, context: &mut ESAggregateContext) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait TAggregateStore2<A>
where
    A: TEntityId + TEntityName + EventApplier,
{
    type Error: std::error::Error;

    async fn load(&self, context: &mut A) -> Result<(), Self::Error>;
    async fn save(&self, context: &mut A) -> Result<(), Self::Error>;
}

#[cfg(test)]
mod aggregate_store_tests {
    use crate::es::context::ESAggregateContext;

    // demo aggregate
    #[derive(Debug)]
    struct DemoAggregate {
        context: ESAggregateContext,
        full_name: String,
        phone: String,
    }

    impl DemoAggregate {}
}
