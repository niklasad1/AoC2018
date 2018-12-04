use super::{parse_input, Output};
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

lazy_static! {
    static ref RE: Regex = Regex::new(r"#(\d+)").unwrap();
}
const INPUT: &str = "day4.txt";

#[derive(Debug, Default)]
struct Guard {
    last_operation: Operation,
    id: Option<usize>,
    slept_since: Option<usize>,
    db: HashMap<usize, HashMap<usize, usize>>,
}

impl Guard {
    fn insert_in_db(&mut self, mins: usize, start: usize) {
        for min in (start..=60).cycle().take(mins) {
            self.db
                .entry(self.id.expect("should have ID after shift; qed"))
                .and_modify(|inner| *inner.entry(min).or_default() += 1)
                .or_insert_with(|| {
                    let mut map = HashMap::new();
                    map.insert(min, 1);
                    map
                });
        }
    }

    fn step(&mut self, next: &Transistion) {
        match next.op {
            Operation::Wakeup => {
                if let Operation::Sleep = self.last_operation {
                    let since = self
                        .slept_since
                        .take()
                        .expect("sleep and wakeup are balanced; qed");
                    let sleep_time = next
                        .date
                        .minute
                        .checked_sub(since)
                        .expect("wakeup time must be later sleep; qed");
                    self.insert_in_db(sleep_time, since);
                }
            }
            Operation::Shift(id) => {
                self.id = Some(id);
            }
            Operation::Sleep => {
                if Operation::Sleep != self.last_operation {
                    self.slept_since = Some(next.date.minute);
                }
            }
            _ => (),
        };
        self.last_operation = next.op;
    }

    fn most_minutes(&self) -> usize {
        let (&id, minutes) = self
            .db
            .iter()
            .max_by_key(|(_id, mins)| mins.values().sum::<usize>())
            .unwrap();
        let (&min, _cnt) = minutes.iter().max_by_key(|(_min, &cnt)| cnt).unwrap();

        id * min
    }

    fn most_sleeps(&self) -> usize {
        let mut max = 0;
        let mut id = 0;
        let mut min = 0;

        for (&c_id, minutes) in &self.db {
            let (&c_min, &cand) = minutes.iter().max_by_key(|&(_min, cnt)| cnt).unwrap();
            if cand > max {
                id = c_id;
                max = cand;
                min = c_min;
            }
        }
        id * min
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Operation {
    Sleep,
    Wakeup,
    Shift(usize),
    Noop,
}

impl Default for Operation {
    fn default() -> Self {
        Operation::Noop
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Date {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Transistion {
    pub date: Date,
    pub op: Operation,
}

impl Ord for Transistion {
    fn cmp(&self, other: &Transistion) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl PartialOrd for Transistion {
    fn partial_cmp(&self, other: &Transistion) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&str> for Transistion {
    fn from(input: &str) -> Self {
        let (timestamp, action) = input.split_at(19);

        let date = Date {
            year: timestamp
                .get(1..5)
                .expect("well-formed input")
                .parse()
                .expect("Year parse"),
            month: timestamp
                .get(6..8)
                .expect("well-formed input")
                .parse()
                .expect("Month parse"),
            day: timestamp
                .get(9..11)
                .expect("well-formed input")
                .parse()
                .expect("Day parse"),
            hour: timestamp
                .get(12..14)
                .expect("well-formed input")
                .parse()
                .expect("Hour parse"),
            minute: timestamp
                .get(15..17)
                .expect("well-formed input")
                .parse()
                .expect("Minute parse"),
        };

        let op = if action.starts_with("Guard") {
            let id = RE
                .captures(action)
                .expect("well-formed input")
                .get(1)
                .map_or("no match", |m| m.as_str())
                .parse()
                .expect("malformed id");
            Operation::Shift(id)
        } else if date.hour != 0 || date.minute >= 60 {
            Operation::Noop
        } else if action.starts_with("falls asleep") {
            Operation::Sleep
        } else if action.contains("wakes up") {
            Operation::Wakeup
        } else {
            unreachable!();
        };

        Self { date, op }
    }
}

pub fn run() -> Output<usize, usize> {
    let mut transistions: Vec<Transistion> = parse_input(INPUT).lines().map(Into::into).collect();
    transistions.sort();
    let mut guard = Guard::default();
    for t in transistions.iter() {
        guard.step(t);
    }

    Output {
        a: part_a(&guard),
        b: part_b(&guard),
    }
}

fn part_a(guard: &Guard) -> usize {
    guard.most_minutes()
}

fn part_b(guard: &Guard) -> usize {
    guard.most_sleeps()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_b() {
        let mut input: Vec<Transistion> = vec![
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
            "[1518-11-05 00:55] wakes up",
        ]
        .into_iter()
        .map(Into::into)
        .collect();
        input.sort();

        let guard = input.iter().fold(Guard::default(), |mut guard, t| {
            guard.step(t);
            guard
        });

        assert_eq!(part_a(&guard), 240);
        assert_eq!(part_b(&guard), 4455);
    }

    #[test]
    fn full() {
        assert_eq!(run(), Output { a: 0, b: 0 });
    }
}
