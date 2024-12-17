use std::collections::HashMap;

fn main() {
    let (registers, instructions) = include_str!("input.txt")
        .split_once("\n\n")
        .map(|(a, b)| {
            let registers = a
                .lines()
                .map(|line| {
                    line.replace("Register ", "")
                        .split_once(": ")
                        .map(|(r, v)| (r.to_string(), v.parse::<i64>().unwrap()))
                        .unwrap()
                })
                .collect::<HashMap<_, _>>();

            let instructions = b
                .replace("Program: ", "")
                .split(",")
                .map(|it| it.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            (registers, instructions)
        })
        .unwrap();

    let r = Registers {
        a: *registers.get("A").unwrap(),
        b: *registers.get("B").unwrap(),
        c: *registers.get("C").unwrap(),
    };

    println!("Part 1: {:?}", execute(&r, &instructions));

    print_program(&instructions);
    println!("expected output: {instructions:?}");

    // from the program printed above we can see, that the program treats the input
    // 3 digits at a time, then it shifts it by 3 bits to the right (the division /8
    // so we can solve digit by digit. The program keeps the state of the previous digits
    // so in later digits, we might find several solutions.
    // we keep the smallest one
    let expected = instructions.clone();

    fn solve(instructions: &[i64], expected: &[i64], i: i64, a: i64, smallest: &mut i64) {
        if i == -1 {
            println!("Found candidate: a = {:?}", a);
            println!(
                "Part 2: {:?}",
                execute(&Registers { a, b: 0, c: 0 }, instructions)
            );

            *smallest = (*smallest).min(a);
            return;
        }

        // try all possible octal digits (3 bits)
        for digit in 0o0..=0o7 {
            let registers = Registers {
                a: (a << 3) + digit, // append this octal digit at the end of a
                b: 0,
                c: 0,
            };
            let r = execute(&(registers), &instructions);
            // check if element i is correct
            if r[0] == *expected.iter().nth(i as usize).unwrap() {
                // candidate .. continue with the next digit
                solve(instructions, expected, i - 1, registers.a, smallest);
            }
        }
    }

    let mut smallest = i64::MAX;
    solve(
        &instructions,
        &expected,
        expected.len() as i64 - 1,
        0,
        &mut smallest,
    );
    println!("Task 2: Smallest a = {smallest}");
}

#[derive(Clone)]
struct Registers {
    a: i64,
    b: i64,
    c: i64,
}

impl Registers {
    fn combo(&self, operand: i64) -> i64 {
        match operand {
            0 | 1 | 2 | 3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid operand"),
        }
    }
}

fn execute(r: &Registers, prog: &[i64]) -> Vec<i64> {
    let mut r = r.clone();
    let mut pc = 0;
    let mut out = Vec::new();

    while pc < prog.len() {
        let opc = prog[pc];
        let operand = prog[pc + 1];
        pc += 2;

        match opc {
            0 /*adv */=> r.a = r.a / 2i64.pow(r.combo(operand) as u32) as i64,
            1 /*bxl */=> r.b = r.b ^ operand,
            2 /*bst */=> r.b = r.combo(operand) & 0b0111,
            3 /*jnz */=> if r.a != 0 { pc = operand as usize },
            3 /*jnz */=> if r.a == 0 { /* ignore */ },
            4 /*bxc */=> r.b = r.b ^ r.c,
            5 /*out */=> out.push(r.combo(operand) & 0b0111),
            6 /*bdv */=> r.b = r.a / 2i64.pow(r.combo(operand) as u32) as i64,
            7 /*bdv */=> r.c = r.a / 2i64.pow(r.combo(operand) as u32) as i64,
            _ => panic!("Invalid opcode"),
        }
    }
    out
}

fn print_program(prog: &[i64]) {
    let mut pc = 0;
    while pc < prog.len() {
        let opc = prog[pc];
        let operand = prog[pc + 1];

        fn combo(operand: i64) -> String {
            match operand {
                0 | 1 | 2 | 3 => format!("{operand}"),
                4 => "A".to_string(),
                5 => "B".to_string(),
                6 => "C".to_string(),
                _ => panic!("Invalid operand"),
            }
        }
        pc += 2;
        match opc {
            0 => println!("A = A/2^{}", combo(operand)),
            1 => println!("B = B xor {}", operand),
            2 => println!("B = {} % 8", combo(operand)),
            3 => println!("jmp if A!=0 to {}", operand),
            4 => println!("B = B xor C"),
            5 => println!("out {} % 8", combo(operand)),
            6 => println!("B = A/2^{}", combo(operand)),
            7 => println!("C = A/2^{}", combo(operand)),
            _ => panic!("Invalid opcode"),
        }
    }
}
