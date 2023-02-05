use std::fs::File;
use std::io::{ Lines, BufReader };

pub(crate) fn solve(input: Lines<BufReader<File>>) {
    let mut instructions = String::new();

    for line in input {
        instructions += &line.unwrap();
    }

    let instructions = instructions.replace("\n", "");

    let mut floor = 0;
    for c in instructions.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
    }

    println!("Part 1: Santa will end up at floor {}", floor);

    floor = 0;
    for (i, c) in instructions.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }

        if floor == -1 {
            println!("Part 2: Santa will enter the basement at position: {}", i+1);
            break;
        }
    }
}
