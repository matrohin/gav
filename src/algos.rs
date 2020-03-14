pub mod graham;

use crate::common::Point;
use raqote::DrawTarget;

pub trait Algo<TState, TAction>
where
    TState: Clone + std::fmt::Debug,
    TAction: Clone + std::fmt::Debug,
{
    fn first_state(points: Vec<Point>) -> TState;
    fn next_state(state: TState) -> (TState, TAction);
    fn is_final(state: &TState) -> bool;

    fn draw_state(dt: &mut DrawTarget, state: &TState);
    fn draw_action(dt: &mut DrawTarget, action: &TAction);
}
