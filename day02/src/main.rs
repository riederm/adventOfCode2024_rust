fn main() {
    let (task1, task2) = solve();

    println!("task1 : {}", task1);
    println!("task2 : {}", task2);
}

#[test]
fn test() {
    assert_eq!(solve(), (534, 577));
}

fn solve() -> (usize, i32) {
    let samples = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|it| it.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let task1 = samples.iter().filter(|it| check(it)).count();

    let mut task2 = 0;
    for s in samples.iter() {
        for i in 0..s.len() {
            let filtered = s
                .iter()
                .enumerate()
                .filter(|(idx, _)| *idx != i)
                .map(|(_, val)| *val)
                .collect::<Vec<_>>();
            if check(&filtered) {
                task2 += 1;
                break;
            }
        }
    }
    (task1, task2)
}

fn check(s: &[i32]) -> bool {
    let expected_signum = (s[1] - s[0]).signum();
    s.windows(2).all(|w| {
        let diff = w[1] - w[0];
        diff.signum() == expected_signum && diff.abs() <= 3
    })
}
