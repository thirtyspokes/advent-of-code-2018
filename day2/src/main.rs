use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let input = File::open("inputs/part-1.txt").expect("Couldn't open input!");
    let reader = BufReader::new(&input);
    let lines: Vec<String> = reader.lines()
        .map(|line| line.unwrap())
        .collect();
    
    println!("Part 1: {}", calculate_checksum(&lines));

    let (first, second) = find_nearly_identical_ids(&lines);
    println!("Part 2: {} and {}", first, second)
}

fn calculate_checksum(lines: &Vec<String>) -> isize {
    let mut twos = 0;
    let mut threes = 0;

    for line in lines {
        let score = get_freq_score(line);

        twos += score.has_two;
        threes += score.has_three;
    }
    
    twos * threes
}

fn find_nearly_identical_ids(lines: &Vec<String>) -> (String, String) {
    for (i, line) in lines.iter().enumerate() {
        for comparison in &lines[i..] {
            if nearly_identical(line, comparison) {
                return (line.to_string(), comparison.to_string())
            }
        }
    }

    panic!("No nearly identical strings were found.")
}

#[derive(Debug,PartialEq)]
struct Score {
    has_two: isize,
    has_three: isize,
}

fn get_freq_score(id: &str) -> Score {
    let mut counts: HashMap<char, isize> = HashMap::new();
    let mut score = Score { has_two: 0isize, has_three: 0isize };

    for c in id.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    for (_c, count) in &counts {
        if *count == 2 {
            score.has_two = 1;
        } else if *count == 3 {
            score.has_three = 1;
        }
    }

    score
}

fn nearly_identical(a: &str, b: &str) -> bool {
    let mut found_difference = false;

    // If the strings aren't the same length, 
    // then they can't be nearly identical
    if a.len() != b.len() {
        return false;
    }

    // Actually identical is not nearly identical
    if a == b {
        return false
    }

    for (a_char, b_char) in a.chars().zip(b.chars()) {
        if a_char != b_char {
            if found_difference {
                return false;
            } else {
                found_difference = true
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = File::open("inputs/part-1.txt").expect("Couldn't open input!");
        let reader = BufReader::new(&input);
        let lines: Vec<String> = reader.lines()
            .map(|line| line.unwrap())
            .collect();
        assert_eq!(6422, calculate_checksum(&lines));
    }

    #[test]
    fn test_part_two() {
        let input = File::open("inputs/part-1.txt").expect("Couldn't open input!");
        let reader = BufReader::new(&input);
        let lines: Vec<String> = reader.lines()
            .map(|line| line.unwrap())
            .collect();
        assert_eq!(
            ("qcslyvphgkrmddawljuefotxbh".to_string(), "qcslyvphgkrmrdawljuefotxbh".to_string()),
            find_nearly_identical_ids(&lines)
        )
    }

    #[test]
    fn test_nearly_identical() {
        assert_eq!(
            true,
            nearly_identical("fghij", "fguij")
        );
        assert_eq!(
            true,
            nearly_identical("xghij", "fghij")
        );
        assert_eq!(
            true,
            nearly_identical("fghij", "fghix")
        );
        assert_eq!(
            false,
            nearly_identical("fghij", "fghij")
        );
        assert_eq!(
            false,
            nearly_identical("fghix", "fguij")
        );
        assert_eq!(
            false,
            nearly_identical("abcde", "fguij")
        );
    }

    #[test]
    fn test_get_freq_score() {
        assert_eq!(
            Score { has_two: 0, has_three: 0},
            get_freq_score("abcdef")
        );
        assert_eq!(
            Score { has_two: 1, has_three: 1},
            get_freq_score("bababc")
        );
        assert_eq!(
            Score { has_two: 1, has_three: 0},
            get_freq_score("abbcde")
        );
        assert_eq!(
            Score { has_two: 0, has_three: 1},
            get_freq_score("abcccd")
        );
        assert_eq!(
            Score { has_two: 1, has_three: 0},
            get_freq_score("aabcdd")
        );
        assert_eq!(
            Score { has_two: 1, has_three: 0},
            get_freq_score("abcdee")
        );
        assert_eq!(
            Score { has_two: 0, has_three: 1},
            get_freq_score("ababab")
        );
    }
}