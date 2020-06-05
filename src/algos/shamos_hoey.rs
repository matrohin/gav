use crate::algos::Algo;
use crate::common::*;
use crate::draw_context::*;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::ops::Bound;

#[derive(Copy, Clone, Debug)]
pub struct Segment {
    a: Point,
    b: Point,
}

impl Segment {
    fn new(mut a: Point, mut b: Point) -> Segment {
        if a.x > b.x {
            std::mem::swap(&mut a, &mut b);
        }
        Segment { a, b }
    }
    fn get_y(&self, x: f32) -> f32 {
        // TODO: use eps?
        if self.a.x == self.b.x {
            self.a.y
        } else {
            self.a.y + (self.b.y - self.a.y) * (x - self.a.x) / (self.b.x - self.a.x)
        }
    }
}

impl Ord for Segment {
    fn cmp(&self, other: &Self) -> Ordering {
        let x = self.a.x.max(other.a.x);
        self.get_y(x)
            .partial_cmp(&other.get_y(x))
            .unwrap()
            .then_with(|| self.a.x.partial_cmp(&other.a.x).unwrap())
            .then_with(|| self.a.y.partial_cmp(&other.a.y).unwrap())
            .then_with(|| self.b.x.partial_cmp(&other.b.x).unwrap())
            .then_with(|| self.b.y.partial_cmp(&other.b.y).unwrap())
    }
}
impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for Segment {}

#[derive(Copy, Clone, Debug)]
pub struct Event {
    x: f32,
    id: usize,
    is_start: bool,
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        // TODO: use eps?
        other
            .x
            .partial_cmp(&self.x)
            .unwrap()
            .then_with(|| self.is_start.cmp(&other.is_start))
    }
}
impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for Event {}

#[derive(Clone, Debug)]
pub struct State {
    segments: Vec<Segment>,
    cur_segments: BTreeSet<Segment>,
    events: Vec<Event>,
    result: Option<(Segment, Segment)>,
}

#[derive(Clone, Debug)]
pub enum Action {
    NoAction,
    Scan((Segment, Option<(Segment, Segment)>, f32)),
}

fn intersect_1d(mut a1: f32, mut a2: f32, mut b1: f32, mut b2: f32) -> bool {
    if a1 > a2 {
        std::mem::swap(&mut a1, &mut a2)
    }
    if b1 > b2 {
        std::mem::swap(&mut b1, &mut b2)
    }
    a1.max(b1) <= a2.min(b2)
}

fn intersect_seg(first: &Segment, second: &Segment) -> bool {
    intersect_1d(first.a.x, first.b.x, second.a.x, second.b.x)
        && intersect_1d(first.a.y, first.b.y, second.a.y, second.b.y)
        && rotation(&first.a, &first.b, &second.a) * rotation(&first.a, &first.b, &second.b) <= 0.
        && rotation(&second.a, &second.b, &first.a) * rotation(&second.a, &second.b, &first.b) <= 0.
}

fn neighbors<'a>(
    segments: &'a BTreeSet<Segment>,
    v: &Segment,
) -> (Option<&'a Segment>, Option<&'a Segment>) {
    let mut before = segments.range((Bound::Unbounded, Bound::Excluded(v)));
    let mut after = segments.range((Bound::Excluded(v), Bound::Unbounded));

    (before.next_back(), after.next())
}

pub struct ShamosHoey;

impl Algo<State, Action> for ShamosHoey {
    fn first_state(mut points: Vec<Point>) -> State {
        points.sort_unstable_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        for i in (0..points.len()).step_by(2) {
            let swap_index = (i + 3).min(points.len() - 1);
            points.swap(i, swap_index);
        }

        let segments: Vec<_> = points
            .iter()
            .step_by(2)
            .zip(points.iter().skip(1).step_by(2))
            .map(|(a, b)| Segment::new(*a, *b))
            .collect();

        let mut events: Vec<_> = segments
            .iter()
            .enumerate()
            .map(|(id, seg)| Event {
                x: seg.a.x,
                id,
                is_start: true,
            })
            .chain(segments.iter().enumerate().map(|(id, seg)| Event {
                x: seg.b.x,
                id,
                is_start: false,
            }))
            .collect();
        events.sort_unstable();

        State {
            segments,
            cur_segments: BTreeSet::new(),
            events,
            result: None,
        }
    }
    fn next_state(mut state: State) -> (State, Action) {
        if state.events.is_empty() {
            return (state, Action::NoAction);
        }

        let cur_event = state.events.pop().unwrap();
        let cur_seg = state.segments[cur_event.id];
        if cur_event.is_start {
            state.cur_segments.insert(cur_seg);
        } else {
            state.cur_segments.remove(&cur_seg);
        }

        let (prev, next) = neighbors(&state.cur_segments, &cur_seg);
        let found = if cur_event.is_start {
            if prev.is_some() && intersect_seg(prev.unwrap(), &cur_seg) {
                Some((cur_seg, *prev.unwrap()))
            } else if next.is_some() && intersect_seg(next.unwrap(), &cur_seg) {
                Some((cur_seg, *next.unwrap()))
            } else {
                None
            }
        } else {
            if prev.is_some() && next.is_some() && intersect_seg(prev.unwrap(), next.unwrap()) {
                Some((*prev.unwrap(), *next.unwrap()))
            } else {
                None
            }
        };
        if let Some(found) = found {
            state.events.clear();
            state.result = Some(found);
            state.cur_segments.clear();
        }
        (state, Action::Scan((cur_seg, found, cur_event.x)))
    }

    fn is_final(state: &State) -> bool {
        state.events.is_empty()
    }

    fn draw_state(dc: &mut DrawContext, state: &State) {
        for seg in &state.segments {
            dc.draw_line(&seg.a, &seg.b, WHITE_COLOR);
        }
        for seg in &state.cur_segments {
            dc.draw_line(&seg.a, &seg.b, BLUE_COLOR);
        }
        if let Some((seg1, seg2)) = &state.result {
            dc.draw_line(&seg1.a, &seg1.b, GREEN_COLOR);
            dc.draw_line(&seg2.a, &seg2.b, GREEN_COLOR);
        }
    }

    fn draw_action(dc: &mut DrawContext, action: &Action) {
        match action {
            Action::NoAction => {}
            Action::Scan((cur_seg, found, x)) => {
                dc.draw_vertical_line(*x, BLUE_COLOR);
                dc.draw_line(&cur_seg.a, &cur_seg.b, YELLOW_COLOR);
                for (seg1, seg2) in found {
                    dc.draw_line(&seg1.a, &seg1.b, RED_COLOR);
                    dc.draw_line(&seg2.a, &seg2.b, RED_COLOR);
                }
            }
        }
    }
}
