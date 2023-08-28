use std::fs::File;
use std::io::{ Lines, BufReader };


const TARGET_VOLUME: u32 = 150;

pub(crate) fn solve(input: Lines<BufReader<File>>) {
    // Recursive backtracking

    let mut containers: Vec<u32> = Vec::new();
    for line in input {
        let v = line.unwrap().parse::<u32>().unwrap();
        containers.push(v);
    }

    // sort from largest to smallest
    containers.sort();
    containers.reverse();

    let part1 = container_combinations(&containers, 0, -1);
    let part2 = part_2(&containers);

    println!("Part 1: Combinations of containers: {}", part1);
    println!("Part 2: {}", part2);
}


fn container_combinations(
    available_containers: &Vec<u32>,
    running_total: u32,
    last_container_used: i32) -> u32 {

    let start_index: usize = (last_container_used+1).try_into().unwrap();
    let mut combos = 0;
    for i in start_index..available_containers.len() {
        let e = available_containers[i];
        let v = e + running_total;

        if v == TARGET_VOLUME {
            combos += 1;
        } else if v < TARGET_VOLUME {
            combos += container_combinations(available_containers, running_total+e, i.try_into().unwrap())
        }
    }

    combos
}

fn part_2(available_containers: &Vec<u32>) -> u32 {
    let n = minimum_number_of_containers(available_containers, 0, -1);
    return n_size_combos_that_sum_to_target(available_containers, 0, -1, 0, n);
}

fn minimum_number_of_containers(
    available_containers: &Vec<u32>,
    running_total: u32,
    last_container_used: i32) -> u32 {

    let start_index: usize = (last_container_used+1).try_into().unwrap();
    for i in start_index..available_containers.len() {
        let e = available_containers[i];
        let v = e + running_total;

        if v == TARGET_VOLUME {
            return 1;
        } else if v < TARGET_VOLUME {
            let recurse = minimum_number_of_containers(available_containers, v, i.try_into().unwrap());
            if recurse > 0 {
                return recurse + 1;
            }
        }
    }
    0
}

fn n_size_combos_that_sum_to_target(
    available_containers: &Vec<u32>,
    running_total: u32,
    last_container_used: i32,
    num_containers_used: u32,
    n: u32) -> u32 {

    if num_containers_used >= n {
        return 0;
    }

    let mut combos = 0;
    let start_index: usize = (last_container_used+1).try_into().unwrap();
    for i in start_index..available_containers.len() {
        let e = available_containers[i];
        let v = e + running_total;

        if v == TARGET_VOLUME {
            combos += 1;
        } else if v < TARGET_VOLUME {
            combos += n_size_combos_that_sum_to_target(
                available_containers, 
                v, 
                i.try_into().unwrap(), 
                num_containers_used+1, n);
        }

    }
    
    combos
}
