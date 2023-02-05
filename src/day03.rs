use std::collections::HashSet;
use std::fs::File;
use std::io::{ Lines, BufReader };

pub(crate) fn solve(input: Lines<BufReader<File>>) {
    
    let mut instructions = String::new();
    for line in input {
        instructions += &line.unwrap();
    }
    
    let instructions = instructions.replace("\n", "");
    

    // Part 1
    let mut locations = HashSet::new();
    let mut current_location = (0, 0);
    locations.insert(current_location);

    for c in instructions.chars() {
        match c {
            '<' => current_location = (current_location.0, current_location.1-1),
            '^' => current_location = (current_location.0-1, current_location.1),
            'v' => current_location = (current_location.0+1, current_location.1),
            '>' => current_location = (current_location.0, current_location.1+1),
            _ => {},
        }
        locations.insert(current_location);
    }

    println!("Part 1: Santa will visit {} unique houses", locations.len());

    // Part 2
    let mut locations = HashSet::new();
    let mut santa = (0, 0);
    let mut robot = (0, 0);
    locations.insert(santa);

    for (i, c) in instructions.chars().enumerate() {
        if i % 2 == 0 {
            match c {
                '<' => santa = (santa.0, santa.1-1),
                '^' => santa = (santa.0-1, santa.1),
                'v' => santa = (santa.0+1, santa.1),
                '>' => santa = (santa.0, santa.1+1),
                _ => {},
            }
            locations.insert(santa);

        } else {
            match c {
                '<' => robot = (robot.0, robot.1-1),
                '^' => robot = (robot.0-1, robot.1),
                'v' => robot = (robot.0+1, robot.1),
                '>' => robot = (robot.0, robot.1+1),
                _ => {},
            }
            locations.insert(robot);
        }
    }

    println!("Part 2: Santa and the robot will visit {} unique houses", locations.len());

}
