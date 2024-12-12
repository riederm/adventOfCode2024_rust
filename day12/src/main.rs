use std::collections::HashSet;

use itertools::{iproduct, Itertools};

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

struct Field {
    field: Vec<Vec<char>>,
}

impl Field {
    fn get(&self, x: i32, y: i32) -> Option<((i32, i32), char)> {
        if y < 0 || x < 0 || y >= self.field.len() as i32 || x >= self.field[0].len() as i32 {
            return None;
        }

        Some(((x, y), self.field[y as usize][x as usize]))
    }
}

#[derive(Debug)]
struct Area {
    _name: char,
    fields: HashSet<(i32, i32)>,
}

impl Area {
    fn get_perimeter(&self) -> u64 {
        let mut peri = 0u64;
        for (x, y) in self.fields.iter() {
            peri += 4 - DIRECTIONS
                .iter()
                .map(|(dx, dy)| (x + dx, y + dy))
                .filter(|it| self.fields.contains(it))
                .count() as u64;
        }
        peri
    }

    fn get_price(&self) -> u64 {
        self.fields.len() as u64 * self.get_perimeter()
    }

    fn get_price2(&self) -> u64 {
        let mut m = Mouse::default();
        let sides = m.walk(self);
        self.fields.len() as u64 * sides
    }
}

#[derive(Default)]
struct Mouse {
    pos: (i32, i32),
    dir: (i32, i32),
}

impl Mouse {
    // mouse walks around the area (touching a wall) and counts how many turns were necessary
    fn walk(&mut self, area: &Area) -> u64 {
        let mut total_sides = 0;
        let mut seen = HashSet::new();

        // lets start the search from every cell with every start direction
        // to make sure that we also find islands in center of the area
        for (start, start_dir) in iproduct!(area.fields.iter(), DIRECTIONS.iter()) {
            self.pos = *start;
            self.dir = *start_dir;
            // walk until we hit a wall
            while self.can_walk(area) {
                self.walk_ahead();
            }
            // turn right to start walking around the wall (keep wall at left shoulder)
            self.turn_right();
            // remember where we start
            let (start, dir) = (self.pos, self.dir);
            let mut turns = 0;
            while self.pos != start || self.dir != dir || turns == 0 {
                if !seen.insert((self.pos, self.dir)) {
                    // we've been here
                    turns = 0;
                    break;
                }
                // walk around the walls, keep walk at left shoulder
                if self.can_left(area) {
                    self.turn_left();
                    self.walk_ahead();
                    turns += 1;
                } else if !self.can_walk(area) {
                    self.turn_right();
                    turns += 1;
                } else {
                    self.walk_ahead();
                }
            }
            total_sides += turns;
        }
        total_sides
    }

    fn turn_right(&mut self) {
        self.dir = (-self.dir.1, self.dir.0);
    }

    fn turn_left(&mut self) {
        self.dir = (self.dir.1, -self.dir.0);
    }

    fn can_left(&mut self, area: &Area) -> bool {
        self.turn_left();
        let can_right = self.can_walk(area);
        self.turn_right();
        can_right
    }

    fn can_walk(&self, area: &Area) -> bool {
        area.fields.contains(&(self.pos.0 + self.dir.0, self.pos.1 + self.dir.1))
    }

    fn walk_ahead(&mut self) {
        self.pos = (self.pos.0 + self.dir.0, self.pos.1 + self.dir.1);
    }
}

fn main() {
    let field = include_str!("test.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let field = Field { field };
    let mut visited = HashSet::new();

    let areas = iproduct!(0..field.field.len() as i32, 0..field.field[0].len() as i32)
        .flat_map(|(y, x)| get_area(&field, y, x, &mut visited))
        .collect_vec();

    println!(
        "task1: {}",
        areas.iter().map(|a| a.get_price()).sum::<u64>()
    );
    println!(
        "task2: {}",
        areas.iter().map(|a| a.get_price2()).sum::<u64>()
    );
}

fn get_area(field: &Field, y: i32, x: i32, already_seen: &mut HashSet<(i32, i32)>) -> Option<Area> {
    if let Some((p, name)) = field.get(x, y) {
        let mut fields = HashSet::new();

        let mut pending = vec![(p, name)];

        // collect connected neighbours with same name
        while let Some(((curr_x, curr_y), c)) = pending.pop() {
            if c == name && already_seen.insert((curr_x, curr_y)) {
                fields.insert((curr_x, curr_y));

                // add new neighbous that need to be tried
                pending.append(
                    &mut DIRECTIONS
                        .iter()
                        .flat_map(|(dy, dx)| field.get(curr_x + dx, curr_y + dy))
                        .collect_vec(),
                );
            }
        }

        if !fields.is_empty() {
            return Some(Area { _name: name, fields });
        }
    }
    return None;
}
