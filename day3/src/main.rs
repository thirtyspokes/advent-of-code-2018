use std::collections::HashMap;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let input = File::open("inputs/part-1.txt").expect("Couldn't open input!");
    let reader = BufReader::new(&input);
    let claims: Vec<Claim> = reader.lines()
        .map(|line| line.unwrap())
        .map(|line| parse_claim(&line))
        .collect();

    let fabric = place_claims(claims);
    let mut count = 0;

    for (_k, entry) in fabric.iter() {
        if entry.len() >= 2 {
            count += 1;
        }
    }

    println!("Part 1: {}", count);

    let mut candidates: HashMap<isize, bool> = HashMap::new();
    for (_k, entry) in fabric.iter() {
        if entry.len() == 1 {
            candidates.insert(entry[0], true);
        }
    }

    for (_k, entry) in fabric.iter() {
        if entry.len() > 1 {
            for i in entry {
                candidates.remove(i);
            }
        }
    }

    println!("Part 2: {:?}", candidates)
}

#[derive(Debug,PartialEq)]
struct Claim {
    id: isize,
    x: isize,
    y: isize,
    width: isize,
    height: isize,
}

fn place_claims(claims: Vec<Claim>) -> HashMap<(isize, isize), Vec<isize>> {
    let mut fabric: HashMap<(isize, isize), Vec<isize>> = HashMap::new();

    for claim in claims {
        // Start at the left side (0 + x)
        for i in claim.x..(claim.x + claim.width) {
            // Move from the top down
            let top = 1000 - claim.y;
            let bottom = top - claim.height;

            for j in bottom..top {
                let loc = (i, j);
                let mut entry = fabric.entry(loc).or_insert(Vec::new());
                entry.push(claim.id);
            }
        }
    }

    fabric
}

fn parse_claim(line: &str) -> Claim {
    let split: Vec<&str> = line.split(" ").collect();
    let id = &split[0][1..].parse::<isize>().unwrap();

    let coords_str_idx = &split[2].len() - 1;
    let coords: &Vec<isize> = &split[2][0..coords_str_idx]
        .split(",")
        .map(|coord| coord.parse::<isize>().unwrap())
        .collect();
    let x = coords[0];
    let y = coords[1];

    let height_width: &Vec<isize> = &split[3]
        .split("x")
        .map(|side| side.parse::<isize>().unwrap())
        .collect();
    let width = height_width[0];
    let height = height_width[1];
    

    Claim {
        id: *id,
        x: x,
        y: y,
        width: width,
        height: height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_claim() {
        let fabric = place_claims(
            vec![
                Claim { id: 1, x: 1, y: 3, width: 4, height: 4},
                Claim { id: 2, x: 3, y: 1, width: 4, height: 4},
                Claim { id: 3, x: 5, y: 5, width: 2, height: 2},
            ]
        );

        // empty area
        assert_eq!(fabric.get(&(2, 998)), None);

        // only contains 2
        assert_eq!(fabric.get(&(3, 998)), Some(&vec![2isize]));

        // overlapping area
        assert_eq!(fabric.get(&(3, 996)), Some(&vec![1isize, 2isize]));
        assert_eq!(fabric.get(&(4, 996)), Some(&vec![1isize, 2isize]));
        assert_eq!(fabric.get(&(3, 995)), Some(&vec![1isize, 2isize]));
        assert_eq!(fabric.get(&(4, 995)), Some(&vec![1isize, 2isize]));
    }

    #[test]
    fn test_parse_claim() {
        assert_eq!(
            parse_claim("#123 @ 3,2: 5x4"),
            Claim { id: 123, x: 3, y: 2, width: 5, height: 4 }
        );
        assert_eq!(
            parse_claim("#1 @ 1,3: 4x4"),
            Claim { id: 1, x: 1, y: 3, width: 4, height: 4 }
        );
        assert_eq!(
            parse_claim("#2 @ 3,1: 4x4"),
            Claim { id: 2, x: 3, y: 1, width: 4, height: 4 }
        );
        assert_eq!(
            parse_claim("#1077 @ 862,1: 17x17"),
            Claim { id: 1077, x: 862, y: 1, width: 17, height: 17 }
        );
    }
}