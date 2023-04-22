use std::fs;
use std::collections::HashMap;

fn check_for_length(transmission: &String, length: usize) -> usize {
    let index_ref = length - 1;
    'outer: for index in index_ref..transmission.len() {
        let mut checker: HashMap<String, bool> = HashMap::new();
        for check in index-index_ref..=index {
            let current = transmission.chars().nth(check).unwrap();
            //println!("{} {}", check, current);
            if checker.contains_key(&current.to_string()) {
                continue 'outer;
            }
            checker.insert(current.to_string(), true);
        }
        return index + 1;
    }
    return 0;
}

fn part1(transmission: &String) {
    println!("Part 1: {}", check_for_length(transmission, 4));
}

fn part2(transmission: &String) {
    println!("Part 2: {}", check_for_length(transmission, 14));
}

fn main() {
    let files = vec!["sample.txt", "input.txt"];
    for file in files {
        println!("Reading {}", file);
        let content = fs::read_to_string(file).expect("Cannot read file");
        println!("Transmission: {}", content);
        part1(&content);
        part2(&content);
    }
}
