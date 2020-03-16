mod algos;
mod common;
mod draw_utils;

use crate::algos::graham::{self, Graham};
use crate::algos::Algo;
use crate::common::Point;
use rand::{thread_rng, Rng};

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use minifb::{Key, Window, WindowOptions};
use raqote::{DrawTarget, SolidSource, Transform};

fn random_points() -> Vec<Point> {
    const N: usize = 30;
    const X: f32 = 30.0;
    const Y: f32 = 30.0;
    let mut rng = thread_rng();
    let mut res = Vec::with_capacity(N);
    for _ in 0..N {
        res.push(Point::new(rng.gen_range(2., X), rng.gen_range(2., Y)));
    }
    res
}

fn all_states<TAlgo, TState, TAction>(points: Vec<Point>) -> (Vec<TState>, Vec<TAction>)
where
    TAlgo: Algo<TState, TAction>,
    TState: Clone + std::fmt::Debug,
    TAction: Clone + std::fmt::Debug,
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

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn show<TAlgo, TState, TAction>(
    states: &Vec<TState>,
    _actions: &Vec<TAction>,
    max_x: f32,
    max_y: f32,
) where
    TAlgo: Algo<TState, TAction>,
    TState: Clone + std::fmt::Debug,
    TAction: Clone + std::fmt::Debug,
{
    let mut window = Window::new("Geometry", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    let mut index = 0;
    let mut was_key_down = false;
    let size = window.get_size();
    let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);
    dt.set_transform(&Transform::create_scale(
        (size.0 as f32) / max_x,
        (size.0 as f32) / max_y,
    ));

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        dt.clear(SolidSource::from_unpremultiplied_argb(0, 0, 0, 0xff));
        TAlgo::draw_state(&mut dt, &states[index]);

        if window.is_key_down(Key::Left) {
            if !was_key_down {
                index = index.saturating_sub(1);
            }
            was_key_down = true;
        } else if window.is_key_down(Key::Right) {
            if !was_key_down {
                index = std::cmp::min(index + 1, states.len() - 1);
            }
            was_key_down = true;
        } else {
            was_key_down = false;
        }
        window
            .update_with_buffer(dt.get_data(), size.0, size.1)
            .unwrap();
    }
}

fn main() {
    let points = random_points();

    let max_x = points
        .iter()
        .map(|a| a.x)
        .max_by(|a, b| a.partial_cmp(&b).unwrap())
        .unwrap();
    let max_y = points
        .iter()
        .map(|a| a.y)
        .max_by(|a, b| a.partial_cmp(&b).unwrap())
        .unwrap();

    let (states, actions) = all_states::<Graham, graham::State, graham::Action>(points);
    show::<Graham, graham::State, graham::Action>(&states, &actions, max_x + 2.0, max_y + 2.0);
}
