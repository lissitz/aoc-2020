use std::str::FromStr;
extern crate lazy_static;
use std::collections::HashSet;
fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("examples/8/input.txt")?;
    let instructions: Vec<_> = input
        .lines()
        .map(|x| Instruction::from_str(x).unwrap())
        .collect();
    let value = run(&instructions).err().unwrap();
    println!("{}", value);

    // Part 2
    let faulty_instructions = instructions
        .iter()
        .enumerate()
        .filter(|(_, instruction)| match instruction.op {
            Op::Nop | Op::Jmp => true,
            _ => false,
        })
        .map(|(i, _)| i);

    for i in faulty_instructions {
        let mut instructions = instructions.clone();
        instructions[i].op = match instructions[i].op {
            Op::Jmp => Op::Nop,
            Op::Nop => Op::Jmp,
            _ => unreachable!(),
        };
        if let Ok(count) = run(&instructions) {
            println!("{}", count);
        }
    }
    Ok(())
}

fn run(instructions: &Vec<Instruction>) -> Result<i64, i64> {
    let mut value = 0;
    let mut pc = 0;
    let mut visited: HashSet<usize> = HashSet::new();
    loop {
        match instructions.get(pc as usize) {
            Some(instruction) => {
                if visited.contains(&(pc as usize)) {
                    break;
                }
                visited.insert(pc as usize);
                match instruction.op {
                    Op::Acc => {
                        value += instruction.argument;
                        pc += 1;
                    }
                    Op::Nop => {
                        pc += 1;
                    }
                    Op::Jmp => {
                        pc += instruction.argument;
                    }
                }
            }
            None => return Ok(value),
        }
    }
    Err(value)
}

#[derive(Copy, Clone)]
struct Instruction {
    op: Op,
    argument: i64,
}

#[derive(Copy, Clone)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug)]
struct ParseInstructionError;
impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.trim().split(' ');
        let op = match it.next().unwrap() {
            "nop" => Op::Nop,
            "acc" => Op::Acc,
            "jmp" => Op::Jmp,
            _ => unreachable!(),
        };
        let argument = it.next().unwrap().parse::<i64>().unwrap();
        Ok(Self { op, argument })
    }
}
