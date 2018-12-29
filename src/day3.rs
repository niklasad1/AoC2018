use super::{parse_input, Output};
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
}

const INPUT: &str = "day3.txt";

#[derive(Debug)]
struct Square {
    pub id: usize,
    pub from_left: usize,
    pub from_top: usize,
    pub width: usize,
    pub depth: usize,
}

impl From<&str> for Square {
    fn from(input: &str) -> Self {
        let caps = RE.captures(input).unwrap();
        Self {
            id: caps[1].parse().expect("well-formed input; qed"),
            from_left: caps[2].parse().expect("well-formed input; qed"),
            from_top: caps[3].parse().expect("well-formed input; qed"),
            width: caps[4].parse().expect("well-formed input; qed"),
            depth: caps[5].parse().expect("well-formed input; qed"),
        }
    }
}

pub fn run() -> Output<usize, usize> {
    let squares: Vec<Square> = parse_input(INPUT).lines().map(Into::into).collect();
    let mut grid = [[0_u8; 1000]; 1000];

    for square in squares.iter() {
        for row in grid.iter_mut().skip(square.from_top).take(square.depth) {
            for seen in row.iter_mut().skip(square.from_left).take(square.width) {
                *seen += 1;
            }
        }
    }

    Output {
        a: part_a(&mut grid[..]),
        b: part_b(&squares, &mut grid[..]),
    }
}

fn part_a(grid: &mut [[u8; 1000]]) -> usize {
    grid.iter().fold(0, |acc, row| {
        let sum = row.iter().filter(|&&s| s > 1).count();
        acc + sum
    })
}

fn part_b(squares: &[Square], grid: &mut [[u8; 1000]]) -> usize {
    for square in squares.iter() {
        let claimed_once: bool = grid
            .iter()
            .skip(square.from_top)
            .take(square.depth)
            .map(|r| {
                r.iter()
                    .skip(square.from_left)
                    .take(square.width)
                    .all(|&c| c == 1)
            })
            .all(|c| c);
        if claimed_once {
            return square.id;
        }
    }
    unreachable!("Exactly one ID claimed; qed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let square = Square::from("#1 @ 11,333: 400x4");
        assert_eq!(square.id, 1);
        assert_eq!(square.from_left, 11);
        assert_eq!(square.from_top, 333);
        assert_eq!(square.width, 400);
        assert_eq!(square.depth, 4);
    }

    #[test]
    fn full() {
        assert_eq!(run(), Output { a: 111935, b: 650 });
    }
}
