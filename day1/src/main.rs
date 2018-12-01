use std::io::prelude::*;
use std::io::BufReader;
use std::io;
use std::io::SeekFrom;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let file = File::open("inputs/part-1.txt").expect("Dang");
    let reader = BufReader::new(file);
    match calculate_frequency(reader) {
        Ok(result) => println!("Part 1: {}", result),
        Err(_) => println!("Unable to complete part 1"),
    }

    let file = File::open("inputs/part-1.txt").expect("Dang");
    match calculate_loop_frequency(file) {
        Ok(result) => println!("Part 2: {}", result),
        Err(_) => println!("Unable to complete part 2"),
    }
}

fn calculate_frequency(reader: BufReader<File>) -> io::Result<i64> {
    let mut frequency = 0;

    for line in reader.lines() {
        let (sign, amount) = parse_line(line?);

        if sign == "-" {
            frequency -= amount
        } else {
            frequency += amount
        }
    }

    Ok(frequency)
}

fn calculate_loop_frequency(file: File) -> io::Result<i64> {
    let mut reader = BufReader::new(file);
    let mut seen: HashMap<i64, bool> = HashMap::new();
    let mut frequency = 0;

    loop {
        reader.seek(SeekFrom::Start(0)).expect("Failed to reset");
        for line in reader.by_ref().lines() {
            let (sign, amount) = parse_line(line?);
            
            if sign == "-" {
                frequency -= amount
            } else {
                frequency += amount
            }

            if seen.contains_key(&frequency) {
                return Ok(frequency);
            } else {
                seen.insert(frequency, true);
            }
        }
    }
}

fn parse_line(line: String) -> (String, i64) {
    let sign = String::from(&line[0..1]);
    let amount = &line[1..].parse::<i64>().unwrap();

    (sign, *amount)
}