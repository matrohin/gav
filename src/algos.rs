pub mod graham;

use crate::common::Point;

pub trait Algo<TState, TAction>
where
    TState: Clone + std::fmt::Debug,
    TAction: Clone + std::fmt::Debug,
{
    fn first_state(points: Vec<Point>) -> TState;
    fn next_state(state: TState) -> (TState, TAction);
    fn is_final(state: &TState) -> bool;
}
