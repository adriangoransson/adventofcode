use std::{
    collections::VecDeque,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Could not read stdin");

    let prepared = prepare_input(&input);
    let (part_1, part_2) = (part_1(&prepared), part_2(&prepared));

    println!("Part 1: {part_1}\nPart 2: {part_2}")
}

struct Instruction {
    from: usize,
    to: usize,
    amount: usize,
}

type Prepared = (Vec<VecDeque<char>>, Vec<Instruction>);
type Output = String;

fn prepare_input(input: &str) -> Prepared {
    let mut lines = input.lines();

    let mut stacks = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mut chars = line.chars();
        let mut stack_index = 0;
        while let Some(item) = chars
            .next()
            .and_then(|_| chars.next().and_then(|c| chars.next().map(|_| c)))
        {
            if stack_index >= stacks.len() {
                stacks.push(VecDeque::new());
            }

            if item.is_alphabetic() {
                stacks[stack_index].push_back(item);
            }

            stack_index += 1;
            chars.next();
        }
    }

    let mut instructions = Vec::new();
    while let Some(line) = lines.next() {
        let instruction = line
            .split_whitespace()
            .filter_map(|s| s.parse().into())
            .flatten()
            .collect::<Vec<_>>();

        instructions.push(Instruction {
            amount: instruction[0],
            from: instruction[1],
            to: instruction[2],
        });
    }

    (stacks, instructions)
}

fn part_1(p: &Prepared) -> Output {
    let mut stacks = p.0.clone();

    for i in &p.1 {
        for _ in 0..i.amount {
            let item = stacks[i.from - 1].pop_front().unwrap();
            stacks[i.to - 1].push_front(item);
        }
    }

    answer(stacks)
}

fn part_2(p: &Prepared) -> Output {
    let mut stacks = p.0.clone();

    let mut temp = Vec::new();
    for i in &p.1 {
        for _ in 0..i.amount {
            let item = stacks[i.from - 1].pop_front().unwrap();
            temp.push(item);
        }

        while let Some(item) = temp.pop() {
            stacks[i.to - 1].push_front(item);
        }
    }

    answer(stacks)
}

fn answer(stacks: Vec<VecDeque<char>>) -> Output {
    let mut ret = String::new();
    for mut stack in stacks {
        ret.push(stack.pop_front().unwrap());
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part_1() {
        let answer = super::part_1(&prepare_input(EXAMPLE));
        assert_eq!(answer, "CMZ");
    }

    #[test]
    fn part_2() {
        let answer = super::part_2(&prepare_input(EXAMPLE));
        assert_eq!(answer, "MCD");
    }
}
