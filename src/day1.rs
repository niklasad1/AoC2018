use super::{parse_input, Output};
use std::collections::HashSet;

const INPUT: &str = "day1.txt";
type Counter = isize;

pub fn run() -> Output<Counter> {
    let input: Vec<Counter> = parse_input(INPUT)
        .lines()
        .map(|line| line.parse().expect("well-formed integers"))
        .collect();
    Output {
        a: part_a(&input),
        b: part_b(&input),
    }
}

fn part_a(numbers: &[Counter]) -> Counter {
    numbers.iter().sum()
}

fn part_b(numbers: &[Counter]) -> Counter {
    let mut current = 0;
    let mut seen = HashSet::new();
    seen.insert(current);

    for n in numbers.iter().cycle() {
        current += n;
        if !seen.insert(current) {
            break;
        }
    }
    current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        panic!("{:?}", run());
    }
}
