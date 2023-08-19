use std::fs::File;
use std::io::{ Lines, BufReader };

#[derive(Debug)]
struct NumberCount {
    value: char,
    count: u32,
}

pub(crate) fn solve(input: Lines<BufReader<File>>) {
    // count and say sequence

    let input_str = input.last().unwrap().unwrap();
  
    let part_1 = length_after_n_iterations(input_str.clone(), 40);
    let part_2 = length_after_n_iterations(input_str.clone(), 50);


    println!("Part 1: Length after 40 iterations: {}", part_1);
    println!("Part 2: Length after 50 iterations: {}", part_2);

}

fn length_after_n_iterations(s: String, n: u32) -> usize {
    let mut result = s;
    
    for _ in 0..n {
        result = iterate(result);
    }

    return result.len();
}

fn iterate(s: String) -> String {
    let mut num_counts = Vec::<NumberCount>::new();
    let mut current_char = ' ';
    let mut current_count = 0;

    for (i, c) in s.chars().enumerate() {
        if c == current_char {
            current_count += 1;
        } else {
            if i > 0 {
                let count = NumberCount {
                    value: current_char,
                    count: current_count,
                };
                num_counts.push(count);
            }

            current_count = 1;
            current_char = c;
        }
    }

    // Need to handle the last group of numbers in the input string
    let count = NumberCount {
        value: current_char,
        count: current_count,
    };
    num_counts.push(count);


    let mut result = String::new();
    for num_count in num_counts {
        result.push_str(num_count.count.to_string().as_str());
        result.push(num_count.value);
    }

    return result;
}
