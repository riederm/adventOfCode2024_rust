use std::collections::{HashMap, HashSet};
use itertools::{iproduct, Itertools};

type XY = (i32, i32);

fn main() {
    let map = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (h, w) = (map.len(), map[0].len());
    let start = iproduct!(0..h, 0..w)
        .find(|&(i, j)| map[i][j] == 'S')
        .unwrap();

    let dist = collect_distances(&map, start.1 as i32, start.0 as i32);

    let cheats = walk_with_cheats(&dist, 2);
    let sum = cheats
        .iter()
        .filter(|(k, _v)| **k >= 100)
        .map(|(_, v)| v)
        .sum::<usize>();
    println!("Part 1: {}", sum);

    let cheats = walk_with_cheats(&dist, 20);
    let sum = cheats
        .iter()
        .filter(|(k, _v)| **k >= 100)
        .map(|(_, v)| v)
        .sum::<usize>();
    println!("Part 2: {}", sum);
}

fn collect_distances(map: &[Vec<char>], x: i32, y: i32) -> HashMap<XY, usize> {
    let mut dist = HashMap::from([((x, y), 0)]);
    let mut x = x;
    let mut y = y;
    // let mut path = vec![(x,y)];
    let mut len = 1;
    while map[y as usize][x as usize] != 'E' {
        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (nx, ny) = ((x + dx), (y + dy));
            if map
                .get(ny as usize)
                .and_then(|row| row.get(nx as usize))
                .is_some_and(|c| *c != '#')
            {
                if !dist.contains_key(&(nx, ny)) {
                    dist.insert((nx, ny), len);
                    len += 1;
                    x = nx;
                    y = ny;
                }
            }
        }
    }
    dist
}

fn walk_with_cheats(dist: &HashMap<XY, usize>, cheat_len: i32) -> HashMap<usize, usize> {
    let cheat_steps = build_cheat_steps(cheat_len);
    let mut found_cheats = HashSet::<(XY, XY)>::new();
    let mut cheats = HashMap::<usize, usize>::new();
    for (x, y) in dist.keys().cloned() {
        for (dx, dy) in &cheat_steps {
            let (nx, ny) = (x + dx, y + dy);
            let curr_len = dist.get(&(x, y)).cloned().unwrap();
            if let Some(reached_len) = dist.get(&(nx, ny)).cloned() {
                let travel_dist = dx.abs() as usize + dy.abs() as usize;
                // see if this really shortens the path
                if reached_len > (curr_len + travel_dist)
                    && found_cheats.insert(((x, y), (nx, ny)))
                {
                    cheats
                        .entry(reached_len - (curr_len + travel_dist))
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }
        }
    }
    return cheats;
}

fn build_cheat_steps(cheat_len: i32) -> Vec<XY> {
    let mut cheat_steps = HashSet::new();
    for i in -cheat_len..=cheat_len {
        for j in -cheat_len..=cheat_len {
            if i.abs() + j.abs() <= cheat_len {
                cheat_steps.insert((i as i32, j as i32));
            }
        }
    }
    return cheat_steps.into_iter().collect_vec();
}
