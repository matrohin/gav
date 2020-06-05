pub use crate::algos::graham_common::Action;
use crate::algos::graham_common::{draw_graham_action, draw_progress, step};
use crate::algos::Algo;
use crate::common::*;
use crate::draw_context::*;

pub struct Graham;

#[derive(Clone, Debug)]
pub struct State {
    points: Vec<Point>,
    hull: Vec<Point>,
}

impl Algo<State, Action> for Graham {
    fn first_state(mut points: Vec<Point>) -> State {
        let (leftmost_idx, p0) = points
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| cmp_by_xy(a, b))
            .map(|(a, b)| (a, *b))
            .unwrap();
        let last_idx = points.len() - 1;
        points.swap(leftmost_idx, last_idx);
        points[..last_idx].sort_unstable_by(|a, b| rotation(&p0, a, b).partial_cmp(&0.).unwrap());
        State {
            points,
            hull: Vec::new(),
        }
    }

    fn next_state(mut state: State) -> (State, Action) {
        if state.points.is_empty() {
            return (state, Action::NoAction);
        }
        let action = step(&mut state.points, &mut state.hull, |a, b, c| {
            rotation(a, b, c) > 0.
        });
        (state, action)
    }

    fn is_final(state: &State) -> bool {
        state.points.is_empty()
    }

    fn draw_state(dc: &mut DrawContext, state: &State) {
        draw_progress(dc, &state.points, &state.hull);
        if Self::is_final(state) {
            dc.draw_line(&state.hull[0], state.hull.last().unwrap(), BLUE_COLOR);
        }
    }

    fn draw_action(dc: &mut DrawContext, action: &Action) {
        draw_graham_action(dc, action);
    }
}
