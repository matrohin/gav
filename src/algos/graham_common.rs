use crate::common::*;
use crate::draw_utils::*;
use raqote::DrawTarget;

#[derive(Copy, Clone, Debug)]
pub enum Action {
    NoAction,
    AcceptPoint(Point),
    RejectPoint(Point),
    AcceptLine((Point, Point, Point)),
    RejectLine((Point, Point, Point)),
}

pub fn step<F>(left: &mut Vec<Point>, result: &mut Vec<Point>, is_convex: F) -> Action
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

pub fn draw_progress(dt: &mut DrawTarget, left: &Vec<Point>, done: &Vec<Point>) {
    for point in left {
        draw_point(dt, point, WHITE_COLOR);
    }
    draw_path(dt, done, BLUE_COLOR);
}

pub fn draw_graham_action(dt: &mut DrawTarget, action: &Action) {
    match action {
        Action::NoAction => {}
        Action::AcceptPoint(p) => draw_point(dt, p, GREEN_COLOR),
        Action::RejectPoint(p) => draw_point(dt, p, RED_COLOR),
        Action::AcceptLine((p1, p2, p3)) => draw_path(dt, &vec![*p1, *p2, *p3], GREEN_COLOR),
        Action::RejectLine((p1, p2, p3)) => draw_path(dt, &vec![*p1, *p2, *p3], RED_COLOR),
    }
}
