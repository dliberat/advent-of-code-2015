use std::cmp::min;
use std::fs::File;
use std::io::{ Lines, BufReader };

struct Box {
    width: u32,
    height: u32,
    depth: u32,
}

impl Box {
    fn wrapping_paper_required(&self) -> u32 {
        let a = self.width * self.height;
        let b = self.width * self.depth;
        let c = self.height * self.depth;
        
        return 2*a + 2*b + 2*c + min(min(a, b), c);
    }

    fn ribbon_required(&self) -> u32 {
        let a = 2*self.width + 2*self.height;
        let b = 2*self.width + 2*self.depth;
        let c = 2*self.height + 2*self.depth;

        let box_ribbon = min(a, min(b, c));
        let volume = self.width * self.height * self.depth;

        return box_ribbon + volume;
    }

    fn from_dimensions(dimensions: String) -> Self {
        let dims = dimensions.split("x");
        let dims: Vec<u32> = dims.map(|x| x.parse::<u32>().unwrap()).collect();
        return Box {
            width: dims[0],
            height: dims[1],
            depth: dims[2],
        }
    }
}

pub(crate) fn solve(input: Lines<BufReader<File>>) {
    let mut total_wrapping_paper_size = 0;
    let mut total_ribbon = 0;

    for line in input {
        let b = Box::from_dimensions(line.unwrap());
        total_wrapping_paper_size += b.wrapping_paper_required();
        total_ribbon += b.ribbon_required();
    }

    println!("Part 1: Wrapping paper required: {}", total_wrapping_paper_size);
    println!("Part 2; Ribbon required: {}", total_ribbon);

}
