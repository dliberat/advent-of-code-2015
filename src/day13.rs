use std::fs::File;
use std::io::{ Lines, BufReader };
use std::collections::HashMap;
use itertools::{self, Itertools};


#[derive(Debug)]
struct Entry {
    principal: String,
    change_in_happiness: i32,
    neighbor: String,
}

type HappinessMap = HashMap<String, HashMap<String, i32>>;

pub(crate) fn solve(input: Lines<BufReader<File>>) {

    let mut happiness_map: HappinessMap = HashMap::new();

    for line in input {
        let entry = line_to_entry(line.unwrap());
        
        if !happiness_map.contains_key(&entry.principal) {
            let neighbors: HashMap<String, i32> = HashMap::new();
            happiness_map.insert(entry.principal.clone(), neighbors);
        }

        let neighbors_for_principal = happiness_map.get_mut(&entry.principal).unwrap();
        neighbors_for_principal.insert(entry.neighbor, entry.change_in_happiness);
    }

    let part1 = part1_brute_force(&happiness_map);

    add_self_to_map(&mut happiness_map);
    let part2 = part1_brute_force(&happiness_map);
    
    println!("Part 1: Total change in happiness: {}", part1);
    println!("Part 2: Total change in happiness: {}", part2);
}

fn add_self_to_map(happiness_map: &mut HappinessMap) {
    let mut neighbors_of_self: HashMap<String, i32> = HashMap::new();

    for (neighbor_name, edges) in &mut *happiness_map {
        neighbors_of_self.insert(neighbor_name.clone(), 0);
        edges.insert(String::from("self"), 0);
    }

    happiness_map.insert(String::from("self"), neighbors_of_self);
}

fn part1_brute_force(happiness: &HappinessMap) -> i32 {
    let keys = happiness.keys().collect_vec();
    let perms = keys.iter().permutations(keys.len());

    let mut highest_happiness = i32::MIN;

    for permutation in perms {
        let h = measure_happiness_for_permutation(permutation, &happiness);
        if h > highest_happiness {
            highest_happiness = h;
        }
    }

    return highest_happiness;
}

fn measure_happiness_for_permutation(permutation: Vec<&&String>, happiness_map: &HappinessMap) -> i32 {
    let guest_count = permutation.len();

    let mut total = 0;
    let mut neighbor1: String;
    let mut neighbor2: String;

    for i in 0..guest_count {
        let principal = (*permutation[i]).clone();
        if i == 0 {
            neighbor1 = (*permutation[guest_count-1]).clone();
        } else {
            neighbor1 = (*permutation[i-1]).clone();
        }

        if i == guest_count-1 {
            neighbor2 = (*permutation[0]).clone();
        } else {
            neighbor2 = (*permutation[i+1]).clone();
        }

        total += measure_happiness_for_individual(principal, neighbor1, neighbor2, happiness_map);
    }
        
    total
}

fn measure_happiness_for_individual(principal: String, neighbor1: String, neighbor2: String, happiness_map: &HappinessMap) -> i32 {
    let map = &happiness_map[&principal];
    let a = map[&neighbor1];
    let b = map[&neighbor2];
    return a+b;
}

fn line_to_entry(line: String) -> Entry {
    // Example input: Alice would gain 54 happiness units by sitting next to Bob.
    let line = line.replace(".", "");
    let split: Vec<&str>= line.split(" ").collect();
    assert!(split.len() == 11);

    let principal = String::from(split[0]);
    let neighbor = String::from(split[10]);
    let mut value = split[3].parse::<i32>().unwrap();
    if split[2] == "lose" {
        value *= -1;
    }
    
    return Entry {
        principal: principal,
        change_in_happiness: value,
        neighbor: neighbor,
    }
}
