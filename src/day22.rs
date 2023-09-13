use std::fs::File;
use std::io::{ Lines, BufReader };

#[derive(Debug)]
struct Boss {
    hit_points: u32,
    damage: u32,
}


pub(crate) fn solve(input: Lines<BufReader<File>>) {

    let boss = parse_input(input);
}


fn parse_input(input: Lines<BufReader<File>>) -> Boss {
    let mut hit_points = 0;
    let mut damage = 0;

    for line in input {
        let v = line.unwrap();
        let split: Vec<&str> = v.split(": ").collect();
        let key = split[0];
        let val = split[1].parse::<u32>().unwrap();

        match key {
            "Hit Points" => hit_points = val,
            "Damage" => damage = val,
            _ => panic!("Unexpected input: {}", key),
        }    
    }
    Boss{hit_points, damage}
}
