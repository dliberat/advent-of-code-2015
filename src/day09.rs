use std::fs::File;
use std::io::{ Lines, BufReader };
use std::collections::HashMap;
use itertools::{self, Itertools};


pub(crate) fn solve(input: Lines<BufReader<File>>) {
    // Traveling salesman. Solved with brute force.

    let mut cities: HashMap<String, HashMap<String, u32>> = HashMap::new();

    for line in input {
        let s = &line.unwrap();

        // This replacement makes it easier to split the string
        let s = s.replace(" to ", " = ");
        let split: Vec<&str> = s.split(" = ").collect();

        let start_city = String::from(*split.get(0).unwrap());
        let end_city = String::from(*split.get(1).unwrap());
        let distance = String::from(*split.get(2).unwrap());
        let distance = distance.parse::<u32>().unwrap();

        // routes are bidirectional, according to the example in the problem statement
        let routes = cities.entry(start_city.clone()).or_insert(HashMap::new());
        routes.insert(end_city.clone(), distance);
        
        let routes = cities.entry(end_city).or_insert(HashMap::new());
        routes.insert(start_city, distance);

    }
    
    // Check that the graph is fully connected. If this is true, we can simplify routing
    // because we don't need to check neighbors for each city.
    let total_cities = cities.len();
    for (_, routes) in &cities {
        let outvertex_count = routes.len();
        assert!(outvertex_count == total_cities-1);
    }

    // Each permutation is a potential route through all the cities. 
    let keys = cities.keys().collect_vec();
    let perms = keys.iter().permutations(keys.len());

    let mut shortest_route = u32::MAX;
    let mut longest_route: u32 = 0;

    for route in perms {
        let current_route = length_of_route(route, &cities);
        if current_route < shortest_route {
            shortest_route = current_route;
        }
        if current_route > longest_route {
            longest_route = current_route;
        }
    }

    println!("Part 1: Distance of shortest route: {}", shortest_route);
    println!("Part 2: Distance of the longest route: {}", longest_route);

}

fn length_of_route(mut route: Vec<&&String>, distances: &HashMap<String, HashMap<String, u32>>) -> u32 {
    let mut total_distance: u32 = 0;
    let mut prev_node = route.pop().unwrap();

    for node in route.into_iter().rev() {
        let distance_traveled = distances.get(*prev_node).unwrap().get(*node).unwrap();
        
        total_distance += distance_traveled;
        
        // This optimization works for the shortest path, but not for the longest
        // if total_distance >= best_so_far {
        //     return u32::MAX;
        // }
        
        prev_node = node;
    }
    
    return total_distance;
}