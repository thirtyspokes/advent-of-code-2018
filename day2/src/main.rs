use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let input = File::open("inputs/part-1.txt").expect("Couldn't open input!");
    println!("Part 1: {}", calculate_checksum(&input));

    let (first, second) = find_nearly_identical_ids(&input);
    println!("Part 2: {} and {}", first, second)
}

fn calculate_checksum(file: &File) -> isize {
    let mut reader = BufReader::new(file);
    let mut twos = 0;
    let mut threes = 0;

    for line in reader.by_ref().lines() {
        let score = get_freq_score(line.unwrap());

        twos += score.has_two;
        threes += score.has_three;
    }
    
    twos * threes
}

fn find_nearly_identical_ids(file: &File) -> (String, String) {
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
        .map(|line| line.unwrap())
        .collect();

    for (i, line) in lines.iter().enumerate() {
        for comparison in &lines[i..] {
            if nearly_identical(line.to_string(), comparison.to_string()) {
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

fn get_freq_score(id: String) -> Score {
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

fn nearly_identical(a: String, b: String) -> bool {
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
        assert_eq!(6422, calculate_checksum(&input));
    }

    #[test]
    fn test_part_two() {
        let input = File::open("inputs/part-1.txt").expect("Couldn't open input!");
        assert_eq!(
            ("qcslyvphgkrmddawljuefotxbh".to_string(), "qcslyvphgkrmrdawljuefotxbh".to_string()),
            find_nearly_identical_ids(&input)
        )
    }

    #[test]
    fn test_nearly_identical() {
        assert_eq!(
            true,
            nearly_identical("fghij".to_string(), "fguij".to_string())
        );
        assert_eq!(
            true,
            nearly_identical("xghij".to_string(), "fghij".to_string())
        );
        assert_eq!(
            true,
            nearly_identical("fghij".to_string(), "fghix".to_string())
        );
        assert_eq!(
            false,
            nearly_identical("fghij".to_string(), "fghij".to_string())
        );
        assert_eq!(
            false,
            nearly_identical("fghix".to_string(), "fguij".to_string())
        );
        assert_eq!(
            false,
            nearly_identical("abcde".to_string(), "fguij".to_string())
        );
    }

    #[test]
    fn test_get_freq_score() {
        assert_eq!(
            Score { has_two: 0, has_three: 0},
            get_freq_score("abcdef".to_string())
        );
        assert_eq!(
            Score { has_two: 1, has_three: 1},
            get_freq_score("bababc".to_string())
        );
        assert_eq!(
            Score { has_two: 1, has_three: 0},
            get_freq_score("abbcde".to_string())
        );
        assert_eq!(
            Score { has_two: 0, has_three: 1},
            get_freq_score("abcccd".to_string())
        );
        assert_eq!(
            Score { has_two: 1, has_three: 0},
            get_freq_score("aabcdd".to_string())
        );
        assert_eq!(
            Score { has_two: 1, has_three: 0},
            get_freq_score("abcdee".to_string())
        );
        assert_eq!(
            Score { has_two: 0, has_three: 1},
            get_freq_score("ababab".to_string())
        );
    }
}