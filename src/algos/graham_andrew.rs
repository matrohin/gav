pub use crate::algos::graham_common::Action;
use crate::algos::graham_common::{draw_graham_action, draw_progress, step};
use crate::algos::Algo;
use crate::common::*;
use crate::draw_context::*;

pub struct GrahamAndrew;

#[derive(Clone, Debug)]
pub struct State {
    left_upper: Vec<Point>,
    left_lower: Vec<Point>,
    upper: Vec<Point>,
    lower: Vec<Point>,
}

impl Algo<State, Action> for GrahamAndrew {
    fn first_state(mut points: Vec<Point>) -> State {
        points.sort_unstable_by(|a, b| cmp_by_x(b, a));
        State {
            left_upper: points.clone(),
            left_lower: points,
            upper: Vec::new(),
            lower: Vec::new(),
        }
    }

    fn next_state(mut state: State) -> (State, Action) {
        let action = if !state.left_upper.is_empty() {
            step(&mut state.left_upper, &mut state.upper, |a, b, c| {
                rotation(a, b, c) < 0.
            })
        } else if !state.left_lower.is_empty() {
            step(&mut state.left_lower, &mut state.lower, |a, b, c| {
                rotation(a, b, c) > 0.
            })
        } else {
            Action::NoAction
        };
        (state, action)
    }

    fn is_final(state: &State) -> bool {
        state.left_upper.is_empty() && state.left_lower.is_empty()
    }

    fn draw_state(dc: &mut DrawContext, state: &State) {
        if !state.left_upper.is_empty() || state.lower.is_empty() {
            draw_progress(dc, &state.left_upper, &state.upper);
        } else {
            draw_progress(dc, &state.left_upper, &state.upper);
            draw_progress(dc, &state.left_lower, &state.lower);
        };
    }

    fn draw_action(dc: &mut DrawContext, action: &Action) {
        draw_graham_action(dc, action);
    }
}
