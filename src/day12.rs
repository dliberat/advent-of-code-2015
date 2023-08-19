use std::fs::File;
use std::io::{ Lines, BufReader };
use serde_json::Value; // https://github.com/serde-rs/json

pub(crate) fn solve(input: Lines<BufReader<File>>) {
    // Parsing JSON

    let input_str = input.last().unwrap().unwrap();

    let v: Value = serde_json::from_str(input_str.as_str()).unwrap();
    let part_1 = sum(&v);

    let part_2 = sum_ignore_reds(&v);


    println!("Part 1: Sum of all numbers in JSON: {}", part_1);
    println!("Part 2: Sum of all numbers ignoring objects with 'red': {}", part_2);

}

fn sum(v: &Value) -> i64 {
    if v.is_number() {
        return v.as_i64().unwrap();
    }
    if v.is_array() {
        let array = v.as_array().unwrap();
        let mut s = 0;
        for vi in array {
            s += sum(vi);
        }
        return s;
    }
    if v.is_object() {
        let obj = v.as_object().unwrap();
        let mut s = 0;
        for vi in obj.values() {
            s += sum(vi);
        }
        return s;
    }

    return 0;
}


fn sum_ignore_reds(v: &Value) -> i64 {
    if v.is_number() {
        return v.as_i64().unwrap();
    }
    if v.is_array() {
        let array = v.as_array().unwrap();
        let mut s = 0;
        for vi in array {
            s += sum_ignore_reds(vi);
        }
        return s;
    }
    if v.is_object() {
        let obj = v.as_object().unwrap();
        let mut s = 0;
        for vi in obj.values() {
            if vi.is_string() {
                let str_val = vi.as_str().unwrap();
                if str_val == "red" {
                    return 0;
                }
            }
            s += sum_ignore_reds(vi);
        }
        return s;
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_empty_json() {
        validate_sum(r#"{}"#, 0);
        validate_sum(r#"[]"#, 0);
    }

    #[test]
    fn test_sum_arrays_and_objects() {
        validate_sum(r#"{"a":2,"b":4}"#, 6);
        validate_sum(r#"[1,2,3]"#, 6);
    }
    
    #[test]
    fn test_sum_negative_numbers() {
        validate_sum(r#"{"a":{"b":4},"c":-1}"#, 3);
        validate_sum(r#"[[[3]]]"#, 3);
    }
    
    #[test]
    fn test_sum_mixed_data_types() {
        validate_sum(r#"{"a":[-1,1]}"#, 0);
        validate_sum(r#"[-1,{"a":1}]"#, 0);
    }

    fn validate_sum(input: &str, expected: i64) {
        let v: Value = serde_json::from_str(input).unwrap();
        let actual = sum(&v);
        assert!(expected == actual);
    }

    #[test]
    fn test_sum_ignore_reds() {
        validate_sum_ignore_reds(r#"{"a":2,"b":4}"#, 6);
        validate_sum_ignore_reds(r#"[1,2,3]"#, 6);
        validate_sum_ignore_reds(r#"[1,{"c":"red","b":2},3]"#, 4);
        validate_sum_ignore_reds(r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0);
        validate_sum_ignore_reds(r#"[1,"red",5]"#, 6);
    }

    fn validate_sum_ignore_reds(input: &str, expected: i64) {
        let v: Value = serde_json::from_str(input).unwrap();
        let actual = sum_ignore_reds(&v);
        assert!(expected == actual, "input={}, expected={}, actual={}", input, expected, actual);
    }
}
