use std::fs::File;
use std::io::{ Lines, BufReader };


pub(crate) fn solve(input: Lines<BufReader<File>>) {

    let mut part_1_total: usize = 0;
    let mut part_2_total: usize = 0;

    for line in input {
        let s = &line.unwrap();

        // Part 1: The difference between the "code size" of a string and
        // the "in-memory" size of a string is equivalent to the number of characters
        // that are removed when we convert the escape sequence to its real representation
        let escaped_chars = count_escaped_chars(s);
        part_1_total += escaped_chars;


        // Part 2: This could be counted in the same loop as part 1 in order to save cycles,
        // but I'm leaving it as separate for clarity.
        // Here, we just need to count how many characters in the string actually require an
        // escape backslash to be placed in front of them. The difference between the original
        // string "code size" and the new encoded size will simply be extra number of backslashes
        // required. We also need to +2 to this amount to account for the enclosing quotation marks.
        let chars_to_escape = count_chars_to_escape(s);
        part_2_total += chars_to_escape + 2;

    }

    println!("Part 1: Total code size minus in-memory size is: {}", part_1_total);
    println!("Part 2: Total re-encoded size minus original code size is: {}", part_2_total);

}


fn count_escaped_chars(s: &String) -> usize {
    let mut count = 2; // leading and trailing quotes

    let mut prev = ' ';
    for c in s.chars() {
        if prev == '\\' {
            if c == 'x' {
                count += 3;
            } else {
                count += 1;
            }

            // Hack for handling the case where there is an escaped backslash.
            prev = ' '

        } else {

            prev = c;

        }
    }

    count
}

fn count_chars_to_escape(s: &String) -> usize {
    let mut count: usize = 0;

    // only backslashes and quotation marks require escapes
    for c in s.chars() {
        if c == '\\' || c == '"' {
            count += 1;
        }
    }

    count
}
