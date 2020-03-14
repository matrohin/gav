mod algos;
mod common;

use crate::algos::graham::{self, Graham};
use crate::algos::Algo;
use crate::common::Point;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn input_points(filename: impl AsRef<Path>) -> Vec<Point> {
    let file = File::open(filename).unwrap();
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let mut it = l.split(' ').map(|s| s.parse());
            Point::new(it.next().unwrap().unwrap(), it.next().unwrap().unwrap())
        })
        .collect()
}

fn solve_with_print<TAlgo, TState, TAction>(points: Vec<Point>)
where
    TAlgo: Algo<TState, TAction>,
    TState: Clone + std::fmt::Debug,
    TAction: Clone + std::fmt::Debug,
{
    let mut state = TAlgo::first_state(points);
    println!("First: {:?}\n", state);

    while !TAlgo::is_final(&state) {
        let (next, action) = TAlgo::next_state(state);
        state = next;
        println!("Action: {:?}\n{:?}\n", action, state);
    }
}

fn main() {
    let points = input_points("in.txt");
    solve_with_print::<Graham, graham::State, graham::Action>(points);
}
