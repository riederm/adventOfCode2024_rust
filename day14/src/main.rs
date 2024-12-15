use std::{collections::HashSet, ops::Range};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}

impl Robot {
    fn step(&mut self, max: (i32, i32)) {
        self.p.0 += self.v.0;
        self.p.1 += self.v.1;
        self.p = (self.p.0.rem_euclid(max.0), self.p.1.rem_euclid(max.1));
    }
}

fn main() {
    let mut robots = include_str!("input.txt")
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(p, v)| {
            let p = p[2..]
                .split_once(",")
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap();
            let v = v[2..]
                .split_once(",")
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap();
            Robot { p, v }
        })
        .collect::<Vec<_>>();

    // dbg!(robots);
    let max = (101, 103);

    let (midx, midy) = (max.0 / 2, max.1 / 2);
    let q = vec![
        (0..midx, 0..midy),
        (midx + 1..max.0, 0..midy),
        (0..midx, midy + 1..max.1),
        (midx + 1..max.0, midy + 1..max.1),
    ];

    let mut step = 0;

    loop {
        for r in robots.iter_mut() {
            r.step(max);
        }
        step += 1;

        if step == 100 {
            let task1 = q
                .iter()
                .map(|it| get_robots_in_q(&robots, it))
                .product::<usize>();

            println!("Task 1: {}", task1);
        } else if has_christmas_tree(&robots) {
            println!("Task 2: step {}", step );
            print(&robots, max);
            break;
        }
        if step % 1000 == 0 {
            println!("Step: {}", step);
        }
    }
}

fn get_robots_in_q(robots: &[Robot], q: &(Range<i32>, Range<i32>)) -> usize {
    robots
        .iter()
        .filter(|r| q.0.contains(&r.p.0) && q.1.contains(&r.p.1))
        .count()
}

fn get_robot_counts(robots: &[Robot], max: (i32, i32)) -> Vec<Vec<usize>> {
    let mut counts = vec![vec![0; max.0 as usize]; max.1 as usize];
    for r in robots.iter() {
        counts[r.p.1 as usize][r.p.0 as usize] += 1;
    }
    counts
}

fn has_christmas_tree(robots: &[Robot]) -> bool {

    let robots = robots.iter().map(|it| it.p).collect::<HashSet<_>>();

    // we look for this pattern (from @'s perspective):
    // ..@..
    // .###.
    // #####

    #[rustfmt::skip]
    let pattern = vec![
                        (0,0),
                (-1,1), (0,1), (1,1),
        (-2,2), (-1,2), (0,2), (1,2), (2,2),
    ];

    for (x,y) in robots.iter() {
        if pattern.iter().all(|(dx, dy)| robots.contains(&(x + dx, y + dy))) {
            return true;
        }
    }
    return false;
}

fn print(robots: &[Robot], max: (i32, i32)) {
    for row in get_robot_counts(robots, max) {
        println!(
            "{}",
            row.iter()
                .map(|it| if it == &0 {
                    '.'
                } else {
                    it.to_string().chars().next().unwrap()
                })
                .collect::<String>()
        );
    }
}
