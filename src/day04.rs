use std::fs::File;
use std::io::{ Lines, BufReader };
use md5;

pub(crate) fn solve(input: Lines<BufReader<File>>) {
    let mut secret_key = String::new();

    for line in input {
        secret_key += &line.unwrap();
    }

    let secret_key = secret_key.replace("\n", "");

    let mut i = 0;
    loop {
        if hash_starts_with_five_zeroes(&secret_key, i) {
            println!("Part 1: The lowest number to yield a good hash is: {i}");
            break;
        }
        i += 1;
    }


    i = 0;
    loop {
        if hash_starts_with_six_zeroes(&secret_key, i) {
            println!("Part 2: The lowest number to yield a good hash is: {i}");
            break;
        }
        i += 1;
    }
}

fn hash_starts_with_five_zeroes(secret_key: &String, num: u32) -> bool {
    let input = format!("{secret_key}{num}");
    let digest = md5::compute(input.as_bytes());
    let digest = &digest[..3];
    return digest[0] == 0 && digest[1] == 0 && digest[2] <= 16;
}

fn hash_starts_with_six_zeroes(secret_key: &String, num: u32) -> bool {
    let input = format!("{secret_key}{num}");
    let digest = md5::compute(input.as_bytes());
    let digest = &digest[..3];
    return digest[0] == 0 && digest[1] == 0 && digest[2] == 0;
}

