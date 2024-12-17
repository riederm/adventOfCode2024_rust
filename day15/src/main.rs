use std::collections::{HashSet, VecDeque};

use itertools::{iproduct, Itertools};

use colored::*;

struct Field(Vec<Vec<char>>);

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Point((i32, i32));

impl Into<(i32, i32)> for &Point {
    fn into(self) -> (i32, i32) {
        self.0
    }
}

impl Into<(i32, i32)> for &mut Point {
    fn into(self) -> (i32, i32) {
        self.0
    }
}

impl Point {
    fn x(&self) -> i32 {
        self.0 .0
    }

    fn y(&self) -> i32 {
        self.0 .1
    }

    fn walk(&mut self, mov: Move) {
        let (x, y) = self.get_neigbour(mov);
        self.0 = (x, y);
    }

    fn get_neigbour(&self, mov: Move) -> (i32, i32) {
        match mov {
            Move::Up => (self.x(), self.y() - 1),
            Move::Down => (self.x(), self.y() + 1),
            Move::Left => (self.x() - 1, self.y()),
            Move::Right => (self.x() + 1, self.y()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Move {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

impl Field {
    fn get(&self, pos: (i32, i32)) -> char {
        let (x, y) = pos;
        if x < 0 || y < 0 || x >= self.width() || y >= self.height() {
            return '#';
        }
        self.0[y as usize][x as usize]
    }

    fn set(&mut self, pos: (i32, i32), value: char) {
        self.0[pos.1 as usize][pos.0 as usize] = value;
    }

    fn width(&self) -> i32 {
        self.0[0].len() as i32
    }

    fn height(&self) -> i32 {
        self.0.len() as i32
    }

    fn get_sum_task1(&self) -> i32 {
        let mut sum = 0;
        for p in iproduct!(0..self.width(), 0..self.height()) {
            if self.get(p) == 'O' {
                sum += 100 * p.1 + p.0;
            }
        }
        sum
    }

    fn get_sum_task2(&self) -> i32 {
        let mut sum = 0;
        for p in iproduct!(0..self.width(), 0..self.height()) {
            if self.get(p) == '[' {
                sum += 100 * p.1 + p.0;
            }
        }
        sum
    }
}

fn main() {
    let (field, moves) = include_str!("input.txt").split_once("\n\n").unwrap();
    let original_field = field
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut field = Field(original_field.clone());
    let mut pos = Point(
        iproduct!(0..field.width(), 0..field.height())
            .find(|pos| field.get(*pos) == '@')
            .unwrap(),
    );

    let moves: Vec<Move> = moves
        .chars()
        .flat_map(|it| it.try_into().ok())
        .collect::<Vec<_>>();

    for m in moves.iter() {
        apply_move(&mut field, &mut pos, *m);
        // clear console

    }

    println!("Sum: {}", field.get_sum_task1());

    // task2
    //expand field
    let mut field = Field(
        original_field
            .iter()
            .map(|line| {
                line.iter()
                    .map(|c| match c {
                        '.' => vec!['.', '.'],
                        'O' => vec!['[', ']'],
                        '@' => vec!['@', '.'],
                        '#' => vec!['#', '#'],
                        _ => panic!("Unknown char"),
                    })
                    .concat()
                    .to_vec()
            })
            .collect_vec(),
    );

    let mut pos = Point(
        iproduct!(0..field.width(), 0..field.height())
            .find(|pos| field.get(*pos) == '@')
            .unwrap(),
    );

    for m in moves.iter() {
        apply_move(&mut field, &mut pos, *m);
        // print!("\x1B[2J\x1B[1;1H");
        // print!("Move: {:?}\n", m);
        // print_field(&field, &pos);

        //read line
        // std::io::stdin().read_line(&mut String::new()).unwrap();
    }

    print_field(&field, &pos);

    println!("Sum: {}", field.get_sum_task2());
}

fn print_field(field: &Field, pos: &Point) {
    for y in 0..field.height() {
        for x in 0..field.width() {
            if pos.x() == x && pos.y() == y {
                print!("{}", "@".green());
            } else {
                let ch = field.get((x, y));
                match ch {
                    '#' => print!("{}", ch.to_string().bright_black()),
                    '[' | ']' => print!("{}", ch.to_string().yellow()),
                    '.' => print!("{}", ch.to_string().bright_black()),
                    _ => print!("{}", ch),
                }
            }
        }
        println!();
    }
}

fn apply_move(field: &mut Field, current_pos: &mut Point, mov: Move) {
    // collect all obstacles in the way
    let mut to_move = HashSet::new();
    let mut pending_positions = VecDeque::from([current_pos.0]);

    let is_obstacle =|c: char| -> bool {c == 'O' || c == '[' || c == ']'};
    while let Some(pos) = pending_positions.pop_front() {
        if !(is_obstacle(field.get(pos)) || pos == current_pos.into())  {
            continue;
        }
        if !to_move.insert(pos) {
            continue;
        };
        pending_positions.push_back(Point(pos).get_neigbour(mov));

        if field.get(pos) == '[' {
            pending_positions.push_back((pos.0 + 1, pos.1));
        } else if field.get(pos) == ']' {
            pending_positions.push_back((pos.0 - 1, pos.1));
        }
    }

    // check if somebody hits on wall
    if to_move
        .iter()
        .map(|p| Point(*p))
        .map(|it| it.get_neigbour(mov))
        .any(|p| field.get(p) == '#')
    {
        // somebody hits wall so we can't move
        return;
    }

    // move all into mov
    let to_move = to_move.into_iter()
        .map(|p| (Point(p), field.get(p)))
        .collect::<Vec<_>>();

    // first clear all
    for (p, _) in to_move.iter() {
        field.set(p.into(), '.');
    }
    // move all
    for (p, c) in to_move.iter() {
        let p = p.get_neigbour(mov);
        field.set(p, *c);
    }

    field.set(current_pos.into(), '.');
    current_pos.walk(mov);
    field.set(current_pos.into(), '@');
}
