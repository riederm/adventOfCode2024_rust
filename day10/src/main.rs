use std::collections::HashSet;

use itertools::iproduct;

struct Map{
    map: Vec<Vec<u8>>,
}

impl Map {
    fn get(&self, x: i32, y: i32) -> u8 {
        if x < 0 || x >= self.map[0].len() as i32 || y < 0 || y >= self.map.len() as i32 {
            return u8::MAX;
        }
        self.map[y as usize][x as usize]
    }
}

fn main() {

    let map = include_str!("input.txt")
        .lines()
        .map(|l| l.chars().map(|c| if c == '.' { 99u8 } else { c.to_digit(10).unwrap() as u8 }).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (len_y, len_x) = (map.len(), map[0].len());

    let map = Map{map};
    let mut count1 = 0;
    let mut count2 = 0;
    for (y,x) in iproduct!(0..len_x as i32, 0..len_y as i32) {
        if map.get(x,y) == 0u8 {
            let mut targets = HashSet::new();
            walk_towards_9(x, y, &map, &mut targets);
            println!("({:?},{:?}) {:?}", x,y, targets.len());
            count1 += targets.len();
        } else if map.get(x,y) == 9u8 {
            walk_towards_0(x, y, &map, &mut count2);
        }
    }

    println!("task1: {:?}", count1);
    println!("task2: {:?}", count2);

}

const DIRECTIONS: [(i32,i32); 4] = [
             (0,-1),
    (-1, 0),         (1, 0),
             (0, 1)
];

fn walk_towards_9(x: i32, y: i32, map : &Map, targets: &mut HashSet<(i32,i32)>) {
    for (dx, dy) in DIRECTIONS {
        let (nx, ny) = (x + dx, y + dy);
        let next = map.get(nx, ny);
        if next == map.get(x,y)+1 {
            if next == 9u8 {
                targets.insert((nx, ny));
            }else{
                walk_towards_9(nx, ny, map, targets);
            }
        }
    }
}

fn walk_towards_0(x: i32, y: i32, map : &Map, count: &mut usize) {
    for (dx, dy) in DIRECTIONS {
        let (nx, ny) = (x + dx, y + dy);
        let next = map.get(nx, ny);
        if next == map.get(x,y)-1 {
            if next == 0u8 {
                *count += 1;
            }else{
                walk_towards_0(nx, ny, map, count);
            }
        }
    }
}


