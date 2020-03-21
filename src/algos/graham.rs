use crate::algos::Algo;
use crate::common::*;
use crate::draw_utils::*;
use raqote::DrawTarget;

pub struct Graham;

#[derive(Copy, Clone, Debug)]
pub enum Action {
    NoAction,
    AcceptPoint(Point),
    RejectPoint(Point),
    AcceptLine((Point, Point, Point)),
    RejectLine((Point, Point, Point)),
}

#[derive(Clone, Debug)]
pub struct State {
    left_upper: Vec<Point>,
    left_lower: Vec<Point>,
    upper: Vec<Point>,
    lower: Vec<Point>,
}

fn step<F>(left: &mut Vec<Point>, result: &mut Vec<Point>, is_convex: F) -> Action
where
    F: Fn(&Point, &Point, &Point) -> bool,
{
    if !result.is_empty()
        && left.len() > 1
        && !is_convex(
            result.first().unwrap(),
            left.last().unwrap(),
            left.first().unwrap(),
        )
    {
        Action::RejectPoint(left.pop().unwrap())
    } else if result.len() < 2 {
        result.push(left.pop().unwrap());
        Action::AcceptPoint(result.last().unwrap().clone())
    } else if !is_convex(
        &result[result.len() - 2],
        &result[result.len() - 1],
        left.last().unwrap(),
    ) {
        let removed = result.pop().unwrap();
        Action::RejectLine((*result.last().unwrap(), removed, *left.last().unwrap()))
    } else {
        result.push(left.pop().unwrap());
        Action::AcceptLine((
            result[result.len() - 3],
            result[result.len() - 2],
            result[result.len() - 1],
        ))
    }
}

fn draw_half(dt: &mut DrawTarget, left: &Vec<Point>, done: &Vec<Point>) {
    for point in left {
        draw_point(dt, point, WHITE_COLOR);
    }
    draw_path(dt, done, BLUE_COLOR);
}

impl Algo<State, Action> for Graham {
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

    fn draw_state(dt: &mut DrawTarget, state: &State) {
        if !state.left_upper.is_empty() || state.lower.is_empty() {
            draw_half(dt, &state.left_upper, &state.upper);
        } else {
            draw_half(dt, &state.left_upper, &state.upper);
            draw_half(dt, &state.left_lower, &state.lower);
        };
    }

    fn draw_action(dt: &mut DrawTarget, action: &Action) {
        match action {
            Action::NoAction => {}
            Action::AcceptPoint(p) => draw_point(dt, p, GREEN_COLOR),
            Action::RejectPoint(p) => draw_point(dt, p, RED_COLOR),
            Action::AcceptLine((p1, p2, p3)) => draw_path(dt, &vec![*p1, *p2, *p3], GREEN_COLOR),
            Action::RejectLine((p1, p2, p3)) => draw_path(dt, &vec![*p1, *p2, *p3], RED_COLOR),
        }
    }
}
