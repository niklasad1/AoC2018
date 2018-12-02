use super::{parse_input, Output};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const INPUT: &str = "day2.txt";

pub fn run() -> Output<usize, String> {
    let input: Vec<String> = parse_input(INPUT)
        .lines()
        .map(Into::into)
        .collect();
    Output {
        a: part_a(&input),
        b: part_b(&input),
    }
}

fn increment_if_exist(val: usize, lookup: &HashMap<char, usize>) -> usize {
    std::cmp::min(1, lookup.iter().filter(|(_, v)| **v == val).count())
}

pub fn part_a(input: &[String]) -> usize {

    let (two, three) = input.iter().fold((0, 0), |(two, three), word| {
        let mut lookup_table = HashMap::new();
        for ch in word.chars() {
            *lookup_table.entry(ch).or_insert(0) += 1;
        }
        //FIXME: ideally, the values should be ordered and `binary searched`
        let two_inc = increment_if_exist(2, &lookup_table);
        let three_inc = increment_if_exist(3, &lookup_table);
        (two + two_inc, three + three_inc)
    });

    two * three
}

pub fn part_b(input: &[String]) -> String {
    for (str1, str2) in input.iter().tuple_combinations() {
        // Get number of chars that differ, O(n)
        //
        // NOTE: we could collect this into a `String` directly here but would allocate
        // a String for every iteration with is slow                             )
        let diff = str1.chars().zip(str2.chars()).fold(0, |diff, (str1, str2)| {
            if str1 == str2 { diff } else { diff + 1 }
        });

        // Construct and return a new string, O(n) + allocation
        //
        // NOTE: we could also return the `idx` to the character to remove in the String from
        // `fold` above but `String::remove` is O(n) + allocation for `cloning the old String`
        if diff == 1 {
            return str1.chars().zip(str2.chars()).filter(|(a, b)| a == b).map(|(a, b)| a).collect()
        }
    }
    unreachable!("the input should have two items that differ just one; qed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = vec![
            "abcdef".to_string(),
            "bababc".to_string(),
            "abbcde".to_string(),
            "abcccd".to_string(),
            "aabcdd".to_string(),
            "abcdee".to_string(),
            "ababab".to_string(),
        ];
        assert_eq!(part_a(&input), 12);
    }

    #[test]
    fn test_part_b() {
        let input = vec![
            "abcde".to_string(),
            "fghij".to_string(),
            "klmno".to_string(),
            "pqrst".to_string(),
            "fguij".to_string(),
            "axcye".to_string(),
            "wvxyz".to_string(),
        ];
        assert_eq!(part_b(&input), "fgij".to_string());
    }

    #[test]
    fn test() {
        panic!("{:?}", run());
    }
}
