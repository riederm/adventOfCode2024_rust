use std::collections::HashSet;

struct Map<'i>(Vec<&'i str>);

fn main() {
    let mut map = Map(include_str!("input.txt").lines().collect::<Vec<&str>>());

    let (x_start, y_start) = map.find_start().expect("start ^ not found");
    println!("Start: {}, {}", x_start, y_start);

    println!(
        "Part 1: {}",
        solve_part1(&map, x_start, y_start, Direction::new(0, -1)).len()
    );
    println!(
        "Part 2: {}",
        solve_part2(&mut map, x_start, y_start, Direction::new(0, -1))
    );
}

impl<'i> Map<'i> {
    fn get(&self, x: i32, y: i32) -> Option<char> {
        let line = self.0.get(y as usize);
        line.and_then(|l| l.chars().nth(x as usize).clone())
    }

    fn find_start(&self) -> Option<(i32, i32)> {
        self.0
            .iter()
            .enumerate()
            .find_map(|(y, line)| line.find('^').map(|x| (x as i32, y as i32)))
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Direction {
    x: i32,
    y: i32,
}

impl Direction {
    fn new(x: i32, y: i32) -> Direction {
        Direction { x, y }
    }

    fn turn_right(&mut self) {
        (self.x, self.y) = (-self.y, self.x);
    }
}

fn solve_part1(map: &Map, mut x: i32, mut y: i32, mut dir: Direction) -> HashSet<(i32, i32)> {
    let mut visited = HashSet::new();

    loop {
        visited.insert((x, y));
        let (next_x, next_y) = (x + dir.x, y + dir.y);
        match map.get(next_x, next_y) {
            Some('#') => dir.turn_right(),
            Some(_) => (x, y) = (next_x, next_y),
            None => return visited,
        }
    }
}

fn solve_part2(map: &mut Map, start_x: i32, start_y: i32, initial_dir: Direction) -> i64 {
    let mut candidates = solve_part1(map, start_x, start_y, initial_dir);
    candidates.remove(&(start_x, start_y));

    let solutions = candidates
        .iter()
        .map(|(nx, ny)| {
            let (mut x, mut y) = (start_x, start_y);
            let mut visited = HashSet::new();
            let mut dir = initial_dir.clone();

            loop {
                let (next_x, next_y) = (x + dir.x, y + dir.y);
                let field = map.get(next_x, next_y);
                if field == Some('#') || (next_x, next_y) == (*nx, *ny) {
                    if !visited.insert((next_x, next_y, dir.x, dir.y)) {
                        return 1;
                    }
                    dir.turn_right();
                } else if field.is_some() {
                    (x, y) = (next_x, next_y);
                } else {
                    return 0;
                }
            }
        })
        .sum();
    solutions
}
