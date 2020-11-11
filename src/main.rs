mod algos;
mod common;
mod draw_context;
mod ui;

use crate::algos::closest_pair_dnc::ClosestPairDivideAndConquer;
use crate::algos::closest_pair_sl::ClosestPairSweepLine;
use crate::algos::convex_hull_dnc::ConvexHullDivideAndConquer;
use crate::algos::graham::Graham;
use crate::algos::graham_andrew::GrahamAndrew;
use crate::algos::shamos_hoey::ShamosHoey;
use crate::algos::{all_states, Algo};
use crate::common::*;
use crate::ui::show;
use clap::{value_t, App, Arg};
use rand::rngs::{OsRng, StdRng};
use rand::{Rng, RngCore, SeedableRng};

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

fn run<TAlgo>(points: Vec<Point>, window_size: usize, draw_width: f32)
where
    TAlgo: Algo,
{
    let (states, actions) = all_states::<TAlgo>(points);
    show::<TAlgo>(&states, &actions, window_size, draw_width);
}

fn main() {
    let matches = App::new("gav")
        .version("0.2")
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
                    "graham_andrew",
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
        .arg(
            Arg::with_name("window size")
                .long("window_size")
                .short("w")
                .takes_value(true)
                .default_value("1000"),
        )
        .arg(
            Arg::with_name("draw width")
                .long("draw_width")
                .short("d")
                .takes_value(true)
                .default_value("0.1"),
        )
        .get_matches();

    let n = value_t!(matches, "number", usize).unwrap();
    let seed = value_t!(matches, "seed", u64).unwrap_or_else(|_| OsRng.next_u64());
    let window_size = value_t!(matches, "window size", usize).unwrap();
    let draw_width = value_t!(matches, "draw width", f32).unwrap();

    println!("Seed: {}", seed);
    let points = random_points(n, StdRng::seed_from_u64(seed));

    match matches.value_of("algo").unwrap() {
        "closest_pair_dnc" => run::<ClosestPairDivideAndConquer>(points, window_size, draw_width),
        "closest_pair_sl" => run::<ClosestPairSweepLine>(points, window_size, draw_width),
        "convex_hull_dnc" => run::<ConvexHullDivideAndConquer>(points, window_size, draw_width),
        "graham_andrew" => run::<GrahamAndrew>(points, window_size, draw_width),
        "graham" => run::<Graham>(points, window_size, draw_width),
        "shamos_hoey" => run::<ShamosHoey>(points, window_size, draw_width),
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
