pub trait DeciderBase<C, Si, So, Ei, Eo> {
    fn decide(&self, command: C, state: Si) -> Vec<Eo>;
    fn evolve(&self, state: Si, event: Ei) -> So;
    fn initial_state(&self) -> &So;
}

pub trait Decider<C, S, E>: DeciderBase<C, S, S, E, E> {
    fn new(
        decide: fn(command: C, state: S) -> Vec<E>,
        evolve: fn(state: S, event: E) -> S,
        initial_state: S,
    ) -> Self;
}
