use crate::algos::Algo;
use crate::common::*;
use crate::draw_utils::*;
use raqote::DrawTarget;

#[derive(Copy, Clone, Debug)]
pub struct IndexBorders {
    l: usize,
    r: usize,
}

impl IndexBorders {
    fn left(&self) -> Self {
        IndexBorders {
            l: self.l,
            r: (self.l + self.r) / 2,
        }
    }
    fn right(&self) -> Self {
        IndexBorders {
            l: (self.l + self.r) / 2,
            r: self.r,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct HorBorders {
    l: f32,
    r: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Pair {
    a: Point,
    b: Point,
}

impl Pair {
    fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }
    fn inf() -> Self {
        Self {
            a: Point::new(0., 0.),
            b: Point::new(std::f32::INFINITY, std::f32::INFINITY),
        }
    }
    fn square_len(&self) -> f32 {
        (self.a - self.b).square_length()
    }
}

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

pub struct TwoNearest;

impl Algo<State, Action> for TwoNearest {
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
                    let mut best = Pair::inf();
                    for i in borders.l..borders.r {
                        for j in borders.l..i {
                            let cur = Pair::new(state.points[i], state.points[j]);
                            if cur.square_len() < best.square_len() {
                                best = cur;
                            }
                        }
                    }

                    let hor_borders = HorBorders {
                        l: state.points[borders.l].x,
                        r: state.points[borders.r - 1].x,
                    };

                    (&mut state.points[borders.l..borders.r]).sort_unstable_by(cmp_by_y);
                    state.result.push(best);
                    Action::PrimitiveSolve((hor_borders, best))
                } else {
                    let left_borders = borders.left();
                    state.stack.push((borders, StackState::ToRightDivide));
                    state.stack.push((left_borders, StackState::ToLeftDivide));
                    Action::Divide((
                        HorBorders {
                            l: state.points[left_borders.l].x,
                            r: state.points[left_borders.r - 1].x,
                        },
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
                    HorBorders {
                        l: midx,
                        r: state.points[right_borders.r - 1].x,
                    },
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
                    HorBorders {
                        l: left_x,
                        r: right_x,
                    },
                    HorBorders {
                        l: midx - h,
                        r: midx + h,
                    },
                ))
            }
        };
        (state, action)
    }

    fn is_final(state: &State) -> bool {
        state.stack.is_empty()
    }

    fn draw_state(dt: &mut DrawTarget, state: &State) {
        for point in &state.points {
            draw_point(dt, point, WHITE_COLOR);
        }
        for line in &state.result {
            draw_line(dt, &line.a, &line.b, GREEN_COLOR);
        }
    }

    fn draw_action(dt: &mut DrawTarget, action: &Action) {
        match action {
            Action::NoAction => {}
            Action::Divide((borders, x)) => {
                draw_vertical_line(dt, *x);
                draw_borders(dt, borders);
            }
            Action::Conquer((best, best_left, best_right, borders, points_borders)) => {
                draw_borders(
                    dt,
                    &HorBorders {
                        l: points_borders.l.max(borders.l),
                        r: points_borders.r.min(borders.r),
                    },
                );
                draw_vertical_line(dt, borders.l);
                draw_vertical_line(dt, borders.r);

                draw_line(dt, &best_left.a, &best_left.b, RED_COLOR);
                draw_line(dt, &best_right.a, &best_right.b, RED_COLOR);
                draw_line(dt, &best.a, &best.b, YELLOW_COLOR);
            }
            Action::PrimitiveSolve((borders, best)) => {
                draw_borders(dt, borders);
                draw_line(dt, &best.a, &best.b, YELLOW_COLOR);
            }
        }
    }
}

fn draw_vertical_line(dt: &mut DrawTarget, x: f32) {
    draw_line(dt, &Point::new(x, 0.), &Point::new(x, MAX_Y), BLUE_COLOR);
}

fn draw_borders(dt: &mut DrawTarget, borders: &HorBorders) {
    fill_part(dt, borders.l - 0.1, borders.r + 0.1, GREEN_COLOR);
}
