// use std::marker::PhantomData;
//
// use crate::func::decider::Decider;
//
// #[derive(Default)]
// pub struct TextExecutor<C, S, E, D>
// where
//     D: Decider<C, S, E>,
// {
//     decider: D,
//     events: Vec<E>,
//     state: PhantomData<S>,
//     command: PhantomData<C>,
// }
//
// #[derive(Default)]
// pub struct TestFramework<C, S, E, D>
// where
//     D: Decider<C, S, E>, {}
//
// impl<C, S, E, D: Decider<C, S, E>> TestFramework<C, S, E, D> {
//     pub fn new() -> Self {
//         todo!()
//     }
//
//     pub fn given_no_previous_events(self) -> TextExecutor<C, S, E, D> {
//         todo!()
//     }
// }
