use std::collections::HashMap;

fn main() {
    let data = include_str!("input.txt")
        .split(" ")
        .map(|it| it.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    println!("task1: {:}", count_stones(&data, 25));
    println!("task2: {:}", count_stones(&data, 75));
}

fn count_stones(stones: &Vec<u64>, blinks: usize) -> u64 {
    let mut memo = HashMap::new();
    stones
        .iter()
        .map(|it| calculate(0, *it, &mut memo, blinks))
        .sum()
}

fn calculate(n: usize, d: u64, memo: &mut HashMap<(u64, usize), u64>, blinks: usize) -> u64 {
    if n == blinks {
        memo.insert((d, n), 1);
        return 1;
    } else if let Some(n) = memo.get(&(d, n)) {
        return *n;
    }

    if d == 0 {
        calculate(n + 1, 1, memo, blinks)
    } else {
        let dstr = format!("{:}", d);
        if dstr.len() % 2 == 0 {
            let (first, second) = dstr.split_at(dstr.len() / 2);
            let (a, b) = (
                first.parse::<u64>().unwrap(),
                second.parse::<u64>().unwrap(),
            );
            let a = calculate(n + 1, a, memo, blinks);
            let b = calculate(n + 1, b, memo, blinks);
            memo.insert((d, n), a + b);
            return a + b;
        } else {
            let count = calculate(n + 1, d * 2024, memo, blinks);
            memo.insert((d, n), count);
            return count;
        }
    }
}
