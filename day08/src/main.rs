use std::collections::{HashMap, HashSet};

use itertools::iproduct;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Vec2D {
    x: i32,
    y: i32,
}

impl Vec2D {
    fn new(x: i32, y: i32) -> Vec2D {
        Vec2D { x, y }
    }

    fn add(&self, other: &Vec2D) -> Vec2D {
        Vec2D::new(self.x + other.x, self.y + other.y)
    }
}

fn main() {
    let input = include_str!("input.txt").lines().collect::<Vec<_>>();

    let height = input.len() as i32;
    let width = input[0].len() as i32;

    let antennas = iproduct!(0..height, 0..width)
        .map(|(y, x)| (x, y, input[y as usize].chars().nth(x as usize).unwrap()))
        .filter(|(_, _, c)| *c != '.')
        .map(|(x, y, c)| (Vec2D::new(x, y), c))
        .collect::<HashMap<_, _>>();

    let mut antinodes_task1 = HashSet::new();
    let mut antinodes_task2 = HashSet::new();

    // check all antenna-combinations with the same frequency
    for ((a1, _), (a2, _)) in iproduct!(antennas.iter(), antennas.iter())
        .filter(|((pos1, f1), (pos2, f2))| pos1 != pos2 && f1 == f2)
    {
        let dist = Vec2D::new(a2.x - &a1.x, a2.y - &a1.y);

        //task 1
        let n = a2.add(&dist);
        if n.x >= 0 && n.x < width && n.y >= 0 && n.y < height {
            antinodes_task1.insert(n);
        }

        //task 2
        let mut v = dist.clone();
        // add v to ant1 position until it reaches the border
        while let Some(p) = Some(a1.add(&v))
            .filter(|p| p.x >= 0 && p.x < width as i32 && p.y >= 0 && p.y < height as i32)
        {
            antinodes_task2.insert(p);
            v = v.add(&dist);
        }
    }

    print(width, height, &antennas, &antinodes_task2);

    println!("Task 1: {}", antinodes_task1.len());
    println!("Task 2: {}", antinodes_task2.len());
}

fn print(w: i32, h: i32, antennas: &HashMap<Vec2D, char>, antinodes: &HashSet<Vec2D>) {
    for y in 0..h {
        for x in 0..w {
            let pos = Vec2D::new(x, y);
            if antennas.contains_key(&pos) {
                print!("{}", antennas.get(&pos).unwrap());
            } else if antinodes.contains(&pos) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
