#![allow(unused)]

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug)]
pub struct Output<T> {
    a: T,
    b: T,
}

pub fn parse_input(input: &str) -> String {
    let file = File::open(input).expect("File open error");
    let mut buf_reader = BufReader::new(file);
    let mut s = String::new();
    buf_reader.read_to_string(&mut s).expect("File read error");
    s
}
