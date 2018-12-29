use super::{parse_input, Output};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

const INPUT: &str = "day7.txt";

#[derive(Clone, Debug, PartialEq)]
enum WorkerStatus {
    Idle,
    Busy(char, usize),
}

type DependencyGraph = HashMap<char, HashSet<char>>;

#[derive(Debug)]
struct WorkerPool(Vec<WorkerStatus>);

impl WorkerPool {
    pub fn new(n: usize) -> Self {
        WorkerPool(vec![WorkerStatus::Idle; n])
    }

    fn execute(&mut self) -> Vec<char> {
        let mut completed = Vec::new();
        for state in self.0.iter_mut() {
            if let WorkerStatus::Busy(ch, ref n) = state {
                let rem = n - 1;
                if rem == 0 {
                    completed.push(*ch);
                    *state = WorkerStatus::Idle;
                } else {
                    *state = WorkerStatus::Busy(*ch, rem);
                }
            }
        }
        completed
    }

    fn try_dispatch(&mut self, work: char) -> bool {
        for state in self.0.iter_mut() {
            if let WorkerStatus::Idle = state {
                *state = WorkerStatus::Busy(work, work as usize - 'A' as usize + 61);
                return true;
            }
        }
        false
    }

    fn available_spots(&self) -> usize {
        self.0.iter().fold(0, |acc, state| {
            if let WorkerStatus::Idle = state {
                acc + 1
            } else {
                acc
            }
        })
    }
}

fn from_str(steps: &str) -> DependencyGraph {
    let mut dep = DependencyGraph::new();
    for step in steps.lines().map(|l| l.trim()) {
        let s = step.chars().nth(5).expect("well-formed input; qed");
        let finish_before = step.chars().nth(36).expect("well-formed input; qed");
        dep.entry(s).or_default().insert(finish_before);
        dep.entry(finish_before).or_default();
    }
    dep
}

pub fn run() -> Output<String, usize> {
    let steps = from_str(parse_input(INPUT).as_str());
    Output {
        a: part_a(steps.clone()),
        b: part_b(steps),
    }
}

fn part_a(mut queue: DependencyGraph) -> String {
    let mut finished = String::with_capacity(queue.keys().len());

    while !queue.is_empty() {
        let mut candidates = BTreeSet::new();
        for c1 in queue.keys() {
            if queue
                .iter()
                .filter(|(c2, _dep)| c1 != *c2)
                .all(|(_, dep)| !dep.contains(&c1))
            {
                candidates.insert(*c1);
            }
        }

        let step = match candidates.iter().nth(0) {
            Some(ref step) => **step,
            None => *queue
                .keys()
                .nth(0)
                .expect("queue has at least one element; qed"),
        };

        queue.remove(&step);
        finished.push(step);
    }

    finished
}

fn part_b(mut queue: DependencyGraph) -> usize {
    let mut pool = WorkerPool::new(5);
    let mut pending: HashMap<char, HashSet<char>> = HashMap::new();
    let mut secs = 0;

    while !queue.is_empty() {
        for step in pool.execute() {
            pending.remove(&step);
            queue.remove(&step);
        }

        let available_spots = pool.available_spots();
        if available_spots > 0 {
            let mut candidates: BTreeMap<char, HashSet<char>> = BTreeMap::new();
            for (curr_step, curr_dep) in &queue {
                if pending.contains_key(&curr_step) || pending.values().any(|dep| dep.contains(&curr_step)) {
                    continue;
                }
                if queue
                    .iter()
                    .filter(|(&next_step, _)| *curr_step != next_step)
                    .all(|(_, dep)| !dep.contains(&curr_step)) {
                    candidates.insert(*curr_step, curr_dep.clone());
                }
            }

            for (step, dep) in candidates.into_iter().take(available_spots) {
                pool.try_dispatch(step);
                pending.insert(step, dep);
            }
        }

        if !queue.is_empty() {
            secs += 1;
        }
    }
    secs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full() {
        assert_eq!(
            run(),
            Output {
                a: "GLMVWXZDKOUCEJRHFAPITSBQNY".to_string(),
                b: 1105,
            }
        );
    }
}
