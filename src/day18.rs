use std::fs::File;
use std::io::{ Lines, BufReader };
use std::fmt;

use itertools::Itertools;

const OFF: u32 = 0;
const ON: u32 = 1;

#[derive(Clone)]
struct Field {
    width: usize,
    height: usize,
    data: Vec<u32>,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rows: Vec<String> = Vec::new();

        for i in 0..self.height {
            let s = i*self.width;
            let e = i*self.width + self.width;
            let row = &self.data[s..e];
            let row: Vec<&str> = row.iter().map(|x| match *x {
                OFF => ".",
                ON => "#",
                _ => panic!("Unrecognized value in row"),
            }).collect();
            rows.push(row.join(""));
        }

        write!(f, "{}", rows.join("\n"))
    }
}

impl Field {
    fn sum_of_neighbors(&self, i: usize) -> u32 {
        let row = i / self.width;
        let col = i % self.width;

        let is_left_edge = col == 0;
        let is_right_edge = col == self.width-1;
        let is_top_edge = row == 0;
        let is_bottom_edge = row == self.height-1;

        let mut ttl = 0;

        // left
        if !is_left_edge {
            ttl += self.data[i-1];

            // upper left diagonal
            if !is_top_edge {
                ttl += self.data[i-1-self.width];
            }
            // lower left diagonal
            if !is_bottom_edge {
                ttl += self.data[i-1+self.width];
            }
        }
        // right
        if !is_right_edge {
            ttl += self.data[i+1];

            // upper right diagonal
            if !is_top_edge {
                ttl += self.data[i+1-self.width];
            }
            // bottom right diagonal
            if !is_bottom_edge {
                ttl += self.data[i+1+self.width];
            }
        }
        // above
        if !is_top_edge {
            ttl += self.data[i-self.width];
        }
        // below
        if !is_bottom_edge {
            ttl += self.data[i+self.width];
        }
        
        ttl
    }

    fn count_lights_on(&self) -> u32 {
        self.data.iter().sum()
    }

    fn iterate(&mut self) {
        let size = self.width*self.height;
        let sums_of_neighbors = (0..size).map(|x| self.sum_of_neighbors(x)).collect_vec();

        for i in 0..size {
            let sum_of_neighbors = sums_of_neighbors[i];
            let value = self.data[i];
            if value == ON {
                if sum_of_neighbors != 2 && sum_of_neighbors != 3 {
                    self.data[i] = OFF;
                }
            } else if value == OFF {
                if sum_of_neighbors == 3 {
                    self.data[i] = ON;
                }
            }
        }
    }

    fn fix_corners(&mut self) {
        self.data[0] = ON;
        self.data[self.width-1] = ON;

        let len = self.data.len();
        let bottom_left = len - self.width;
        self.data[bottom_left] = ON;

        self.data[len-1] = ON;
    }
}

pub(crate) fn solve(input: Lines<BufReader<File>>) {

    let mut field1 = parse_input(input);

    let mut field2 = field1.clone();
    field2.fix_corners();
    
    for _ in 0..100 {
        field1.iterate();

        field2.iterate();
        field2.fix_corners();
    }

    let part1 = field1.count_lights_on();
    let part2 = field2.count_lights_on();

    println!("Part 1: Total lights on after 100 iterations: {}", part1);
    println!("Part 2: Total lights on after 100 iterations: {}", part2);
}

fn parse_input(input: Lines<BufReader<File>>) -> Field {
    let mut width = 0;
    let mut height: usize = 0;
    let mut data: Vec<u32> = Vec::new();

    for line in input {
        let v = line.unwrap();

        if width == 0 {
            width = v.len();
        }
        height += 1;

        for c in v.chars() {
            match c {
                '#' => data.push(ON),
                '.' => data.push(OFF),
                _ => panic!("Unexpected char {}", c),
            }
        }
    }

    return Field {width, height, data}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_neighbors_empty_board() {
        let data = vec![
            0, 0, 0,
            0, 0, 0,
            0, 0, 0,
            0, 0, 0,
        ];
        let width = 3;
        let height = 4;
        let field = Field{width, height, data};
        for i in 0..(width*height) {
            assert_eq!(0, field.sum_of_neighbors(i));
        }
    }

    #[test]
    fn test_sum_of_neighbors_single_on() {
        let data = vec![
            1, 0, 0,
            0, 0, 0,
            0, 0, 0,
            0, 0, 0,
        ];
        let width = 3;
        let height = 4;
        let field = Field{width, height, data};

        let expected = vec![
            0, 1, 0,
            1, 1, 0,
            0, 0, 0,
            0, 0, 0,
        ];

        for i in 0..(width*height) {
            assert_eq!(expected[i], field.sum_of_neighbors(i));
        }
    }

    #[test]
    fn test_sum_of_neighbors_full_example() {
        let data = vec![
            1, 0, 0,
            0, 0, 1,
            0, 1, 0,
            0, 1, 0,
        ];
        let width = 3;
        let height = 4;
        let field = Field{width, height, data};

        let expected = vec![
            0, 2, 1,
            2, 3, 1,
            2, 2, 3,
            2, 1, 2,
        ];

        for i in 0..(width*height) {
            assert_eq!(expected[i], field.sum_of_neighbors(i));
        }
    }

    #[test]
    fn test_count_lights_on() {
        let data = vec![
            0, 1, 0,
            0, 0, 0,
            1, 0, 1,
            0, 1, 0,
        ];
        let width = 3;
        let height = 4;
        let field = Field{width, height, data};
        assert_eq!(4, field.count_lights_on());
    }

    #[test]
    fn test_example() {
        // example from the problem statement
        let data = vec![
            0, 1, 0, 1, 0, 1,
            0, 0, 0, 1, 1, 0,
            1, 0, 0, 0, 0, 1,
            0, 0, 1, 0, 0, 0,
            1, 0, 1, 0, 0, 1,
            1, 1, 1, 1, 0, 0,
        ];
        let width = 6;
        let height = 6;
        let mut field = Field{width, height, data};
        
        assert_eq!(15, field.count_lights_on());
        
        field.iterate();
        println!("{}", field);
        assert_eq!(11, field.count_lights_on());

        field.iterate();
        assert_eq!(8, field.count_lights_on());

        field.iterate();
        assert_eq!(4, field.count_lights_on());

        field.iterate();
        assert_eq!(4, field.count_lights_on());
    }

    #[test]
    fn test_example_part_2() {
        // example from the problem statement
        let data = vec![
            0, 1, 0, 1, 0, 1,
            0, 0, 0, 1, 1, 0,
            1, 0, 0, 0, 0, 1,
            0, 0, 1, 0, 0, 0,
            1, 0, 1, 0, 0, 1,
            1, 1, 1, 1, 0, 0,
        ];
        let width = 6;
        let height = 6;
        let mut field = Field{width, height, data};
        field.fix_corners();
        println!("Initial state: \n{}\n", field);
        
        assert_eq!(17, field.count_lights_on());
        
        field.iterate();
        field.fix_corners();
        println!("{}", field);
        assert_eq!(18, field.count_lights_on());

        field.iterate();
        field.fix_corners();
        assert_eq!(18, field.count_lights_on());

        field.iterate();
        field.fix_corners();
        assert_eq!(18, field.count_lights_on());

        field.iterate();
        field.fix_corners();
        assert_eq!(14, field.count_lights_on());

        field.iterate();
        field.fix_corners();
        assert_eq!(17, field.count_lights_on());
    }
}
