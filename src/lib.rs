extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate regex;

pub mod day1;
#[allow(unused)]
pub mod day10;
#[allow(unused)]
pub mod day11;
#[allow(unused)]
pub mod day12;
#[allow(unused)]
pub mod day13;
#[allow(unused)]
pub mod day14;
#[allow(unused)]
pub mod day15;
#[allow(unused)]
pub mod day16;
#[allow(unused)]
pub mod day17;
#[allow(unused)]
pub mod day19;
pub mod day2;
#[allow(unused)]
pub mod day20;
#[allow(unused)]
pub mod day21;
#[allow(unused)]
pub mod day22;
#[allow(unused)]
pub mod day23;
#[allow(unused)]
pub mod day24;
#[allow(unused)]
pub mod day25;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
#[allow(unused)]
pub mod day7;
#[allow(unused)]
pub mod day8;
#[allow(unused)]
pub mod day9;

use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug, PartialEq)]
pub struct Output<A, B> {
    a: A,
    b: B,
}

pub fn parse_input(input: &str) -> String {
    let file = File::open(input).expect("File open error");
    let mut buf_reader = BufReader::new(file);
    let mut s = String::new();
    buf_reader.read_to_string(&mut s).expect("File read error");
    s
}
