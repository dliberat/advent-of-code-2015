use std::fs::File;
use std::io::{ Lines, BufReader };

// ascii code points
const A: u8 = 97;
const I: u8 = 105;
const L: u8 = 108;
const O: u8 = 111;
const Z: u8 = 122;

pub(crate) fn solve(input: Lines<BufReader<File>>) {
    // all passwords have exactly 8 characters
    let mut password: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

    for line in input {
        password = convert_from_string(line.unwrap());
        break; // input should only have 1 line
    }
    
    let next_password = iterate(password);
    println!("Part 1: Next password is {}", convert_to_string(next_password));

    let next_password = iterate(next_password);
    println!("Part 2: Next password is {}", convert_to_string(next_password));

}

fn iterate(mut current_password: [u8; 8]) -> [u8; 8] {
    // Brute force solution. Increment the current index until it reaches Z.
    // Once a character reaches Z, it needs to be cycle around to A, we increment
    // the next index, and then start incrementing again from the first index.
    //
    // For some example inputs (e.g., "ghijklmn") this is horribly inefficient,
    // since it will blindly try to iterate every single character, oblivious to
    // the fact that all iterations will fail until the illegal "i" character gets
    // incremented.
    //
    // However, for the actual puzzle inputs this performs fine.
    
    let mut current_index: usize = 0;

    while current_index < 8 {

        let next_value = next_character_at_index(&current_password, current_index);
        current_password[current_index] = next_value;

        
        match next_value {
            A => {
                current_index += 1;
                continue;
            },
            _ => current_index = 0,
        }

        // println!("Validating {}", convert_to_string(current_password));
        if validate(&current_password) {
            return current_password;
        }
    }

    panic!("Did not find any valid passwords!");
}

fn next_character_at_index(pw: &[u8; 8], index: usize) -> u8 {
    let mut val = pw[index];
    val += 1;

    // passwords may not contain these letters
    if val == I || val == L || val == O {
        val += 1;
    } else if val > Z {
        val = A;
    }
    val
}

fn convert_to_string(password: [u8; 8]) -> String {
    let mut password = password.clone();
    password.reverse();
    std::str::from_utf8(&password).expect("invalid utf-8 sequence").to_string()
}

fn validate(password: &[u8; 8]) -> bool {
    let mut contains_straight_run = false;
    let mut contains_two_non_overlapping_pairs = false;
    let mut head_of_first_non_overlapping_pair = 10; // anything greater than 8 is the same as infinity

    for (i, e) in password.iter().enumerate() {
        // We perform this check during the iteration process so that we can avoid
        // unnecessary calls to this function, but having it here allows us to also 
        // accept passwords with bad characters as inputs.
        if *e == I || *e == L || *e == O {
            return false;
        }

        // in the password array, characters are in reverse order relative to how
        // it is displayed in string format, so to find an "increasing straight of 
        // at least three letters", we need to find three consecutively decreasing
        // ascii code points
        if !contains_straight_run && i > 1 {
            contains_straight_run = password[i-2] == e+2 && password[i-1] == e+1
        }

        if !contains_two_non_overlapping_pairs && i > 0 {

            if password[i-1] == *e {
                // we have a candidate pair.

                if head_of_first_non_overlapping_pair > 7 {
                    // if we haven't found any pairs yet, use this as the first one
                    head_of_first_non_overlapping_pair = i-1;

                } else if head_of_first_non_overlapping_pair < i - 2 {
    
                    // If the first non overlapping pair was found exactly two places ago,
                    // that means that we're looking at a run of three identical letters
                    //    a b c c c d e f
                    //    0 1 2 3 4 5 6 7
                    // 
                    // e.g., if we are currently at i = 4, there's a candidate pair,
                    // but it overlaps with the pair at 2-3

                    contains_two_non_overlapping_pairs = true;
                }
            }
        }
    }

    return contains_straight_run && contains_two_non_overlapping_pairs;
}

fn convert_from_string(s: String) -> [u8; 8] {
    
    let current_password = s.as_bytes();
    assert!(current_password.len() == 8);
    
    let mut password: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let mut j = password.len() - 1;
    for char in current_password {
        password[j] = *char;
        
        if j > 0 { // need the check here to avoid overflow after the last char
            j -= 1;
        }
    }
    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_happy_path() {
        let pw = convert_from_string(String::from("aabccyte"));
        assert!(validate(&pw));

        let pw = convert_from_string(String::from("abcdzzhh"));
        assert!(validate(&pw));

        let pw = convert_from_string(String::from("abcdffaa"));
        assert!(validate(&pw));

        let pw = convert_from_string(String::from("ghjaabcc"));
        assert!(validate(&pw));
    }

    #[test]
    fn validate_missing_increasing_run() {
        let pw = convert_from_string(String::from("ghjaatcc"));
        assert!(!validate(&pw));
    }

    #[test]
    fn validate_missing_double_letters() {
        let pw = convert_from_string(String::from("abcdfqaa"));
        assert!(!validate(&pw));
    }
}