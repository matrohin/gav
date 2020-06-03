pub mod closest_pair_dnc;
pub mod closest_pair_sl;
pub mod convex_hull_dnc;
pub mod graham;
pub mod graham_andrew;
pub mod graham_common;
pub mod shamos_hoey;

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
