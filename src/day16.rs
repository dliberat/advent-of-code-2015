use std::collections::HashMap;
use std::fs::File;
use std::io::{ Lines, BufReader };

use itertools::Itertools;


pub(crate) fn solve(input: Lines<BufReader<File>>) {
    let mut message: HashMap<String, u32> = HashMap::new();
    message.insert(String::from("children"), 3);
    message.insert(String::from("cats"), 7);
    message.insert(String::from("samoyeds"), 2);
    message.insert(String::from("pomeranians"), 3);
    message.insert(String::from("akitas"), 0);
    message.insert(String::from("vizslas"), 0);
    message.insert(String::from("goldfish"), 5);
    message.insert(String::from("trees"), 3);
    message.insert(String::from("cars"), 2);
    message.insert(String::from("perfumes"), 1);
    
    let mut sue_map: HashMap<u32, HashMap<String, u32>> = HashMap::new();

    for line in input {
        let (num, data) = parse_line(line.unwrap());
        sue_map.insert(num, data);
    }

    let part1 = part_1(&sue_map, &message);
    let part2 = part_2(sue_map, &message);

    println!("Part 1: The only Sue that matches is: Sue {}", part1);
    println!("Part 2: The only Sue that matches is: Sue {}", part2);
}

fn part_1(sue_map: &HashMap<u32, HashMap<String, u32>>, message: &HashMap<String, u32>) -> u32 {
    for (sue_num, posessions_list) in sue_map {
        let mut is_match = true;
        for (item, count) in posessions_list {
            if message.contains_key(item) {
                let message_value = *message.get(item).unwrap();
                if message_value != *count {
                    is_match = false;
                    break;
                }
            }
        }
        if is_match {
            return *sue_num;
        }
    }
    return 0;
}

fn part_2(sue_map: HashMap<u32, HashMap<String, u32>>, message: &HashMap<String, u32>) -> u32 {
    for (sue_num, posessions_list) in sue_map {
        let mut is_match = true;

        for (item, count) in posessions_list {
            if message.contains_key(&item) {
                let message_value = *message.get(&item).unwrap();

                if item == String::from("cats") || item == String::from("trees") {
                    if message_value >= count {
                        is_match = false;
                        break;
                    }

                } else if item == String::from("pomeranians") || item == String::from("goldfish") {
                    if message_value <= count {
                        is_match = false;
                        break;
                    }

                } else {
                    if message_value != count {
                        is_match = false;
                        break;
                    }
                }
            }
        }
        if is_match {
            return sue_num;
        }
    }
    return 0;
}

fn parse_line(line: String) -> (u32, HashMap<String, u32>) {
    // get the number of the sue
    let line = line.replace(":", "");
    let line = line.replace(",", "");

    let s = line.split(" ").collect_vec();
    let num = s[1].parse::<u32>().unwrap();

    let mut posessions: HashMap<String, u32> = HashMap::new();
    for i in (2..s.len()).step_by(2) {
        let key = String::from(s[i]);
        let val = s[i+1].parse::<u32>().unwrap();
        posessions.insert(key, val);
    }

    return (num, posessions);
}




// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_parse() {
//         let line = String::from("Sprinkles: capacity 5, durability -1, flavor 0, texture 0, calories 5");
//         let actual = parse_line(line);
//         let expected = Ingredient {
//             name: String::from("Sprinkles"),
//             capacity: 5,
//             durability: -1,
//             flavor: 0,
//             texture: 0,
//             calories: 5,
//         };
//         assert!(expected == actual);
//     }
// }
