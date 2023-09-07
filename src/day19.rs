use std::collections::{HashMap, HashSet, BinaryHeap};
use std::fs::File;
use std::io::{ Lines, BufReader };

type ReplacementMap = HashMap<String, Vec<String>>;

pub(crate) fn solve(input: Lines<BufReader<File>>) {

    let (replacements, target_molecule) = parse_input(input);

    let part1 = part_1(&replacements, target_molecule.clone());
    println!("Part 1: Number of distinct molecules that can be created: {}", part1);


    let part2 = part_2(&replacements, target_molecule);
    println!("Part 2: Steps to create target molecule: {}", part2);
}

fn part_2(replacements_map: &ReplacementMap, target_molecule: String) -> u32 {

    // For any given element, there are multiple possibilities for what it can
    // be replaced into. However, the right hand side of each replacement operation
    // is unique. That means that figuring out which replacements can be done in
    // reverse order (i.e., starting at the target molecule and working our way back
    // down to the starting electron 'e', requires far less branching).
    // This approach also provides us with a very handy rule for calculating priorities:
    // The shorter a node is, the closer it is to just being 'e'
    let mut irm: ReplacementMap = HashMap::new();
    for (key, value) in replacements_map {
        for s in value {
            if irm.contains_key(s) {
                let v = irm.get_mut(s).unwrap();
                v.push(key.clone());
            } else {
                irm.insert(s.clone(), vec!(key.clone()));
            }
        }
    }


    return dijkstra(&irm, target_molecule, String::from("e"));
}

fn dijkstra(replacements_map: &ReplacementMap, start_node: String, target_node: String) -> u32 {
    if start_node == target_node {
        return 0;
    }

    let mut visited: HashSet<String> = HashSet::new();

    let mut queue: BinaryHeap<(i32, String, u32)> = BinaryHeap::new();
    // (priority, node, distance from start_node)
    queue.push((0, start_node.clone(), 0));

    while queue.len() > 0 {
        let (_, current_node, current_depth) = queue.pop().unwrap();

        let neighbors = get_neighbors(replacements_map, &current_node);
        let next_depth = current_depth + 1;
        for neighbor in neighbors {
            if neighbor == target_node {
                return next_depth;
            }
            if visited.contains(&neighbor) {
                // this node can be reached through a faster path
                continue;
            }
            visited.insert(neighbor.clone());

            // Negate the priority so smaller edit distances get higher priority
            let priority = -1 * (neighbor.len() as i32);
            queue.push((priority, neighbor, next_depth));
        }

    }
    
    panic!("No solution found starting from {} to reach {}", start_node, target_node);
}

fn part_1(replacements_map: &ReplacementMap, target_molecule: String) -> usize {
    // brute force.
    // For each possible element, find the index of every occurence in the molecule.
    // Then, apply each possible replacement at each index where it occurs.

    let results = get_neighbors(replacements_map, &target_molecule);    
    return results.len();
}

fn get_neighbors(replacements_map: &ReplacementMap, node: &String) -> HashSet<String> {
    let mut results: HashSet<String> = HashSet::new();

    for (key, replacements_list) in replacements_map {

        let key_length = key.len();
        let match_indices: Vec<_> = node.match_indices(key).map(|v| v.0).collect();
        for index in match_indices {
            for replacement in replacements_list {
                
                let rng = index..index+key_length;
                let mut replaced = node.clone();
                replaced.replace_range(rng, replacement);
                results.insert(replaced);
            }
        }
    }
    return results;
}

fn parse_input(input: Lines<BufReader<File>>) -> (ReplacementMap, String) {
    // Group the possible replacements by the element on the left-hand side.
    //
    // Example: If the list of replacements contains these possible replacements
    // H => HO
    // H => OH
    // group them as follows
    // H => [HO, OH]
    let mut replacements: ReplacementMap = HashMap::new();
    let mut target_molecule = String::new();

    for line in input {
        let v = line.unwrap();

        match v.contains(" => ") {
            true => {
                let split: Vec<&str> = v.split(" => ").collect();
                let lhs = String::from(split[0]);
                let rhs = String::from(split[1]);

                let v = replacements.entry(lhs).or_insert_with(|| Vec::new());
                v.push(rhs);
            },
            false => target_molecule = v,
        }
    }

    return (replacements, target_molecule);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_neighbors() {
        let h = String::from("H");
        let o = String::from("O");
        let e = String::from("e");
        let ho = String::from("HO");
        let oh = String::from("OH");
        let hh = String::from("HH");
        
        let mut replacements_map: ReplacementMap = HashMap::new();
        replacements_map.insert(e.clone(), vec!(h.clone(), o.clone()));
        replacements_map.insert(h.clone(), vec!(oh, ho));
        replacements_map.insert(o.clone(), vec!(hh.clone()));

        let results = get_neighbors(&replacements_map, &e);
        assert_eq!(2, results.len());
        assert!(results.contains(&o));
        assert!(results.contains(&h));

        let results = get_neighbors(&replacements_map, &o);
        assert_eq!(1, results.len());
        assert!(results.contains(&hh));

        // hh -> ohh hoh hoh hho , but the hoh is duplicated
        let ohh = String::from("HOH");
        let hoh = String::from("HOH");
        let hho = String::from("HHO");
        let results = get_neighbors(&replacements_map, &hh);
        assert_eq!(3, results.len());
        assert!(results.contains(&ohh));
        assert!(results.contains(&hoh));
        assert!(results.contains(&hho));
    }

    #[test]
    fn test_bfs() {
        let h = String::from("H");
        let o = String::from("O");
        let e = String::from("e");
        let ho = String::from("HO");
        let oh = String::from("OH");
        let hh = String::from("HH");
        
        let mut replacements_map: ReplacementMap = HashMap::new();
        replacements_map.insert(e.clone(), vec!(h.clone(), o.clone()));
        replacements_map.insert(h.clone(), vec!(oh, ho));
        replacements_map.insert(o.clone(), vec!(hh.clone()));

        let distance = dijkstra(&replacements_map, e.clone(), e.clone());
        assert_eq!(0, distance);

        let distance = dijkstra(&replacements_map, e.clone(), String::from("HOH"));
        assert_eq!(3, distance);

        let distance = dijkstra(&replacements_map, e.clone(), String::from("HOHOHO"));
        assert_eq!(6, distance);
    }
}
