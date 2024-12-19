use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let (samples, patterns) = include_str!("input.txt")
        .split("\n\n")
        .collect_tuple()
        .map(|(a, b)| (a.split(", ").collect_vec(), b.lines().collect_vec()))
        .unwrap();

    let mut c = 0;
    for p in &patterns {
        if try_sample(&samples, p, 0, &mut HashMap::new()) > 0 {
            c += 1;
        }
    }
    println!("task 1: {}", c);

    let task2 = patterns
        .iter()
        .map(|p| try_sample(&samples, p, 0, &mut HashMap::new()))
        .sum::<usize>();

    println!("task 2: {}", task2);
}

fn try_sample<'a, 'b>(
    samples: &[&'a str],
    pattern: &'b str,
    covered_len: usize,
    dp: &mut HashMap<&'b str, usize>,
) -> usize {
    let rest = &pattern[covered_len..];
    if let Some(cnt) = dp.get(&rest) {
        *cnt
    } else {
        let count = samples
            .iter()
            .map(|s| {
                if rest.starts_with(s) {
                    let next_len = covered_len + s.len();
                    if next_len == pattern.len() {
                        1
                    } else {
                        try_sample(samples, pattern, next_len, dp)
                    }
                } else {
                    0
                }
            })
            .sum::<usize>();

        dp.insert(rest, count);
        count
    }
}
