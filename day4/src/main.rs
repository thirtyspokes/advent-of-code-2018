use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("inputs/part-1.txt").expect("Can't open input");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Couldn't read file");

    let mut entries: Vec<&str> = buf.split("\n").collect();
    entries.sort();

    let results = get_minutes_asleep_by_guard(entries);

    // Part one
    let mut winner = "";
    let mut most_mins = 0;

    let mut most_common_min = 0;
    let mut most_common_count = 0;
    let mut part2_winner = "";

    for (k, v) in &results {
        // Determine if this guard spend the most
        // total minutes asleep
        let mins = v.iter().sum();
        if mins > most_mins {
            winner = k;
            most_mins = mins
        }

        // Determine if this guard has the highest value
        // for any one minute in the time they spent
        // asleep (for part 2)
        for (min, amount) in v.iter().enumerate() {
            if *amount > most_common_count {
                most_common_count = *amount;
                most_common_min = min;
                part2_winner = k;
            }
        }
    }

    // Found the guard who was asleep the most - which minute
    // did they spend asleep the most?
    let sleepiest = results.get(winner).expect("Winning key is missing");
    let mut max_min = 0;
    let mut max_amount = 0;

    for (min, amount) in sleepiest.iter().enumerate() {
        if *amount > max_amount {
            max_min = min;
            max_amount = *amount;
        }
    }

    // Part one
    println!("Sleepiest guard {} was asleep the most on minute {}", winner, max_min);

    // Part two
    println!("Guard {} was asleep on minute {} {} times", part2_winner, most_common_min, most_common_count);
}

fn get_minutes_asleep_by_guard(records: Vec<&str>) -> HashMap<&str, Vec<isize>> {
    let mut results: HashMap<&str, Vec<isize>> = HashMap::new();
    let mut current_guard: &str = "";
    let mut sleep_start: &str = "";

    for entry in records {
        let parts: Vec<&str> = entry.split(" ").collect();

        // A new day
        if parts.len() == 6 {
            current_guard = parts[3];
        } else {
            // fell asleep
            if parts[2] == "falls" {
                sleep_start = &parts[1][3..5];
            } else {
                // Woke up
                let sleep_stop = &parts[1][3..5];

                // if we haven't seen this guard init with a 
                // new vec
                let mut zero_vec = vec![0isize; 60];
                let mut minutes = results
                    .entry(current_guard)
                    .or_insert(zero_vec);
                
                let start = sleep_start.parse::<usize>().expect("could not parse minutes");
                let end   = sleep_stop.parse::<usize>().expect("could not parse minutes");

                for i in start..end {
                    minutes[i] = minutes[i] + 1;
                }
            }
        }
    }

    results
}