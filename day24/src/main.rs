use std::collections::HashSet;
use std::collections::{btree_map::Values, HashMap};

fn main() {
    let (mut wires, gates) = include_str!("input.txt")
        .split_once("\n\n")
        .map(|(wires, connections)| {
            let wires = wires
                .lines()
                .map(|line| {
                    let (name, value) = line.split_once(": ").unwrap();
                    (name, value.parse::<i8>().unwrap() != 0)
                })
                .collect::<std::collections::HashMap<_, _>>();

            let gates = connections
                .lines()
                .map(|line| {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    Gate {
                        input1: parts[0],
                        func: parts[1],
                        input2: parts[2],
                        output: parts[4],
                    }
                })
                .collect::<Vec<_>>();
            (wires, gates)
        })
        .unwrap();

    let mut changed = true;
    while changed {
        changed = false;
        for gate in &gates {
            changed = gate.eval(&mut wires) || changed;
        }
    }

    let mut z_wires = wires
        .iter()
        .filter(|(name, _)| name.starts_with("z"))
        .collect::<Vec<_>>();

    z_wires.sort_by_key(|(&name, _)| name);
    for (name, value) in &z_wires {
        println!("{}: {}", name, value);
    }

    let s: u64 = z_wires
        .iter()
        .map(|(name, &value)| {
            let num = name[1..].parse::<i32>().unwrap();
            if value {
                1u64 << num
            } else {
                0u64
            }
        })
        .sum();
    println!("Signal: {}", s);

    println!("---------------------------");
    fs::write(
        "/home/mari/projects/adventOfCode2024_rust/day24/mermaid.txt",
        to_mermaid(&gates),
    )
    .unwrap();

    verify_full_adder(&gates);
}

#[derive(Debug)]
struct Gate<'a> {
    input1: &'a str,
    input2: &'a str,
    func: &'a str,
    output: &'a str,
}

impl<'a> Gate<'a> {
    fn eval(&self, wires: &mut HashMap<&'a str, bool>) -> bool {
        if let Some((i1, i2)) = wires
            .get(self.input1)
            .cloned()
            .zip(wires.get(self.input2).cloned())
        {
            if !wires.contains_key(self.output) {
                let result = match self.func {
                    "AND" => i1 && i2,
                    "XOR" => i1 ^ i2,
                    "OR" => i1 || i2,
                    _ => panic!("Unknown function: {}", self.func),
                };
                wires.insert(self.output, result);
                return true;
            }
        }
        false
    }
}

use std::fmt::{format, Write};
use std::fs;

fn to_mermaid(gates: &[Gate]) -> String {
    let mut output = String::new();

    writeln!(&mut output, "flowchart aoc_day24");

    for g in gates.iter() {
        writeln!(&mut output, " {}[({})]", g.input1, g.input1).unwrap();
        writeln!(&mut output, " {}[({})]", g.input1, g.input2).unwrap();
        writeln!(&mut output, " {}[({})]", g.input1, g.output).unwrap();
    }

    for (i, g) in gates.iter().enumerate() {
        //     a00([a00]) --> xor1[xor]
        let g_name = format!("{}{}", g.func, i);
        writeln!(
            &mut output,
            " {}[({})] --> {}[{}]",
            g.input1, g.input1, g_name, g.func
        )
        .unwrap();
        writeln!(
            &mut output,
            " {}[({})] --> {}[{}]",
            g.input2, g.input2, g_name, g.func
        )
        .unwrap();
        writeln!(
            &mut output,
            " {}[{}] --> {}[({})]",
            g_name, g.func, g.output, g.output
        )
        .unwrap();
    }

    output
}

fn verify_full_adder(gates: &Vec<Gate>) {
    // according to full adder from https://en.wikipedia.org/wiki/Adder_(electronics)#/media/File:Fulladder.gif

    let mut faulty_wires: HashSet<&str> = HashSet::new();

    // every input should be connected to XORs (there are xors that are connected to inputs only, or none input)
    gates.iter().for_each(|g| {
        let xy1 = g.input1.starts_with("x") || g.input1.starts_with("y");
        let xy2 = g.input2.starts_with("x") || g.input2.starts_with("y");

        if xy1 ^ xy2 {
            if xy1 {
                faulty_wires.insert(g.input2);
            } else {
                faulty_wires.insert(g.input1);
            }
        }
    });

    // make sure all z-bits are connected to an XOR gate
    gates
        .iter()
        .filter(|g| g.output.starts_with("z"))
        // .filter(|g| g.func != "XOR")
        .filter(|g| g.output != "z45" && g.output != "z00") // this OR is fine
        .for_each(|g| {
            if g.func != "XOR" {
                faulty_wires.insert(g.output);
            }
        });

    // make sure the or gate is connected to two and-outputs
    gates
        .iter()
        .filter(|g| g.func == "OR" && g.output != "z45")
        .for_each(|g| {
            let faulty1 = gates
                .iter()
                .find(|it| it.output == g.input1 && it.func != "AND");
            let faulty2 = gates
                .iter()
                .find(|it| it.output == g.input2 && it.func != "AND");

            faulty1.inspect(|it| {
                faulty_wires.insert(it.output);
            });
            faulty2.inspect(|it| {
                faulty_wires.insert(it.output);
            });
        });

    let mut faulty_wires = faulty_wires.into_iter().collect::<Vec<_>>();

    // found tdv and tjp via visual inspection of the mermaid graph
    faulty_wires.push("tdv");
    faulty_wires.push("tjp");
    faulty_wires.sort();
    println!("Faulty wires: {:?}", faulty_wires.join(","));
}
