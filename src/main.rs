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
        i32::MIN..=0_i32 | 2_i32..=i32::MAX => println!("Invalid input!"),
    }
}


fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap(); 
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines(); 
}
