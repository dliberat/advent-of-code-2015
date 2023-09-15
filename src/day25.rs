use std::fs::File;
use std::io::{ Lines, BufReader };

use regex::Regex;

const START_CODE: u64 = 20151125;
const F: u64 = 252533;
const MOD: u64 = 33554393;

pub(crate) fn solve(input: Lines<BufReader<File>>) {

    let (row, col) = parse_input(input);


    let part1 = part_1(row, col);
    println!("Part 1: Input code: {}", part1);


    let part2 = "";
    println!("Part 2: {}", part2);
}

fn part_1(row: u64, col: u64) -> u64 {
    let seq_number = convert_row_col_to_sequence_num(row, col);

    let mut code = START_CODE;

    for _ in 1..seq_number {
        code = (code*F) % MOD;
    }
    code
}

fn convert_row_col_to_sequence_num(row: u64, col: u64) -> u64 {
    // There's probably a closed form mathematical expression for this,
    // but it's quick enough to figure out with a bit of iteration for now.

    assert!(row > 0);
    assert!(col > 0);

    // row 1 of any given column is equal to the
    // sum of all natural numbers up until the column num
    let top_of_col = (col * (col+1))/2;
    let mut seq_number = top_of_col;

    for i in 0..row-1 {
        seq_number += col + (i as u64);
    }
    return seq_number;
}

fn parse_input(input: Lines<BufReader<File>>) -> (u64, u64) {

    let re = Regex::new(r"row (?P<row>\d+), column (?P<col>\d+)").unwrap();

    for line in input {
        let line = line.unwrap();

        let Some(caps) = re.captures(&line) else { panic!("Unexpected input"); };
        let row = &caps["row"];
        let row = row.parse::<u64>().unwrap();

        let col = &caps["col"];
        let col = col.parse::<u64>().unwrap();

        return (row, col);
    }

    panic!("No input data");
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_convert_row_col_to_sequence_num() {
        let mut expecteds: HashMap<(u64, u64), u64> = HashMap::new();
        expecteds.insert((1, 1), 1);
        expecteds.insert((2, 1), 2);
        expecteds.insert((1, 2), 3);
        expecteds.insert((3, 1), 4);
        expecteds.insert((2, 2), 5);
        expecteds.insert((1, 3), 6);
        expecteds.insert((4, 1), 7);
        expecteds.insert((3, 2), 8);
        expecteds.insert((2, 3), 9);
        expecteds.insert((1, 4), 10);
        expecteds.insert((5, 1), 11);
        expecteds.insert((4, 2), 12);
        expecteds.insert((3, 3), 13);
        expecteds.insert((2, 4), 14);
        expecteds.insert((1, 5), 15);
        expecteds.insert((6, 1), 16);
        expecteds.insert((5, 2), 17);
        expecteds.insert((4, 3), 18);
        expecteds.insert((3, 4), 19);
        expecteds.insert((2, 5), 20);

        for ((row, col), expected) in expecteds {
            let actual = convert_row_col_to_sequence_num(row, col);
            assert_eq!(expected, actual, "expected ({}, {}) -> {} but was {}", row, col, expected, actual);
        }
    }

    #[test]
    fn test_part_1() {
        let mut expecteds: HashMap<(u64, u64), u64> = HashMap::new();
        expecteds.insert((1, 1), 20151125);
        expecteds.insert((2, 1), 31916031);
        expecteds.insert((1, 2), 18749137);
        expecteds.insert((3, 1), 16080970);
        expecteds.insert((2, 2), 21629792);
        expecteds.insert((1, 3), 17289845);
        expecteds.insert((4, 1), 24592653);
        expecteds.insert((3, 2), 8057251);
        expecteds.insert((2, 3), 16929656);
        expecteds.insert((1, 4), 30943339);
        expecteds.insert((5, 1), 77061);
        expecteds.insert((4, 2), 32451966);
        expecteds.insert((3, 3), 1601130);
        expecteds.insert((2, 4), 7726640);
        expecteds.insert((1, 5), 10071777);
        expecteds.insert((6, 1), 33071741);
        expecteds.insert((5, 2), 17552253);
        expecteds.insert((4, 3), 21345942);
        expecteds.insert((3, 4), 7981243);
        expecteds.insert((2, 5), 15514188);

        for ((row, col), expected) in expecteds {
            let actual = part_1(row, col);
            assert_eq!(expected, actual, "expected ({}, {}) -> {} but was {}", row, col, expected, actual);
        }
    }
        
}
