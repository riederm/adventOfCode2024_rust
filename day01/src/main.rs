use std::collections::HashMap;

fn main() {
    let lines = include_str!("input.txt")
        .lines()
        .into_iter()
        .map(|it| {
            it.split_once("   ")
                .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
                .unwrap()
        })
        .collect::<Vec<(i32, i32)>>();

    let (mut left, mut right)= lines.into_iter().unzip::<i32, i32, Vec<_>, Vec<_>>();
    left.sort();
    right.sort();

    let task1 = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<u32>();

    println!("task1: {task1}");

    let mut frequencies = HashMap::<i32, i32>::new();
    let mut current: (i32, i32) = (right.first().cloned().unwrap(), 0);
    // -1 at end makes sure we get a difference in the last element
    for v in right.iter().chain([-1].iter()).cloned() {
        if v == current.0 {
            current.1 += 1;
        } else if current.1 > 0 {
            frequencies.insert(current.0, current.1);
            current = (v, 1);
        }
    }

    let task2 = left
        .iter()
        .map(|it| frequencies.get(it).cloned().unwrap_or(0) * it)
        .sum::<i32>();
    println!("task2: {task2}");
}
