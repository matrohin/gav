use crate::algos::Algo;
use crate::common::*;
use crate::draw_utils::*;
use raqote::DrawTarget;

#[derive(Copy, Clone, Debug)]
enum StackState {
    ToLeftDivide,
    ToRightDivide,
    ToConquer,
}

#[derive(Clone, Debug)]
pub struct State {
    points: Vec<Point>,
    result: Vec<Vec<Point>>,
    stack: Vec<(IndexBorders, StackState)>,
}

#[derive(Clone, Debug)]
pub enum Action {
    NoAction,
    Divide((HorBorders, f32)),
    Conquer((Vec<Point>, Vec<Point>, Pair, Pair)),
    PrimitiveSolve(Vec<Point>),
}

fn insert_unique(vec: &mut Vec<Point>, p: &Point) {
    if let Err(idx) = vec.binary_search_by(|a| cmp_by_xy(a, p)) {
        vec.insert(idx, *p);
    }
}

fn cmp_by_rotation(left: &Point, right: &Point, mid: &Point) -> std::cmp::Ordering {
    let left_up = left.y > mid.y;
    let right_up = right.y > mid.y;
    right_up.cmp(&left_up).then_with(|| {
        if left.y == right.y && left.y == mid.y {
            (right.x > mid.x).cmp(&(left.x > mid.x))
        } else {
            let rot = -rotation(left, right, mid);
            rot.partial_cmp(&0.).unwrap()
        }
    })
}

fn brute_force(borders: &IndexBorders, points: &Vec<Point>) -> Vec<Point> {
    let mut res = Vec::new();
    let len = borders.r - borders.l;
    for i in borders.l..borders.r {
        for j in (i + 1)..borders.r {
            let sum = |x, y| x + y as usize;
            let rot = |k| rotation(&points[i], &points[j], &points[k]);
            let pos = (borders.l..borders.r)
                .map(rot)
                .map(|r| r >= 0.)
                .fold(0, sum);
            let neg = (borders.l..borders.r)
                .map(rot)
                .map(|r| r <= 0.)
                .fold(0, sum);
            if pos == len || neg == len {
                insert_unique(&mut res, &points[i]);
                insert_unique(&mut res, &points[j]);
            }
        }
    }

    let mut mp = res
        .iter()
        .fold(Point::new(0., 0.), |a, b| Point::new(a.x + b.x, a.y + b.y));
    mp.x /= res.len() as f32;
    mp.y /= res.len() as f32;
    res.sort_unstable_by(|a, b| cmp_by_rotation(a, b, &mp));
    res
}

fn find_tangent<F>(
    left: &Vec<Point>,
    right: &Vec<Point>,
    mut li: usize,
    mut ri: usize,
    is_convex: F,
) -> (usize, usize)
where
    F: Fn(&Point, &Point, &Point) -> bool,
{
    let mut done = false;
    while !done {
        done = true;
        while !is_convex(&left[(li + 1) % left.len()], &left[li], &right[ri]) {
            li = (li + 1) % left.len();
        }
        while !is_convex(
            &left[li],
            &right[ri],
            &right[(ri + right.len() - 1) % right.len()],
        ) {
            ri = (ri + right.len() - 1) % right.len();
            done = false;
        }
    }
    (li, ri)
}

fn overflowing_range(i: usize, j: usize, l: usize) -> impl Iterator<Item = usize> {
    if i > j {
        (i..l).chain(0..=j)
    } else {
        (0..0).chain(i..=j)
    }
}

pub struct ConvexHullDivideAndConquer;

impl Algo<State, Action> for ConvexHullDivideAndConquer {
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
                if borders.r - borders.l <= 5 {
                    let r = brute_force(&borders, &state.points);
                    state.result.push(r.clone());
                    Action::PrimitiveSolve(r)
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
                state.stack.push((borders, StackState::ToConquer));
                state.stack.push((right_borders, StackState::ToLeftDivide));
                Action::Divide((
                    HorBorders {
                        l: state.points[right_borders.l].x,
                        r: state.points[right_borders.r - 1].x,
                    },
                    state.points[borders.l].x,
                ))
            }
            StackState::ToConquer => {
                let right = state.result.pop().unwrap();
                let left = state.result.pop().unwrap();
                let (leftmost_in_right, _) = right
                    .iter()
                    .enumerate()
                    .min_by(|a, b| cmp_by_x(a.1, b.1))
                    .unwrap();
                let (rightmost_in_left, _) = left
                    .iter()
                    .enumerate()
                    .max_by(|a, b| cmp_by_x(a.1, b.1))
                    .unwrap();

                let (left_upper, right_upper) = find_tangent(
                    &left,
                    &right,
                    rightmost_in_left,
                    leftmost_in_right,
                    |a, b, c| rotation(a, b, c) < 0.,
                );
                let (right_lower, left_lower) = find_tangent(
                    &right,
                    &left,
                    leftmost_in_right,
                    rightmost_in_left,
                    |a, b, c| rotation(a, b, c) < 0.,
                );

                let r = overflowing_range(left_upper, left_lower, left.len())
                    .map(|i| left[i])
                    .chain(
                        overflowing_range(right_lower, right_upper, right.len()).map(|i| right[i]),
                    )
                    .collect();

                state.result.push(r);
                let first_t = Pair::new(left[left_upper], right[right_upper]);
                let second_t = Pair::new(left[left_lower], right[right_lower]);
                Action::Conquer((left, right, first_t, second_t))
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
        for x in &state.result {
            draw_hull(dt, x);
        }
    }

    fn draw_action(dt: &mut DrawTarget, action: &Action) {
        match action {
            Action::NoAction => {}
            Action::Divide((borders, x)) => {
                draw_vertical_line(dt, *x, BLUE_COLOR);
                draw_borders(dt, borders);
            }
            Action::Conquer((left, right, upper, lower)) => {
                draw_hull(dt, &left);
                draw_hull(dt, &right);
                draw_line(dt, &upper.a, &upper.b, GREEN_COLOR);
                draw_line(dt, &lower.a, &lower.b, GREEN_COLOR);
            }
            Action::PrimitiveSolve(hull) => {
                draw_hull(dt, &hull);
            }
        }
    }
}

fn draw_hull(dt: &mut DrawTarget, x: &Vec<Point>) {
    draw_path(dt, x, BLUE_COLOR);
    draw_line(dt, x.first().unwrap(), x.last().unwrap(), BLUE_COLOR);
}
