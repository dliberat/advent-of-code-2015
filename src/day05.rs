use std::fs::File;
use std::io::{ Lines, BufReader };
use std::collections::HashMap;

pub(crate) fn solve(input: Lines<BufReader<File>>) {
    let mut nice_strings_part_1: u32 = 0;
    let mut nice_strings_part_2: u32 = 0;

    for line in input {
        let slice = &line.unwrap()[..];

        if is_nice_string_part_1(&slice) {
            nice_strings_part_1 += 1;
        }

        if is_nice_string_part_2(&slice) {
            nice_strings_part_2 += 1;
        }
    }

    println!("Part 1: Total number of nice strings: {}", nice_strings_part_1);
    println!("Part 2: Total number of nice strings: {}", nice_strings_part_2);
}

fn is_nice_string_part_1(s: &str) -> bool {
    let mut vowel_count = 0;
    let mut has_doubled_letter = false;

    let mut prev = ' ';

    for c in s.chars() {
        if c == prev {
            has_doubled_letter = true;
        }
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            vowel_count += 1;
        }
        if prev == 'a' && c == 'b' {
            return false;
        }
        if prev == 'c' && c == 'd' {
            return false;
        }
        if prev == 'p' && c == 'q' {
            return false;
        }
        if prev == 'x' && c == 'y' {
            return false;
        }

        prev = c;
    }

    vowel_count >= 3 && has_doubled_letter
}


fn is_nice_string_part_2(s: &str) -> bool {
    let mut p = ' ';
    let mut pp = ' ';

    let mut pairs: HashMap<String, usize> = HashMap::new();

    // contains a pair of letters that appears twice without overlapping
    let mut condition1 = false;

    // contains one letter repeated with a single letter between them
    let mut condition2 = false;

    for (i, c) in s.chars().enumerate() {

        if i > 0 {
            let pair = format!("{p}{c}");
            match pairs.get(&pair) {
                Some(location) => {
                    if *location < (i-1) {
                        condition1 = true;
                    }
                },
                None => {
                    pairs.insert(pair, i);
                },
            };
        }

        if pp == c {
            condition2 = true;
        }

        if condition1 && condition2 {
            return true;
        }

        pp = p;
        p = c;
    }

    false
}