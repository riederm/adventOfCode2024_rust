use regex::Regex;

fn main() {
    let input = include_str!("input.txt");

    println!("Task 1: {}", task1(input));
    println!("Task 2: {}", task2(input));
}

fn task1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let task1: i32 = re
        .captures_iter(input)
        .map(|mul| mul[1].parse::<i32>().unwrap() * mul[2].parse::<i32>().unwrap())
        .sum();
    task1
}

fn task2(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    let mut mul = 0;
    for cap in re.captures_iter(input) {
        if cap[0].eq("do()") {
            enabled = true;
        } else if cap[0].eq("don't()") {
            enabled = false;
        } else if enabled {
            mul += cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
        }
    }
    mul
}


#[test]
fn test() {
    let input = include_str!("input.txt");
    assert_eq!((174336360, 88802350), (task1(input), task2(input)));
}