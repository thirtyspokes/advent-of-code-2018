use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let input = File::open("inputs/part-1.txt").expect("Couldn't open input!");
    
    let result = calculate_frequency(&input);
    println!("Part 1: {}", result);

    let result = calculate_loop_frequency(&input);
    println!("Part 2: {}", result);
}

fn calculate_frequency(input_file: &File) -> isize {
    let reader = BufReader::new(input_file);

    reader.lines()
        .map(|line| line.unwrap())
        .map(|change| change.parse::<isize>().unwrap())
        .sum()
}

fn calculate_loop_frequency(input_file: &File) -> isize {
    let mut reader = BufReader::new(input_file);
    let mut seen: HashMap<isize, bool> = HashMap::new();
    let mut frequency: isize = 0;

    loop {
        reader.seek(SeekFrom::Start(0)).expect("Failed to seek to start of file!");

        for line in reader.by_ref().lines() {
            frequency += line.unwrap().parse::<isize>().unwrap();

            if seen.contains_key(&frequency) {
                return frequency;
            } else {
                seen.insert(frequency, true);
            }
        }
    }
}