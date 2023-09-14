use std::fs::File;
use std::io::{ Lines, BufReader };

pub(crate) fn solve(input: Lines<BufReader<File>>) {

    parse_input(input);


    let part1 = "";
    println!("Part 1: {}", part1);


    let part2 = "";
    println!("Part 2: {}", part2);
}



fn parse_input(input: Lines<BufReader<File>>) {

    for line in input {
        let line = line.unwrap();
        println!("{}", line);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
        
}
