fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("examples/18/input.txt")?;
    use crate::Operator::*;
    use Token::*;
    let lines = input.lines().map(|line| {
        line.chars().filter_map(|ch| match ch {
            ' ' => None,
            '+' => Some(Operator(Add)),
            '*' => Some(Operator(Mul)),
            '(' => Some(Operator(LParen)),
            ')' => Some(Operator(RParen)),
            _ => Some(Number(ch.to_digit(10).unwrap() as u64)),
        })
    });

    // Shunting-yard_algorithm
    let mut sum = 0;
    for line in lines.clone() {
        let mut stack: Vec<Token> = Vec::new();
        let mut output_queue: Vec<Token> = Vec::new();
        for token in line {
            match token {
                Number(_) => output_queue.push(token),
                Operator(LParen) => stack.push(token),
                Operator(RParen) => {
                    while let Some(stack_op) = stack.pop() {
                        if stack_op == Operator(LParen) {
                            break;
                        }
                        output_queue.push(stack_op);
                    }
                }
                Operator(op) => {
                    while let Some(stack_op) = stack.pop() {
                        if stack_op == Operator(LParen) {
                            stack.push(stack_op);
                            break;
                        }
                        output_queue.push(stack_op);
                    }
                    stack.push(Operator(op));
                }
            }
        }
        while let Some(stack_op) = stack.pop() {
            output_queue.push(stack_op);
        }
        let rps = output_queue;
        let mut stack: Vec<u64> = Vec::new();
        for token in rps {
            match token {
                Operator(op) => match op {
                    Add => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(a + b)
                    }
                    Mul => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(a * b)
                    }
                    LParen | RParen => unreachable!(),
                },
                Number(n) => {
                    stack.push(n);
                }
            }
        }
        sum += stack.pop().unwrap();
    }
    println!("{}", sum);

    // Part 2
    let mut sum = 0;
    for line in lines {
        let mut stack: Vec<Token> = Vec::new();
        let mut output_queue: Vec<Token> = Vec::new();
        for token in line {
            match token {
                Number(_) => output_queue.push(token),
                Operator(LParen) => stack.push(token),
                Operator(RParen) => {
                    while let Some(stack_op) = stack.pop() {
                        if stack_op == Operator(LParen) {
                            break;
                        }
                        output_queue.push(stack_op);
                    }
                }
                Operator(op) => {
                    while let Some(stack_op) = stack.pop() {
                        if stack_op != Operator(LParen)
                            && (stack_op == Operator(Add) || (Operator(op) == stack_op))
                        {
                            output_queue.push(stack_op);
                        } else {
                            stack.push(stack_op);
                            break;
                        }
                    }
                    stack.push(Operator(op));
                }
            }
        }
        while let Some(stack_op) = stack.pop() {
            output_queue.push(stack_op);
        }
        let rps = output_queue;
        let mut stack: Vec<u64> = Vec::new();
        for token in rps {
            match token {
                Operator(op) => match op {
                    Add => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(a + b)
                    }
                    Mul => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(a * b)
                    }
                    LParen | RParen => unreachable!(),
                },
                Number(n) => {
                    stack.push(n);
                }
            }
        }
        sum += stack.pop().unwrap();
    }
    println!("{}", sum);
    Ok(())
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Token {
    Number(u64),
    Operator(Operator),
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Operator {
    Add,
    Mul,
    LParen,
    RParen,
}
