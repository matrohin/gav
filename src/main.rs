mod algos;
mod common;
mod draw_utils;

use crate::algos::closest_pair_dnc::{self, ClosestPairDivideAndConquer};
use crate::algos::closest_pair_sl::{self, ClosestPairSweepLine};
use crate::algos::convex_hull_dnc::{self, ConvexHullDivideAndConquer};
use crate::algos::graham::{self, Graham};
use crate::algos::shamos_hoey::{self, ShamosHoey};
use crate::algos::Algo;
use crate::common::*;
use clap::{value_t, App, Arg};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use rand::rngs::{OsRng, StdRng};
use rand::{Rng, RngCore, SeedableRng};
use raqote::{DrawTarget, SolidSource, Transform};

fn random_points(n: usize, mut rng: impl Rng) -> Vec<Point> {
    let mut res = Vec::with_capacity(n);
    for _ in 0..n {
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

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

fn get_next_index(window: &Window, index: usize, max_index: usize) -> usize {
    if window.is_key_pressed(Key::Right, KeyRepeat::Yes) {
        std::cmp::min(index + 1, max_index)
    } else if window.is_key_pressed(Key::Left, KeyRepeat::Yes) {
        index.saturating_sub(1)
    } else if window.is_key_pressed(Key::Home, KeyRepeat::No) || index == std::usize::MAX {
        0
    } else if window.is_key_pressed(Key::End, KeyRepeat::No) {
        max_index
    } else {
        index
    }
}

fn show<TAlgo, TState, TAction>(states: &Vec<TState>, actions: &Vec<TAction>)
where
    TAlgo: Algo<TState, TAction>,
    TState: Clone + std::fmt::Debug,
    TAction: Clone + std::fmt::Debug,
{
    let title = "Geometry Algorithms Visualization";
    let mut window = Window::new(title, WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    let mut index = std::usize::MAX;
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
        let new_index = get_next_index(&window, index, actions.len() * 2);
        if new_index != index {
            index = new_index;
            dt.clear(SolidSource::from_unpremultiplied_argb(0, 0, 0, 0xff));
            if index % 2 == 0 {
                TAlgo::draw_state(&mut dt, &states[index / 2]);
            } else {
                TAlgo::draw_state(&mut dt, &states[index / 2]);
                TAlgo::draw_action(&mut dt, &actions[index / 2]);
            }
            window
                .update_with_buffer(dt.get_data(), size.0, size.1)
                .unwrap();
        } else {
            window.update();
        }
    }
}

fn run<TAlgo, TState, TAction>(points: Vec<Point>)
where
    TAlgo: Algo<TState, TAction>,
    TState: Clone + std::fmt::Debug,
    TAction: Clone + std::fmt::Debug,
{
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
                    "closest_pair_dnc",
                    "closest_pair_sl",
                    "convex_hull_dnc",
                    "graham",
                    "shamos_hoey",
                ])
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("seed")
                .long("seed")
                .short("s")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("number")
                .long("number")
                .short("n")
                .takes_value(true)
                .default_value("50")
                .validator(is_number),
        )
        .get_matches();

    let n = value_t!(matches, "number", usize).unwrap();
    let seed = value_t!(matches, "seed", u64).unwrap_or_else(|_| OsRng.next_u64());

    println!("Seed: {}", seed);
    let points = random_points(n, StdRng::seed_from_u64(seed));

    match matches.value_of("algo").unwrap() {
        "closest_pair_dnc" => {
            run::<ClosestPairDivideAndConquer, closest_pair_dnc::State, closest_pair_dnc::Action>(
                points,
            )
        }
        "closest_pair_sl" => {
            run::<ClosestPairSweepLine, closest_pair_sl::State, closest_pair_sl::Action>(points)
        }
        "convex_hull_dnc" => {
            run::<ConvexHullDivideAndConquer, convex_hull_dnc::State, convex_hull_dnc::Action>(
                points,
            )
        }
        "graham" => run::<Graham, graham::State, graham::Action>(points),
        "shamos_hoey" => run::<ShamosHoey, shamos_hoey::State, shamos_hoey::Action>(points),
        _ => panic!(),
    }
}

fn is_number(val: String) -> Result<(), String> {
    let val = val
        .parse::<usize>()
        .map_err(|e| format!("failed to parse a number: {}", e))?;
    if val <= 3 {
        Err(String::from(
            "the number of points should be greater than 3",
        ))
    } else {
        Ok(())
    }
}
