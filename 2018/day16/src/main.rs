use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

type Registers = [u32; 4];

#[derive(Copy, Clone, Debug)]
struct Instruction {
    a: u32,
    b: u32,
    out: usize,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Opcode {
    AddR,
    AddI,
    MulR,
    MulI,
    BanR,
    BanI,
    BorR,
    BorI,
    SetR,
    SetI,
    GtIR,
    GtRI,
    GtRR,
    EqIR,
    EqRI,
    EqRR,
}

impl Opcode {
    fn execute(
        self,
        mut registers: Registers,
        Instruction { a, b, out }: Instruction,
    ) -> Registers {
        let load = |i: u32| registers[i as usize];
        let gt = |a: u32, b: u32| if a > b { 1 } else { 0 };
        let eq = |a: u32, b: u32| if a == b { 1 } else { 0 };

        registers[out] = match self {
            Opcode::AddR => load(a) + load(b),
            Opcode::AddI => load(a) + b,
            Opcode::MulR => load(a) * load(b),
            Opcode::MulI => load(a) * b,
            Opcode::BanR => load(a) & load(b),
            Opcode::BanI => load(a) & b,
            Opcode::BorR => load(a) | load(b),
            Opcode::BorI => load(a) | b,
            Opcode::SetR => load(a),
            Opcode::SetI => a,
            Opcode::GtIR => gt(a, load(b)),
            Opcode::GtRI => gt(load(a), b),
            Opcode::GtRR => gt(load(a), load(b)),
            Opcode::EqIR => eq(a, load(b)),
            Opcode::EqRI => eq(load(a), b),
            Opcode::EqRR => eq(load(a), load(b)),
        };

        registers
    }
}

fn matching_opcodes(input: Registers, instruction: Instruction, output: Registers) -> Vec<Opcode> {
    use Opcode::*;

    let ops = [
        AddR, AddI, MulR, MulI, BanR, BanI, BorR, BorI, SetR, SetI, GtIR, GtRI, GtRR, EqIR, EqRI,
        EqRR,
    ];

    ops.iter()
        .filter(|op| op.execute(input, instruction) == output)
        .copied()
        .collect()
}

fn parse_registers(s: &str) -> Registers {
    let pattern: &[char] = &['[', ']', ' '];

    let parsed: Vec<u32> = s
        .trim_matches(pattern)
        .split(',')
        .flat_map(|s| s.trim().parse::<u32>())
        .collect();

    [parsed[0], parsed[1], parsed[2], parsed[3]]
}

fn parse_instruction(s: &str) -> (usize, Instruction) {
    let parsed: Vec<u32> = s
        .split_whitespace()
        .flat_map(|s| s.trim().parse::<u32>())
        .collect();

    (
        parsed[0] as usize,
        Instruction {
            a: parsed[1],
            b: parsed[2],
            out: parsed[3] as usize,
        },
    )
}

fn solve_opcodes(mut opcodes: Vec<HashSet<Opcode>>) -> Vec<Opcode> {
    let mut known: HashMap<Opcode, usize> = HashMap::new();

    while known.len() < opcodes.len() {
        for (i, op) in opcodes.iter_mut().enumerate() {
            op.retain(|c| !known.contains_key(c));

            if op.len() == 1 {
                for code in op.iter() {
                    known.insert(*code, i);
                }
            }
        }
    }

    let mut out: Vec<(Opcode, usize)> = known.into_iter().collect();
    out.sort_by_key(|i| i.1);

    // Drop the index
    out.into_iter().map(|(k, _)| k).collect()
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let mut iter = input.lines();
    let mut instructions = Vec::new();
    let mut matching_codes: Vec<HashSet<Opcode>> = vec![HashSet::new(); 16];
    let mut count = 0;

    while let Some(line) = iter.next() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("Before: ") {
            let instruction_line = iter.next().expect("Invalid input.");
            let output_line = iter.next().expect("Invalid input.");

            let input = parse_registers(&line[8..]);
            let (code, instruction) = parse_instruction(&instruction_line);
            let output = parse_registers(&output_line[8..]);

            let ops = matching_opcodes(input, instruction, output);

            if ops.len() >= 3 {
                count += 1; // Part 1.
            }

            for op in ops {
                matching_codes[code].insert(op);
            }
        } else {
            instructions.push(parse_instruction(&line));
        }
    }

    let ops = solve_opcodes(matching_codes);
    let mut registers = [0, 0, 0, 0];
    for (code, instruction) in instructions {
        registers = ops[code].execute(registers, instruction);
    }

    println!("Number of codes matching three or more opcodes: {}", count);
    println!("Registers: {:?}", registers)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Before: [3, 2, 1, 1]
    const SAMPLE_REGISTERS: Registers = [3, 2, 1, 1];

    // 9 2 1 2
    const SAMPLE_OP: Instruction = Instruction { a: 2, b: 1, out: 2 };

    // After:  [3, 2, 2, 1]
    const SAMPLE_RESULT: Registers = [3, 2, 2, 1];

    #[test]
    fn addi() {
        // addi 0 7 3
        let op = Opcode::AddI;
        let reg = op.execute([0, 1, 2, 3], Instruction { a: 0, b: 7, out: 3 });
        assert_eq!([0, 1, 2, 7], reg);
    }

    #[test]
    fn sample_addi() {
        let op = Opcode::AddI;
        let reg = op.execute(SAMPLE_REGISTERS, SAMPLE_OP);

        assert_eq!(SAMPLE_RESULT, reg);
    }

    #[test]
    fn sample_mulr() {
        let op = Opcode::MulR;
        let reg = op.execute(SAMPLE_REGISTERS, SAMPLE_OP);

        assert_eq!(SAMPLE_RESULT, reg);
    }

    #[test]
    fn sample_seti() {
        let op = Opcode::SetI;
        let reg = op.execute(SAMPLE_REGISTERS, SAMPLE_OP);

        assert_eq!(SAMPLE_RESULT, reg);
    }

    #[test]
    fn example() {
        // Before: [3, 2, 1, 1]
        // 9 2 1 2
        // After:  [3, 2, 2, 1]

        assert_eq!(
            vec![Opcode::AddI, Opcode::MulR, Opcode::SetI],
            matching_opcodes(SAMPLE_REGISTERS, SAMPLE_OP, SAMPLE_RESULT)
        );
    }
}
