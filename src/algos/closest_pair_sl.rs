use crate::algos::Algo;
use crate::common::*;
use crate::draw_context::*;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::ops::Bound;
use std::ops::Deref;

#[derive(Copy, Clone, Debug)]
pub struct OrderedPoint(Point);

impl From<Point> for OrderedPoint {
    fn from(x: Point) -> Self {
        Self(x)
    }
}
impl Deref for OrderedPoint {
    type Target = Point;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Ord for OrderedPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .y
            .partial_cmp(&other.0.y)
            .unwrap()
            .then_with(|| self.0.x.partial_cmp(&other.0.x).unwrap())
    }
}
impl PartialOrd for OrderedPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for OrderedPoint {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for OrderedPoint {}

#[derive(Clone, Debug)]
pub struct State {
    points: Vec<Point>,
    current_set: BTreeSet<OrderedPoint>,
    left_index: usize,
    right_index: usize,
    nearest: Pair,
}

#[derive(Copy, Clone, Debug)]
pub enum Action {
    NoAction,
    Scan((Point, f32)),
}

pub struct ClosestPairSweepLine;

impl Algo<State, Action> for ClosestPairSweepLine {
    fn first_state(mut points: Vec<Point>) -> State {
        points.sort_unstable_by(cmp_by_x);
        State {
            points,
            current_set: BTreeSet::new(),
            left_index: 0,
            right_index: 0,
            nearest: Pair::inf(),
        }
    }

    fn next_state(mut state: State) -> (State, Action) {
        if state.right_index >= state.points.len() {
            return (state, Action::NoAction);
        }
        let h2 = state.nearest.square_len();
        let p = state.points[state.right_index];
        while state.left_index < state.right_index && p.x - state.points[state.left_index].x > h2 {
            state
                .current_set
                .remove(&OrderedPoint::from(state.points[state.left_index]));
            state.left_index += 1;
        }

        let h = h2.sqrt();
        let lb = Point::new(p.x - h, p.y - h);
        let rt = Point::new(p.x, p.y + h);
        for a in state.current_set.range((
            Bound::Included(OrderedPoint::from(lb)),
            Bound::Included(OrderedPoint::from(rt)),
        )) {
            let cur = Pair::new(**a, p);
            if cur.square_len() < state.nearest.square_len() {
                state.nearest = cur;
            }
        }
        state.current_set.insert(OrderedPoint::from(p));
        state.right_index += 1;
        (state, Action::Scan((p, h)))
    }

    fn is_final(state: &State) -> bool {
        state.right_index >= state.points.len()
    }

    fn draw_state(dc: &mut DrawContext, state: &State) {
        for point in &state.points {
            dc.draw_point(point, WHITE_COLOR);
        }
        if Pair::inf() != state.nearest {
            dc.draw_line(&state.nearest.a, &state.nearest.b, BLUE_COLOR);
        }
    }

    fn draw_action(dc: &mut DrawContext, action: &Action) {
        match action {
            Action::NoAction => {}
            Action::Scan((p, h)) => {
                if *h != std::f32::INFINITY {
                    let lb = Point::new((p.x - h).max(0.), (p.y - h).max(0.));
                    let rt = Point::new(p.x, (p.y + h).min(MAX_Y));
                    dc.fill_rect(&lb, &rt, GREEN_COLOR);
                } else {
                    dc.fill_part(0., p.x, GREEN_COLOR);
                }
                dc.draw_vertical_line(p.x, YELLOW_COLOR);
                dc.draw_point(p, RED_COLOR);
            }
        }
    }
}
