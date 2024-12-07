enum Op {
    MUL,
    ADD,
    CONCAT,
}

impl Op {
    fn eval(&self, a: i64, b: i64) -> i64 {
        match self {
            Op::MUL => a * b,
            Op::ADD => a + b,
            Op::CONCAT => format!("{a}{b}").parse::<i64>().unwrap(),
        }
    }
}

fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|line| line.split_once(":").unwrap())
        .map(|(left, right)| {
            (
                left.parse::<i64>().unwrap(),
                right
                    .trim()
                    .split_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let part1: i64 = input
        .iter()
        .filter(|(expected, operands)| {
            try_solve(*expected, operands[0], operands, 1, &vec![Op::MUL, Op::ADD])
        })
        .map(|(expected, _)| *expected)
        .sum();
    println!("Part 1: {}", part1);

    let part2: i64 = input
        .iter()
        .filter(|(expected, operands)| {
            try_solve(
                *expected,
                operands[0],
                operands,
                1,
                &vec![Op::MUL, Op::ADD, Op::CONCAT],
            )
        })
        .map(|(expected, _)| *expected)
        .sum();
    println!("Part 2: {}", part2);
}

fn try_solve(expected: i64, current: i64, operands: &Vec<i64>, i: usize, operators: &Vec<Op>) -> bool {
    if current > expected {
        return false;
    } else if i == operands.len() {
        return current == expected;
    } else {
        operators.iter().any(|op| {
            let operand = operands[i];
            let next = op.eval(current, operand);
            try_solve(expected, next, operands, i + 1, operators)
        })
    }
}
