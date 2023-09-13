use std::fs::File;
use std::io::{ Lines, BufReader };

#[derive(Debug)]
enum Register {
    A,
    B,
}

#[derive(Debug)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(i32),
    JumpIfEven(Register, i32),
    JumpIfOne(Register, i32),
}

#[derive(Debug)]
struct Computer {
    a: u32,
    b: u32,
    instr: i32,
}

impl Computer {
    fn new() -> Self {
        Computer{a: 0, b: 0, instr: 0}
    }

    fn execute(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Half(r) => self.half(r),
            Instruction::Triple(r) => self.triple(r),
            Instruction::Increment(r) => self.increment(r),
            Instruction::Jump(v) => self.jump(*v),
            Instruction::JumpIfEven(r, v) => self.jie(r, *v),
            Instruction::JumpIfOne(r, v) => self.jio(r, *v),
        };
    }

    fn half(&mut self, r: &Register) {
        match r {
            Register::A => self.a /= 2,
            Register::B => self.b /= 2,
        };
        self.instr += 1;
    }

    fn triple(&mut self, r: &Register) {
        match r {
            Register::A => self.a *= 3,
            Register::B => self.b *= 3,
        }
        self.instr += 1;
    }

    fn increment(&mut self, r: &Register) {
        match r {
            Register::A => self.a += 1,
            Register::B => self.b += 1,
        }
        self.instr += 1;
    }

    fn jump(&mut self, offset: i32) {
        self.instr += offset;
    }

    fn jie(&mut self, r: &Register, offset: i32) {
        let condition = match r {
            Register::A => self.a % 2 == 0,
            Register::B => self.b % 2 == 0,
        };
        if condition {
            self.instr += offset;
        } else {
            self.instr += 1;
        }
    }

    fn jio(&mut self, r: &Register, offset: i32) {
        let condition = match r {
            Register::A => self.a == 1,
            Register::B => self.b == 1,
        };
        if condition {
            self.instr += offset;
        } else {
            self.instr += 1;
        }
    }

}

const INSTR_HALF: &str = "hlf";
const INSTR_TRIPLE: &str = "tpl";
const INSTR_INCR: &str = "inc";
const INSTR_JMP: &str = "jmp";
const INSTR_JMP_EVEN: &str = "jie";
const INSTR_JMP_ODD: &str = "jio";

pub(crate) fn solve(input: Lines<BufReader<File>>) {

    let instructions = parse_input(input);


    let part1 = part_1(&instructions);
    println!("Part 1: Value of register b: {}", part1);


    let part2 = part_2(&instructions);
    println!("Part 2: Value of register b: {}", part2);
}


fn part_1(program: &Vec<Instruction>) -> u32 {
    let mut cpu = Computer::new();

    while 0 <= cpu.instr && cpu.instr < program.len() as i32 {
        let i = cpu.instr as usize;
        let instruction = program.get(i).unwrap();
        cpu.execute(instruction);

        // println!("Executed instr {} {:?}. {:?}", i, instruction, cpu);
    }

    cpu.b
}

fn part_2(program: &Vec<Instruction>) -> u32 {
    let mut cpu = Computer::new();
    cpu.a = 1;

    while 0 <= cpu.instr && cpu.instr < program.len() as i32 {
        let i = cpu.instr as usize;
        let instruction = program.get(i).unwrap();
        cpu.execute(instruction);

        // println!("Executed instr {} {:?}. {:?}", i, instruction, cpu);
    }

    cpu.b
}


fn parse_input<'b>(input: Lines<BufReader<File>>) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();

    for line in input {
        let v = line.unwrap().replace(",", "");
        let split: Vec<&str> = v.split(" ").collect();

        let instruction_name = split[0];

        let target_register = match split[1] {
            "a" => Register::A,
            _ => Register::B, // if it's not "a" or "b" then it's a jump instruction
        };

        match instruction_name {
            INSTR_HALF => program.push(Instruction::Half(target_register)),
            INSTR_TRIPLE => program.push(Instruction::Triple(target_register)),
            INSTR_INCR => program.push(Instruction::Increment(target_register)),
            INSTR_JMP => {
                let value = split[1].parse::<i32>().unwrap();
                program.push(Instruction::Jump(value));
            },
            INSTR_JMP_EVEN => {
                let value = split[2].parse::<i32>().unwrap();
                program.push(Instruction::JumpIfEven(target_register, value));
            },
            INSTR_JMP_ODD => {
                let value = split[2].parse::<i32>().unwrap();
                program.push(Instruction::JumpIfOne(target_register, value));
            },
            _ => panic!("Unexpected instruction name"),
        }
    }
    return program;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_program() {
        let mut program: Vec<Instruction> = Vec::new();
        program.push(Instruction::Increment(Register::A));
        program.push(Instruction::JumpIfOne(Register::A, 2));
        program.push(Instruction::Triple(Register::A));
        program.push(Instruction::Increment(Register::A));
        
        let mut cpu = Computer::new();

        cpu.execute(program.get(0).unwrap());
        assert_eq!(1, cpu.a);
        assert_eq!(0, cpu.b);
        assert_eq!(1, cpu.instr);

        cpu.execute(program.get(1).unwrap());
        assert_eq!(1, cpu.a);
        assert_eq!(0, cpu.b);
        assert_eq!(3, cpu.instr);

        cpu.execute(program.get(3).unwrap());
        assert_eq!(2, cpu.a);
        assert_eq!(0, cpu.b);
        assert_eq!(4, cpu.instr);
        
    }
}
