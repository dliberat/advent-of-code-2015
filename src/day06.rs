use lazy_static::lazy_static;
use std::fmt;
use std::fs::File;
use std::io::{ Lines, BufReader };
use regex::Regex;

#[derive(Debug)] 
enum Operation {
    TurnOn,
    TurnOff,
    Toggle,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct Instruction {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
    operation: Operation,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {},{} -> {},{}", self.operation, self.start_x, self.start_y, self.end_x, self.end_y)
    }
}

impl Instruction {
    fn from_text(s: &String) -> Self {
        let mut start_x = 0;
        let mut start_y = 0;
        let mut end_x = 0;
        let mut end_y = 0;
        let mut operation = Operation::Toggle;

        if s.starts_with("turn on") {
            operation = Operation::TurnOn;
        } else if s.starts_with("turn off") {
            operation = Operation::TurnOff;
        }

        lazy_static! {
            static ref RE: Regex = Regex::new("(\\d+)").unwrap();
        }

        for (i, cap) in RE.captures_iter(s).enumerate() {
            match i {
                0 => start_x = (&cap[1]).parse().unwrap(),
                1 => start_y = (&cap[1]).parse().unwrap(),
                2 => end_x = (&cap[1]).parse().unwrap(),
                3 => end_y = (&cap[1]).parse().unwrap(),
                _ => panic!(),
            }
        }

        return Instruction {
            start_x: start_x,
            start_y: start_y,
            end_x: end_x,
            end_y: end_y,
            operation: operation,
        }
    }
}

pub(crate) fn solve(input: Lines<BufReader<File>>) {

    let mut part_1_lights: [[bool; 1000]; 1000] = [[false; 1000]; 1000];
    let mut part_2_lights: [[u32; 1000]; 1000] = [[0; 1000]; 1000];


    for line in input {
        let instr = Instruction::from_text(&line.unwrap());
        update_part_1_lights(&mut part_1_lights, &instr);
        update_part_2_lights(&mut part_2_lights, &instr);
    }

    println!("Part 1: Total number of lit lights: {}", count_lights(&part_1_lights));
    println!("Part 2: Total brightness: {}", calculate_brightness(&part_2_lights));
}

fn count_lights(lights: &[[bool; 1000]; 1000]) -> u32 {
    let mut count = 0;

    for y in 0..1000 {
        for x in 0..1000 {
            if lights[y][x] {
                count += 1;
            }
        }
    }

    return count;
}

fn calculate_brightness(lights: &[[u32; 1000]; 1000]) -> u32 {
    let mut brightness = 0;

    for y in 0..1000 {
        for x in 0..1000 {
            brightness += lights[y][x]
        }
    }

    return brightness;
}

fn update_part_1_lights(lights: &mut [[bool; 1000]; 1000], instr: &Instruction) {

    match instr.operation {
        Operation::TurnOn => {
            for y in instr.start_y..instr.end_y+1 {
                for x in instr.start_x..instr.end_x+1 {
                    lights[y][x] = true;
                }
            }
        },
        Operation::TurnOff => {
            for y in instr.start_y..instr.end_y+1 {
                for x in instr.start_x..instr.end_x+1 {
                    lights[y][x] = false;
                }
            }
        },
        Operation::Toggle => {
            for y in instr.start_y..instr.end_y+1 {
                for x in instr.start_x..instr.end_x+1 {
                    lights[y][x] = !lights[y][x];
                }
            }
        },
    }
}

fn update_part_2_lights(lights: &mut [[u32; 1000]; 1000], instr: &Instruction) {

    match instr.operation {
        Operation::TurnOn => {
            for y in instr.start_y..instr.end_y+1 {
                for x in instr.start_x..instr.end_x+1 {
                    lights[y][x] += 1;
                }
            }
        },
        Operation::TurnOff => {
            for y in instr.start_y..instr.end_y+1 {
                for x in instr.start_x..instr.end_x+1 {
                    if lights[y][x] > 0 {
                        lights[y][x] -= 1;
                    }
                }
            }
        },
        Operation::Toggle => {
            for y in instr.start_y..instr.end_y+1 {
                for x in instr.start_x..instr.end_x+1 {
                    lights[y][x] += 2;
                }
            }
        },
    }
}
