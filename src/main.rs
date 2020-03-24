mod algos;
mod common;
mod draw_utils;

use crate::algos::closest_pair_sl::{self, ClosestPairSweepLine};
use crate::algos::graham::{self, Graham};
use crate::algos::shamos_hoey::{self, ShamosHoey};
use crate::algos::two_nearest_dnc::{self, TwoNearest};
use crate::algos::Algo;
use crate::common::*;
use clap::{App, Arg};
use rand::{thread_rng, Rng};

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use raqote::{DrawTarget, SolidSource, Transform};

fn random_points() -> Vec<Point> {
    const N: usize = 30;
    let mut rng = thread_rng();
    let mut res = Vec::with_capacity(N);
    for _ in 0..N {
        res.push(Point::new(
            rng.gen_range(0., MAX_X),
            rng.gen_range(0., MAX_Y),
        ));
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

fn show<TAlgo, TState, TAction>(states: &Vec<TState>, actions: &Vec<TAction>)
where
    TAlgo: Algo<TState, TAction>,
    TState: Clone + std::fmt::Debug,
    TAction: Clone + std::fmt::Debug,
{
    let mut window = Window::new("Geometry", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    let mut index = 0;
    let size = window.get_size();
    let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);
    let transform = Transform::create_translation(1., -MAX_Y - 1.);
    let transform = transform.post_scale(
        (size.0 as f32) / (MAX_X + 2.0),
        -(size.0 as f32) / (MAX_Y + 2.0),
    );
    dt.set_transform(&transform);

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    window.set_key_repeat_rate(0.01);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        dt.clear(SolidSource::from_unpremultiplied_argb(0, 0, 0, 0xff));
        if index % 2 == 0 {
            TAlgo::draw_state(&mut dt, &states[index / 2]);
        } else {
            TAlgo::draw_state(&mut dt, &states[index / 2]);
            TAlgo::draw_action(&mut dt, &actions[index / 2]);
        }

        if window.is_key_pressed(Key::Right, KeyRepeat::Yes) {
            index = std::cmp::min(index + 1, actions.len() * 2);
        } else if window.is_key_pressed(Key::Left, KeyRepeat::Yes) {
            index = index.saturating_sub(1);
        } else if window.is_key_pressed(Key::Home, KeyRepeat::No) {
            index = 0;
        } else if window.is_key_pressed(Key::End, KeyRepeat::No) {
            index = actions.len() * 2;
        }
        window
            .update_with_buffer(dt.get_data(), size.0, size.1)
            .unwrap();
    }
}

fn run<TAlgo, TState, TAction>()
where
    TAlgo: Algo<TState, TAction>,
    TState: Clone + std::fmt::Debug,
    TAction: Clone + std::fmt::Debug,
{
    let points = random_points();
    let (states, actions) = all_states::<TAlgo, TState, TAction>(points);
    show::<TAlgo, TState, TAction>(&states, &actions);
}

fn main() {
    let matches = App::new("gav")
        .version("0.1")
        .author("Dmitry Matrokhin <matrokhin.d@gmail.com>")
        .about("Geometry Algorithms Visualization")
        .arg(
            Arg::with_name("algo")
                .long("algorithm")
                .takes_value(true)
                .possible_values(&[
                    "closest_pair_sl",
                    "graham",
                    "shamos_hoey",
                    "two_nearest_dnc",
                ])
                .required(true)
                .index(1),
        )
        .get_matches();

    match matches.value_of("algo").unwrap() {
        "graham" => run::<Graham, graham::State, graham::Action>(),
        "two_nearest_dnc" => run::<TwoNearest, two_nearest_dnc::State, two_nearest_dnc::Action>(),
        "closest_pair_sl" => {
            run::<ClosestPairSweepLine, closest_pair_sl::State, closest_pair_sl::Action>()
        }
        "shamos_hoey" => run::<ShamosHoey, shamos_hoey::State, shamos_hoey::Action>(),
        _ => panic!(),
    }
}
