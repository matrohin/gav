pub mod closest_pair_dnc;
pub mod closest_pair_sl;
pub mod convex_hull_dnc;
pub mod graham;
pub mod graham_andrew;
pub mod graham_common;
pub mod shamos_hoey;

use crate::common::Point;
use crate::draw_context::DrawContext;

pub trait Algo {
    type State: Clone + std::fmt::Debug;
    type Action: Clone + std::fmt::Debug;

    fn first_state(points: Vec<Point>) -> Self::State;
    fn next_state(state: Self::State) -> (Self::State, Self::Action);
    fn is_final(state: &Self::State) -> bool;

    fn draw_state(dc: &mut DrawContext, state: &Self::State);
    fn draw_action(dc: &mut DrawContext, action: &Self::Action);
}
