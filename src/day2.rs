use super::{parse_input, Output};
use itertools::Itertools;

const INPUT: &str = "day2.txt";
const NUM_LOWER_ALPHA: usize = 26;

pub fn run() -> Output<usize, String> {
    let input: Vec<String> = parse_input(INPUT).lines().map(Into::into).collect();
    Output {
        a: part_a(&input),
        b: part_b(&input),
    }
}

pub fn part_a(input: &[String]) -> usize {
    let (two, three) = input.iter().fold((0, 0), |(two, three), word| {
        let mut lookup = [0_usize; NUM_LOWER_ALPHA];
        for ch in word
            .chars()
            .filter(|ch| ch.is_ascii_alphabetic())
            .map(|ch| ch.to_ascii_lowercase())
        {
            let idx = (ch as usize) - 'a' as usize;
            // could overflow
            lookup[idx] += 1;
        }
        let two_inc = if lookup.iter().any(|&v| v == 2) { 1 } else { 0 };
        let three_inc = if lookup.iter().any(|&v| v == 3) { 1 } else { 0 };
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
        let diff =
            str1.chars().zip(str2.chars()).fold(
                0,
                |diff, (str1, str2)| {
                    if str1 == str2 {
                        diff
                    } else {
                        diff + 1
                    }
                },
            );

        // Construct and return a new string, O(n) + allocation
        //
        // NOTE: we could also return the `idx` to the character to remove in the String from
        // `fold` above but `String::remove` is O(n) + allocation for `cloning the old String`
        if diff == 1 {
            return str1
                .chars()
                .zip(str2.chars())
                .filter(|(a, b)| a == b)
                .map(|(a, b)| a)
                .collect();
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
