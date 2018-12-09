use super::{parse_input, Output};
use regex::Regex;
use std::collections::{HashMap, HashSet};

const INPUT: &str = "day6.txt";

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+).{2}(\d+)").unwrap();
}

#[derive(Debug)]
struct Infinite {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl Default for Infinite {
    fn default() -> Self {
        Self {
            max_x: 0,
            max_y: 0,
            min_x: isize::max_value(),
            min_y: isize::max_value(),
        }
    }
}

#[derive(Default, Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Coordinate {
    pub x: isize,
    pub y: isize,
}

impl Coordinate {
    fn manhattan_distance(&self, other: &Coordinate) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn is_infinite(&self, inf: &Infinite) -> bool {
        self.x == inf.max_x || self.x == inf.min_x || self.y == inf.max_y || self.y == inf.min_y
    }
}

pub fn run() -> Output<isize, isize> {
    let raw = parse_input(INPUT);

    let mut coords = HashSet::new();
    let mut inf = Infinite::default();

    for cap in RE.captures_iter(&raw) {
        let x: isize = cap[1].parse().expect("well formed input");
        let y: isize = cap[2].parse().expect("well formed input");
        let cord = Coordinate { x, y };
        coords.insert(cord);

        inf.max_x = std::cmp::max(x, inf.max_x);
        inf.max_y = std::cmp::max(y, inf.max_y);
        debug_assert!(x >= 0 && y >= 0, "no negative points");
        inf.min_x = std::cmp::min(x, inf.min_x);
        inf.min_y = std::cmp::min(y, inf.min_y);
    }

    Output {
        a: part_a(&coords, &inf),
        b: part_b(&coords, &inf),
    }
}

fn part_a(coords: &HashSet<Coordinate>, inf: &Infinite) -> isize {
    let mut points = HashMap::new();

    for x in inf.min_x..=inf.max_x {
        for y in inf.min_y..=inf.max_y {
            let current_cord = Coordinate { x, y };

            if current_cord.is_infinite(inf) {
                continue;
            }

            let mut shortest_dist = isize::max_value();
            let mut uniq_cord = None;

            for known_cord in coords.iter() {
                let current_dist = known_cord.manhattan_distance(&current_cord);
                if current_dist < shortest_dist {
                    shortest_dist = current_dist;
                    uniq_cord = Some(known_cord);
                } else if shortest_dist == current_dist {
                    uniq_cord = None;
                }
            }

            if let Some(coord) = uniq_cord {
                *points.entry(coord).or_insert(0) += 1;
            }
        }
    }
    *points.values().max().unwrap()
}

fn part_b(coords: &HashSet<Coordinate>, inf: &Infinite) -> isize {
    const MAX_DISTANCE: isize = 9_999;
    let mut num_points = 0;

    for x in inf.min_x..=inf.max_x {
        for y in inf.min_y..=inf.max_y {
            let current_cord = Coordinate { x, y };

            if current_cord.is_infinite(inf) {
                continue;
            }

            let total_distance: isize = coords
                .iter()
                .map(|c| c.manhattan_distance(&current_cord))
                .sum();

            if total_distance <= MAX_DISTANCE {
                num_points += 1;
            }
        }
    }
    num_points
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn full() {
        assert_eq!(run(), Output { a: 5333, b: 35334 });
    }
}
