use crate::algos::Algo;
use crate::common::*;
use crate::draw_context::*;

#[derive(Copy, Clone, Debug)]
enum StackState {
    ToLeftDivide,
    ToRightDivide,
    ToConquer(f32),
}

#[derive(Clone, Debug)]
pub struct State {
    points: Vec<Point>,
    result: Vec<Pair>,
    stack: Vec<(IndexBorders, StackState)>,
}

#[derive(Copy, Clone, Debug)]
pub enum Action {
    NoAction,
    Divide((HorBorders, f32)),
    Conquer((Pair, Pair, Pair, HorBorders, HorBorders)),
    PrimitiveSolve((HorBorders, Pair)),
}

fn brute_force(borders: &IndexBorders, points: &Vec<Point>) -> Pair {
    let mut best = Pair::inf();
    for i in borders.l..borders.r {
        for j in borders.l..i {
            let cur = Pair::new(points[i], points[j]);
            if cur.square_len() < best.square_len() {
                best = cur;
            }
        }
    }
    best
}

pub struct ClosestPairDivideAndConquer;

impl Algo<State, Action> for ClosestPairDivideAndConquer {
    fn first_state(mut points: Vec<Point>) -> State {
        points.sort_unstable_by(cmp_by_x);
        let borders = IndexBorders {
            l: 0,
            r: points.len(),
        };
        State {
            points,
            result: Vec::new(),
            stack: vec![(borders, StackState::ToLeftDivide)],
        }
    }

    fn next_state(mut state: State) -> (State, Action) {
        if state.stack.is_empty() {
            return (state, Action::NoAction);
        }
        let (borders, cur) = state.stack.pop().unwrap();
        let action = match cur {
            StackState::ToLeftDivide => {
                if borders.r - borders.l <= 3 {
                    let best = brute_force(&borders, &state.points);
                    let hor_borders = HorBorders::from_indexes(&state.points, &borders);
                    (&mut state.points[borders.l..borders.r]).sort_unstable_by(cmp_by_y);
                    state.result.push(best);
                    Action::PrimitiveSolve((hor_borders, best))
                } else {
                    let left_borders = borders.left();
                    state.stack.push((borders, StackState::ToRightDivide));
                    state.stack.push((left_borders, StackState::ToLeftDivide));
                    Action::Divide((
                        HorBorders::from_indexes(&state.points, &left_borders),
                        state.points[borders.r - 1].x,
                    ))
                }
            }
            StackState::ToRightDivide => {
                let right_borders = borders.right();
                let midx = state.points[right_borders.l].x;
                state.stack.push((borders, StackState::ToConquer(midx)));
                state.stack.push((right_borders, StackState::ToLeftDivide));
                Action::Divide((
                    HorBorders::new(midx, state.points[right_borders.r - 1].x),
                    state.points[borders.l].x,
                ))
            }
            StackState::ToConquer(midx) => {
                let right_best = state.result.pop().unwrap();
                let left_best = state.result.pop().unwrap();
                let mut best = if left_best.square_len() < right_best.square_len() {
                    left_best
                } else {
                    right_best
                };
                // TODO: rewrite to merge sort
                (&mut state.points[borders.l..borders.r]).sort_unstable_by(cmp_by_y);
                let h = best.square_len().sqrt();
                let mut border_points: Vec<Point> = Vec::new();
                let mut left_x = MAX_X;
                let mut right_x = 0.;
                for cur in &state.points[borders.l..borders.r] {
                    if (cur.x - midx).abs() < h {
                        for p in border_points.iter().rev() {
                            if cur.y - p.y > h {
                                break;
                            }
                            let t = Pair::new(*cur, *p);
                            if t.square_len() < best.square_len() {
                                best = t;
                            }
                        }
                        border_points.push(*cur);
                    }
                    left_x = cur.x.min(left_x);
                    right_x = cur.x.max(right_x);
                }

                state.result.push(best);
                Action::Conquer((
                    best,
                    left_best,
                    right_best,
                    HorBorders::new(left_x, right_x),
                    HorBorders::new(midx - h, midx + h),
                ))
            }
        };
        (state, action)
    }

    fn is_final(state: &State) -> bool {
        state.stack.is_empty()
    }

    fn draw_state(dc: &mut DrawContext, state: &State) {
        for point in &state.points {
            dc.draw_point(point, WHITE_COLOR);
        }
        for line in &state.result {
            dc.draw_line(&line.a, &line.b, GREEN_COLOR);
        }
    }

    fn draw_action(dc: &mut DrawContext, action: &Action) {
        match action {
            Action::NoAction => {}
            Action::Divide((borders, x)) => {
                dc.draw_vertical_line(*x, YELLOW_COLOR);
                dc.draw_borders(borders);
            }
            Action::Conquer((best, best_left, best_right, borders, points_borders)) => {
                dc.draw_borders(&HorBorders::new(
                    points_borders.l.max(borders.l),
                    points_borders.r.min(borders.r),
                ));
                dc.draw_vertical_line(borders.l, YELLOW_COLOR);
                dc.draw_vertical_line(borders.r, YELLOW_COLOR);

                dc.draw_line(&best_left.a, &best_left.b, RED_COLOR);
                dc.draw_line(&best_right.a, &best_right.b, RED_COLOR);
                dc.draw_line(&best.a, &best.b, BLUE_COLOR);
            }
            Action::PrimitiveSolve((borders, best)) => {
                dc.draw_borders(borders);
                dc.draw_line(&best.a, &best.b, BLUE_COLOR);
            }
        }
    }
}
