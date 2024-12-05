use std::{cmp::Ordering, collections::HashSet};

fn main() {
    let (task1, task2) = solve();
    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

fn solve() -> (i32, i32) {
    let (orderings, printjobs) = include_str!("input.txt").split_once("\n\n").unwrap();

    let orderings = orderings
        .lines()
        .map(|m| m.split_once("|").unwrap())
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .collect::<HashSet<_>>();

    let printjobs = printjobs
        .lines()
        .map(|it| {
            it.split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let comp = |a: &i32, b: &i32| {
        if orderings.contains(&(*a, *b)) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    };
    let is_ordered =|p:&[i32]| p.windows(2).all(|w| comp(&w[0], &w[1]) == Ordering::Less);

    let task1 = printjobs
        .iter()
        .filter(|pages| is_ordered(pages))
        .map(|it| it[it.len() / 2])
        .sum::<i32>();

    let task2 = printjobs
        .iter()
        .filter(|pages| !is_ordered(pages))
        .cloned()
        .map(|mut it| {
            it.sort_by(comp);
            it[it.len() / 2]
        })
        .sum::<i32>();
    (task1, task2)
}

#[test]
fn test() {
    let result = solve();
    assert_eq!(result, (6612, 4944));
}
