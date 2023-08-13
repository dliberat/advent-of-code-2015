use std::fmt;
use std::fs::File;
use std::io::{ Lines, BufReader };
use std::collections::HashMap;

#[derive(Debug)] 
enum LogicGate {
    AND,
    OR,
    NOT,
    LSHIFT,
    RSHIFT,
}

impl fmt::Display for LogicGate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


pub(crate) fn solve(input: Lines<BufReader<File>>) {

    let mut circuit = HashMap::new();
    let mut signals = HashMap::new();

    for line in input {
        read_instruction(line.unwrap(), &mut circuit);
    }

    let result = solve_for_wire(String::from("a"), &circuit, &mut signals);
    println!("Part 1: Final signal on wire a: {}", result);
}

fn solve_for_wire(wire: String, circuit: &HashMap<String, String>, signals: &mut HashMap<String, u16>) -> u16 {
    // Early exit: If the wire is not a wire, but a raw signal
    match wire.parse::<u16>() {
        Ok(v) => {
            return v;
        },
        Err(_) => {},
    }

    // Early exit: If the signal on this wire has already been worked out
    if signals.contains_key(&wire) {
        return *signals.get(&wire).unwrap();
    }

    let msg = format!("Circuit does not contain wire: '{}'", wire);
    let instruction = circuit.get(&wire).expect(&msg);
    
    // Early exit: If the signal on the wire is trivially assigned in the circuit specs
    match instruction.parse::<u16>() {
        Ok(v) => {
            signals.insert(wire, v);
            return v;
        },
        Err(_) => {},
    }
    
    // Hard work: solve a logic gate
    let value: u16;
    if instruction.contains("AND") {
        let split: Vec<&str> = instruction.split(" AND ").collect();
        let lhs = split.get(0).unwrap().to_string();
        let lhs = solve_for_wire(lhs, circuit, signals);

        let rhs = split.get(1).unwrap().to_string();
        let rhs = solve_for_wire(rhs, circuit, signals);
        value = apply_gate(lhs, rhs, LogicGate::AND);
        
    } else if instruction.contains("OR") {
        let split: Vec<&str> = instruction.split(" OR ").collect();
        let lhs = split.get(0).unwrap().to_string();
        let lhs = solve_for_wire(lhs, circuit, signals);

        let rhs = split.get(1).unwrap().to_string();
        let rhs = solve_for_wire(rhs, circuit, signals);
        value = apply_gate(lhs, rhs, LogicGate::OR);
        
    } else if instruction.contains("LSHIFT") {
        let split: Vec<&str> = instruction.split(" LSHIFT ").collect();
        let lhs = split.get(0).unwrap().to_string();
        let lhs = solve_for_wire(lhs, circuit, signals);

        let rhs = split.get(1).unwrap().to_string();
        let rhs = solve_for_wire(rhs, circuit, signals);
        value = apply_gate(lhs, rhs, LogicGate::LSHIFT);

    } else if instruction.contains("RSHIFT") {
        let split: Vec<&str> = instruction.split(" RSHIFT ").collect();
        let lhs = split.get(0).unwrap().to_string();
        let lhs = solve_for_wire(lhs, circuit, signals);

        let rhs = split.get(1).unwrap().to_string();
        let rhs = solve_for_wire(rhs, circuit, signals);
        value = apply_gate(lhs, rhs, LogicGate::RSHIFT);

    } else if instruction.contains("NOT") {
        let rhs = instruction[4..].to_string();
        let rhs = solve_for_wire(rhs, circuit, signals);
        value = apply_gate(rhs, 0, LogicGate::NOT);

    } else {
        // instruction is a single wire connected directly to the target wire
        value = solve_for_wire(instruction.to_string(), circuit, signals);
    }

    signals.insert(wire, value);
    return value;
}


fn read_instruction(s: String, circuit: &mut HashMap<String, String>) {
    let split: Vec<&str> = s.split(" -> ").collect();
    let lhs = *split.get(0).unwrap();
    let rhs = *split.get(1).unwrap();
    circuit.insert(String::from(rhs), String::from(lhs));
}

fn apply_gate(a: u16, b: u16, gate: LogicGate) -> u16 {
    match gate {
        LogicGate::AND => a & b,
        LogicGate::OR => a | b,
        LogicGate::NOT => !a,
        LogicGate::LSHIFT => a << b,
        LogicGate::RSHIFT => a >> b,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_or() {
        let gate = LogicGate::OR;
        let x: u16 = 123;
        let y: u16 = 456;
        let expected: u16 = 507;
        let actual = apply_gate(x, y, gate);

        assert_eq!(actual, expected);
    }

    #[test]
    fn apply_and() {
        let gate = LogicGate::AND;
        let x: u16 = 123;
        let y: u16 = 456;
        let expected: u16 = 72;
        let actual = apply_gate(x, y, gate);

        assert_eq!(actual, expected);
    }

    #[test]
    fn apply_lshift() {
        let gate = LogicGate::LSHIFT;
        let x: u16 = 123;
        let y: u16 = 2;
        let expected: u16 = 492;
        let actual = apply_gate(x, y, gate);

        assert_eq!(actual, expected);
    }

    #[test]
    fn apply_rshift() {
        let gate = LogicGate::RSHIFT;
        let x: u16 = 456;
        let y: u16 = 2;
        let expected: u16 = 114;
        let actual = apply_gate(x, y, gate);

        assert_eq!(actual, expected);
    }

    #[test]
    fn apply_not() {
        let gate = LogicGate::NOT;
        let x: u16 = 123;
        let expected: u16 = 65412;
        let actual = apply_gate(x, 0, gate);

        assert_eq!(actual, expected);
    }
}