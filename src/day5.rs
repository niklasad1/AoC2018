use super::{parse_input, Output};
use std::collections::HashSet;

const INPUT: &str = "day5.txt";

pub fn run() -> Output<usize, usize> {
    let raw = parse_input(INPUT);
    let input = raw.trim();
    Output {
        a: part_a(input),
        b: part_b(input),
    }
}

fn part_a(polymer: &str) -> usize {
    react_polymer(polymer)
}

fn part_b(polymer: &str) -> usize {
    let lookup: HashSet<char> = polymer.chars().collect();

    lookup.iter().fold(usize::max_value(), |curr_shortest, ch| {
        let candidate: String = polymer
            .chars()
            .filter(|&c| !c.eq_ignore_ascii_case(&ch))
            .collect();
        std::cmp::min(react_polymer(&candidate), curr_shortest)
    })
}

fn react_polymer(polymer: &str) -> usize {
    let mut stack = Vec::with_capacity(polymer.len());
    for next in polymer.chars() {
        match stack.last() {
            Some(prev) if has_opposit_polarity(*prev, next) => {
                stack.pop();
            }
            _ => stack.push(next),
        }
    }
    stack.len()
}

#[inline(always)]
fn has_opposit_polarity(ch1: char, ch2: char) -> bool {
    ch1 != ch2 && ch1.eq_ignore_ascii_case(&ch2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(part_a("dabAcCaCBAcCcaDA"), 10);
    }

    #[test]
    fn b() {
        assert_eq!(part_b("dabAcCaCBAcCcaDA"), 4);
    }

    #[test]
    fn full() {
        assert_eq!(run(), Output { a: 10250, b: 6188 });
    }
}
