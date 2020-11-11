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

pub fn all_states<TAlgo>(points: Vec<Point>) -> (Vec<TAlgo::State>, Vec<TAlgo::Action>)
where
    TAlgo: Algo,
{
    let mut states = vec![TAlgo::first_state(points)];
    let mut actions = Vec::new();
    while !TAlgo::is_final(states.last().unwrap()) {
        let (next, action) = TAlgo::next_state(states.last().unwrap().clone());
        states.push(next);
        actions.push(action);
    }
    (states, actions)
}
