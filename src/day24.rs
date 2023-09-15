use std::fs::File;
use std::io::{ Lines, BufReader };

pub(crate) fn solve(input: Lines<BufReader<File>>) {

    let present_weights = parse_input(input);
    let present_weights = validated_present_weights(&present_weights);
    
    let part1 = part_1(present_weights.clone());
    println!("Part 1: {}", part1);

    let part2 = part_2(present_weights);
    println!("Part 2: {}", part2);
}

fn part_1(present_weights: Vec<u64>) -> u64 {
    // Since we know exactly how many compartments there are, we know exactly how much weight should end up in each one
    let target = target_weight_per_compartment(3, &present_weights);
    
    // Look for combinations of presents that add up to the target weight.
    // Generating a full list of all the possible combinations that add up to that weight will take a long time,
    for i in 1..present_weights.len() {
        let combos = get_combinations_summing_to_n(target, present_weights.clone(), i);

        let valid_combos: Vec<Vec<u64>> = combos.iter()
            .map(|x| x.clone())
            .filter(|c| is_valid_three_way_split(target, c, &present_weights))
            .collect();

        if valid_combos.len() > 0 {
            let entanglements: Vec<u64> = valid_combos.iter().map(|x| x.iter().product()).collect();
            match entanglements.iter().min() {
                Some(&x) => return x,
                None => panic!("Entanglements vec is empty?!"),
            }
        }
    }

    panic!("Failed to find valid balancing arrangement! Do you need to search with a larger i value?");
}

fn part_2(present_weights: Vec<u64>) -> u64 {
    let target = target_weight_per_compartment(4, &present_weights);

    for i in 1..present_weights.len() {
        let combos = get_combinations_summing_to_n(target, present_weights.clone(), i);

        let valid_combos: Vec<Vec<u64>> = combos.iter()
            .map(|x| x.clone())
            .filter(|c| is_valid_four_way_split(target, c, &present_weights))
            .collect();

        if valid_combos.len() > 0 {
            let entanglements: Vec<u64> = valid_combos.iter().map(|x| x.iter().product()).collect();
            match entanglements.iter().min() {
                Some(&x) => return x,
                None => panic!("Entanglements vec is empty?!"),
            }
        }
    }

    panic!("Failed to find valid balancing arrangement! Do you need to search with a larger i value?");
}

fn validated_present_weights(present_weights: &Vec<u64>) -> Vec<u64> {
    let mut present_weights = present_weights.clone();
    present_weights.sort();
    present_weights.reverse();
    let len = present_weights.len();
    present_weights.dedup();
    if present_weights.len() != len {
        panic!("Input data doesn't conform to the critical assumption that there no two presents have the exact same weight");
    }

    return present_weights;
}

fn get_combinations_summing_to_n(target_sum: u64, nums: Vec<u64>, max_size: usize) -> Vec<Vec<u64>> {
    return combinations_sum_n(&[], target_sum, &nums, max_size);
}

fn combinations_sum_n(prev: &[u64], n: u64, nums: &[u64], max_size: usize) -> Vec<Vec<u64>> {
    // nums must be in decreasing order with no duplicates

    let mut retval: Vec<Vec<u64>> = Vec::new();
    let start_amount: u64 = prev.iter().sum();

    for (i, &val) in nums.iter().enumerate() {
        if start_amount + val > n {

            continue;

        } else if start_amount + val == n {

            let mut v = prev.to_vec();
            v.push(val);
            retval.push(v);

        } else if prev.len() < max_size - 1 {

            let mut accumul = prev.to_vec();
            accumul.push(val);
            let mut recurse = combinations_sum_n(&accumul, n, &nums[(i+1)..], max_size);
            retval.append(&mut recurse);
        }
    }
    return retval;
}

fn is_valid_four_way_split(target_weight: u64, combo: &Vec<u64>, present_weights: &Vec<u64>) -> bool {
    let filtered_weights: Vec<u64> = present_weights.iter().copied().filter(|&x| !combo.contains(&x)).collect();
    for i in 1..10 {
        let combos = get_combinations_summing_to_n(target_weight, filtered_weights.clone(), i);

        for combo in combos {
            if is_valid_three_way_split(target_weight, &combo, present_weights) {
                return true;
            }
        }
    }
    false
}

fn is_valid_three_way_split(target_weight: u64, combo: &Vec<u64>, present_weights: &Vec<u64>) -> bool {
    let filtered_weights: Vec<u64> = present_weights.iter().copied().filter(|&x| !combo.contains(&x)).collect();
    return can_make_total(&[], target_weight, &filtered_weights);
}

fn can_make_total(prev: &[u64], n: u64, nums: &[u64]) -> bool {
    let start_amount: u64 = prev.iter().sum();
    for (i, &val) in nums.iter().enumerate() {
        if start_amount + val > n {
            continue;
        } else if start_amount+val == n {
            return true;
        } else {
            let mut accumul = prev.to_vec();
            accumul.push(val);
            let recurse = can_make_total(&accumul, n, &nums[(i+1)..]);
            if recurse {
                return recurse;
            }
        }
    }
    false
}

fn target_weight_per_compartment(num_compartments: u64, present_weights: &Vec<u64>) -> u64 {
    let ttl: u64 = present_weights.iter().sum();
    assert!(ttl % num_compartments == 0);
    ttl / num_compartments
}

fn parse_input(input: Lines<BufReader<File>>) -> Vec<u64> {
    let mut present_weights: Vec<u64> = Vec::new();

    for line in input {
        let line = line.unwrap();
        present_weights.push(line.parse::<u64>().unwrap());
    }
    present_weights.sort();

    return present_weights;
}


#[cfg(test)]
mod tests {
    use super::*;

    const MAX_SIZE: usize = usize::MAX;

    #[test]
    fn test_combinations_sum_n_base_case() {
        let prev: Vec<u64> = Vec::new();
        let nums: Vec<u64> = vec![10];
        let target: u64 = 10;

        // Only one element to choose
        let result = combinations_sum_n(&prev, target, &nums, MAX_SIZE);
        assert_eq!(1, result.len());
        let res1 = result.get(0).unwrap().clone();
        assert_eq!(res1, vec![10]);

        // No good choices
        let target: u64 = 11;
        let result = combinations_sum_n(&prev, target, &nums, MAX_SIZE);
        assert_eq!(0, result.len());

        // Sum with previously accumulated nums
        let prev = vec![2];
        let target = 12;
        let result = combinations_sum_n(&prev, target, &nums, MAX_SIZE);
        assert_eq!(1, result.len());

        let res1 = result.get(0).unwrap().clone();
        assert_eq!(res1, vec![2, 10]);
    }
    
    #[test]
    fn test_combinations_sum_n_no_matches() {
        let prev: Vec<u64> = vec![];
        let nums: Vec<u64> = vec![200, 100, 50];
        let target = 10;

        let result = combinations_sum_n(&prev, target, &nums, MAX_SIZE);
        assert_eq!(0, result.len());
    }

    #[test]
    fn test_combinations_sum_n_recurse() {
        let prev: Vec<u64> = vec![];
        let nums: Vec<u64> = vec![12, 10, 7, 3, 2, 1];
        let target = 10;

        let result = combinations_sum_n(&prev, target, &nums, MAX_SIZE);

        let mut expecteds: Vec<Vec<u64>> = Vec::new();
        expecteds.push(vec![10]);
        expecteds.push(vec![7, 3]);
        expecteds.push(vec![7, 2, 1]);

        assert_eq!(expecteds.len(), result.len());
        for expected in expecteds  {
            assert!(result.contains(&expected), "expected {:?} to be in {:?}", expected, result);
        }
    }

    #[test]
    fn test_combinations_sum_n_max_size() {
        let prev: Vec<u64> = vec![];
        let nums: Vec<u64> = vec![12, 10, 6, 3, 2, 1];
        let target = 12;
        let max: usize = 2;

        let result = combinations_sum_n(&prev, target, &nums, max);

        let mut expecteds: Vec<Vec<u64>> = Vec::new();
        expecteds.push(vec![12]);
        expecteds.push(vec![10, 2]);

        assert_eq!(expecteds.len(), result.len());
        for expected in expecteds  {
            assert!(result.contains(&expected), "expected {:?} to be in {:?}", expected, result);
        }
    }

    #[test]
    fn test_can_make_total() {
        let nums: Vec<u64> = vec![12, 10, 6, 3, 2];

        assert!(can_make_total(&[], 12, &nums));
        assert!(can_make_total(&[], 18, &nums));
        assert!(can_make_total(&[], 21, &nums));
        assert!(!can_make_total(&[], 1000, &nums));
        assert!(!can_make_total(&[], 7, &nums));
    }

    #[test]
    fn test_is_valid_three_way_split() {
        let present_weights: Vec<u64> = vec![11, 10, 9, 8, 7, 5, 4, 3, 2, 1];

        let group1: Vec<u64> = vec![11, 9];
        assert!(is_valid_three_way_split(20, &group1, &present_weights));

        let group1: Vec<u64> = vec![10, 9, 1];
        assert!(is_valid_three_way_split(20, &group1, &present_weights));

        let group1: Vec<u64> = vec![9, 7, 4];
        assert!(is_valid_three_way_split(20, &group1, &present_weights));

        let group1: Vec<u64> = vec![8, 5, 4, 3];
        assert!(is_valid_three_way_split(20, &group1, &present_weights));
    }

    #[test]
    fn test_is_valid_three_way_split_false_case() {
        let present_weights: Vec<u64> = vec![14, 13, 12, 3, 2, 1];

        let group1: Vec<u64> = vec![14, 1];
        assert!(is_valid_three_way_split(15, &group1, &present_weights));

        let group1: Vec<u64> = vec![13, 2];
        assert!(is_valid_three_way_split(15, &group1, &present_weights));

        let group1: Vec<u64> = vec![12, 3];
        assert!(is_valid_three_way_split(15, &group1, &present_weights));

        let group1: Vec<u64> = vec![12, 2, 1];
        assert!(!is_valid_three_way_split(15, &group1, &present_weights));
    }
}
