use std::fs::File;
use std::io::{ Lines, BufReader };


pub(crate) fn solve(input: Lines<BufReader<File>>) {

    let puzzle_input = parse_input(input);

    let part1 = part_1(puzzle_input);
    println!("Part 1: {}", part1);


    let part2 = part_2(puzzle_input);
    println!("Part 2: {}", part2);
}

fn part_2(puzzle_input: u32) -> u32 {
    let mut house = 1;
    let packages_per_elf = 11;
    loop {
        let mut count_of_presents = 0;

        let top = sqrt(house) + 1;
        for elf in 1..top {
            if house % elf == 0 {
                let d = house / elf;

                if elf*50 >= house {
                    count_of_presents += elf*packages_per_elf;
                }

                // check for d != elf to avoid double-counting perfect squares
                if d != elf && d*50 >= house {
                    count_of_presents += d*packages_per_elf;
                }
            }
        }


        if count_of_presents >= puzzle_input {
            return house;
        }

        house += 1;
    }
}


fn part_1(puzzle_input: u32) -> u32 {
    let mut house = 1;
    let packages_per_elf = 10;
    loop {
        let mut count_of_presents = 0;

        let top = sqrt(house) + 1;
        for elf in 1..top {
            if house % elf == 0 {
                let d = house / elf;
                count_of_presents += elf*packages_per_elf;

                // check for d != elf to avoid double-counting perfect squares
                if d != elf {
                    count_of_presents += d*packages_per_elf;
                }
            }
        }


        if count_of_presents >= puzzle_input {
            return house;
        }

        house += 1;
    }
}

fn sqrt(x: u32) -> u32 {
    (x as f32).sqrt().floor() as u32
}


fn parse_input(input: Lines<BufReader<File>>) -> u32 {

    for line in input {
        let v = line.unwrap();
        return v.parse::<u32>().unwrap();
    }

    return 0;
}
