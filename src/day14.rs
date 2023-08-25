use std::collections::HashMap;
use std::fs::File;
use std::io::{ Lines, BufReader };

use itertools::Itertools;

const TIME_LIMIT: u32 = 2503;

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

impl Reindeer {
    fn distance_after_n_seconds(&self, n: u32) -> u32 {
        let mut total_distance = 0;

        let modulo = self.fly_time + self.rest_time;
        let distance_per_modulo = self.speed * self.fly_time;
        let factored = n / modulo;
        let remainder = n % modulo;

        total_distance += distance_per_modulo * factored;

        if remainder >= self.fly_time {
            total_distance += self.speed * self.fly_time
        } else {
            total_distance += self.speed * remainder;
        }

        total_distance
    }
}

pub(crate) fn solve(input: Lines<BufReader<File>>) {

    let mut reindeer: Vec<Reindeer> = Vec::new();

    for line in input {
        let r = parse_line(line.unwrap());
        reindeer.push(r);
    }

    let part_1 = part1(&reindeer);
    let part2 = part2(&reindeer, TIME_LIMIT);
    
    println!("Part 1: Distance traveled by winning reindeer: {}", part_1);
    println!("Part 2: Total points for winning reindeer: {}", part2);
}

fn part1(reindeer: &Vec<Reindeer>) -> u32 {
    let mut farthest_distance = 0;
    for r in reindeer {
        let d = r.distance_after_n_seconds(TIME_LIMIT);
        if d > farthest_distance {
            farthest_distance = d;
        }
    }
    farthest_distance
}

fn part2(reindeer: &Vec<Reindeer>, time_limit: u32) -> u32 {
    let mut points: HashMap<String, u32> = reindeer.iter().map(|r| (r.name.clone(), 0)).collect();
    
    for i in 1..(time_limit+1) {
        let scores: HashMap<String, u32> = reindeer
            .iter()
            .map(|r| (r.name.clone(), r.distance_after_n_seconds(i)))
            .collect();
        let top_score = *scores.values().max().unwrap();

        for (name, score) in scores {
            if score == top_score {
                let x = points.get_mut(&name).unwrap();
                *x += 1;
            }
        }

    }

    return *points.values().max().unwrap();
}

fn parse_line(line: String) -> Reindeer {
    let s = line.split(" ").collect_vec();
    let name = String::from(s[0]);
    let speed = s[3].parse::<u32>().unwrap();
    let fly_time = s[6].parse::<u32>().unwrap();
    let rest_time = s[13].parse::<u32>().unwrap();

    return Reindeer {name, speed, fly_time, rest_time}
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reindeer_distance_after_zero_seconds() {
        let r = Reindeer {
            name: String::from("name"),
            speed: 10,
            fly_time: 5,
            rest_time: 20,
        };

        let distance = r.distance_after_n_seconds(0);
        assert!(0 == distance);
    }

    #[test]
    fn reindeer_distance_after_one_second() {
        let r = Reindeer {
            name: String::from("name"),
            speed: 10,
            fly_time: 5,
            rest_time: 20,
        };

        let distance = r.distance_after_n_seconds(1);
        assert!(10 == distance);
    }

    #[test]
    fn reindeer_distance_after_exact_multiple() {
        let r = Reindeer {
            name: String::from("name"),
            speed: 10,
            fly_time: 5,
            rest_time: 20,
        };

        let distance = r.distance_after_n_seconds(25);
        assert!(50 == distance);
    }

    #[test]
    fn reindeer_distance_example_comet() {
        let comet = Reindeer {
            name: String::from("name"),
            speed: 14,
            fly_time: 10,
            rest_time: 127,
        };

        let distance = comet.distance_after_n_seconds(1);
        assert!(14 == distance);

        let distance = comet.distance_after_n_seconds(10);
        assert!(140 == distance);

        let distance = comet.distance_after_n_seconds(12);
        assert!(140 == distance);

        let distance = comet.distance_after_n_seconds(1000);
        assert!(1120 == distance);
    }

    #[test]
    fn reindeer_distance_example_dancer() {
        let dancer = Reindeer {
            name: String::from("name"),
            speed: 16,
            fly_time: 11,
            rest_time: 162,
        };

        let distance = dancer.distance_after_n_seconds(1);
        assert!(16 == distance);

        let distance = dancer.distance_after_n_seconds(10);
        assert!(160 == distance);

        let distance = dancer.distance_after_n_seconds(11);
        assert!(176 == distance);

        let distance = dancer.distance_after_n_seconds(1000);
        assert!(1056 == distance);
    }

    #[test]
    fn part2_winning_reindeer() {
        let comet = Reindeer {
            name: String::from("comet"),
            speed: 14,
            fly_time: 10,
            rest_time: 127,
        };

        let dancer = Reindeer {
            name: String::from("dancer"),
            speed: 16,
            fly_time: 11,
            rest_time: 162,
        };

        let time_limit = 1000;
        let reindeer: Vec<Reindeer> = vec![comet, dancer];
        let result = part2(&reindeer, time_limit);
        assert!(689 == result, "{} != 689", result);
    }
}
