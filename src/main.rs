extern crate lapp;
use std::fs::File;
use std::io::{ self, BufRead, BufReader };

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn main() {

    let args = lapp::parse_args("
Run solutions to Advent Of Code 2015.
  <day> (integer) Which day's challenge to solve
  <file> (string) The input file to use
    ");

    let d = args.get_integer("day");
    let f = args.get_string("file");
    println!("Solving day {} with input {}", d, f);

    let input = read_lines(f);

    match d {
        1 => day01::solve(input),
        2 => day02::solve(input),
        3 => day03::solve(input),
        4 => day04::solve(input),
        5 => day05::solve(input),
        6 => day06::solve(input),
        7 => day07::solve(input),
        8 => day08::solve(input),
        9 => day09::solve(input),
        10 => day10::solve(input),
        11 => day11::solve(input),
        12 => day12::solve(input),
        13 => day13::solve(input),
        14 => day14::solve(input),
        15 => day15::solve(input),
        16 => day16::solve(input),
        17 => day17::solve(input),
        18 => day18::solve(input),
        19 => day19::solve(input),
        20 => day20::solve(input),
        21 => day21::solve(input),
        22 => day22::solve(input),
        23 => day23::solve(input),
        24 => day24::solve(input),
        25 => day25::solve(input),
        i32::MIN..=0_i32 | 2_i32..=i32::MAX => println!("Invalid input!"),
    }
}


fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap(); 
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines(); 
}
