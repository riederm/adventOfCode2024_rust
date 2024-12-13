
macro_rules! parse_button {
    ($line:expr, $delete01:expr, $delete02:expr) => {
        $line
            .replace($delete01, "")
            .replace($delete02, "")
            .split_once(",")
            .map(|(a, b)| (a.parse::<f64>().unwrap(), b.parse::<f64>().unwrap()))
            .unwrap()
    };
}

fn main() {
    let machines = include_str!("input.txt")
        .split("\n\n")
        .into_iter()
        .map(|machine_string| {
            let mut lines = machine_string.split("\n").into_iter();

            let button_a = parse_button!(lines.next().unwrap(), "Button A: X+", " Y+");
            let button_b = parse_button!(lines.next().unwrap(), "Button B: X+", " Y+");
            let prize = parse_button!(lines.next().unwrap(), "Prize: X=", " Y=");

            Machine {
                a: button_a,
                b: button_b,
                s: prize,
            }
        })
        .collect::<Vec<_>>();

    let result = machines
        .iter()
        .flat_map(|machine| machine.solve())
        .map(|(a, b)| a * 3 + b)
        .sum::<i64>();
    println!("task1: {}", result);

    let offset = 10000000000000f64;
    let result = machines
        .iter()
        .map(|m| Machine {
            a: m.a.clone(),
            b: m.b.clone(),
            s: (m.s.0 + offset, m.s.1 + offset),
        })
        .flat_map(|m| m.solve())
        .map(|(a, b)| a * 3 + b)
        .sum::<i64>();
    println!("task2: {}", result);
}

#[derive(Debug)]
struct Machine {
    a: (f64, f64),
    b: (f64, f64),
    s: (f64, f64),
}

impl Machine {
    fn solve(&self) -> Option<(i64, i64)> {
        // solve linear equations
        let d1 = self.s.0 / self.a.0 - self.s.1 / self.a.1;
        let div1 = self.b.0 / self.a.0 - self.b.1 / self.a.1;

        let b = d1 / div1;
        let a = self.s.1 / self.a.1 - b * self.b.1 / self.a.1;

        // the huuuge numbers of task2 cause big rounding errors, therefore our delta is pretty small
        if (a - a.round()).abs() < 0.01 && (b - b.round()).abs() < 0.01 {
            Some((a.round() as i64, b.round() as i64))
        } else {
            // no integer-solution
            None
        }
    }
}
