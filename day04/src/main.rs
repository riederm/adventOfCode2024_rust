use std::vec;

use itertools::iproduct;

fn main() {
    let map = read_input();

    let task1 = task1(&map);
    println!("Task 1: {}", task1);

    let task2 = task2(&map);
    println!("Task 2: {}", task2);
}

fn read_input() -> Vec<Vec<char>> {
    let map: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|it| it.chars().collect())
        .collect();
    map
}

fn task1(map: &Vec<Vec<char>>) -> usize {
    let directions = vec![
        (1, 0),   //left
        (0, 1),   //down
        (-1, 0),  //right
        (0, -1),  //up
        (-1, -1), //up-left
        (1, -1),  //up-right
        (-1, 1),  //down-left
        (1, 1),   //down-right
    ];

    iproduct!(0..map[0].len() as i32, 0..map.len() as i32)
        .map(|(x, y)| {
            // try all directions from (x,y)
            directions
                .iter()
                .filter(|direction| {
                    // the direction hits, if all characters are found
                    (vec!['X', 'M', 'A', 'S']).iter().enumerate().all(|(i, c)| {
                        get_char(&map, x + direction.0 * i as i32, y + direction.1 * i as i32) == *c
                    })
                })
                .count()
        })
        .sum()
}

fn task2(map: &Vec<Vec<char>>) -> i32 {
    let words = vec![
        vec![('M', (-1, -1)), ('A', (0, 0)), ('S', (1, 1))], //top-left to bottom-right
        vec![('S', (-1, -1)), ('A', (0, 0)), ('M', (1, 1))], //top-left to bottom-right
        vec![('M', (-1, 1)), ('A', (0, 0)), ('S', (1, -1))], //bottom-left to top right
        vec![('S', (-1, 1)), ('A', (0, 0)), ('M', (1, -1))], //bottom-left to top right
    ];

    let mut count = 0;
    for (x, y) in iproduct!(0..map[0].len() as i32, 0..map.len() as i32) {
        // check if exactly two words are found from x,y
        if words
            .iter()
            .filter(|word| {
                word.iter()
                    .all(|(c, direction)| get_char(&map, x + direction.0, y + direction.1) == *c)
            })
            .count()
            == 2
        {
            count += 1;
        }
    }
    count
}

fn get_char(map: &Vec<Vec<char>>, x: i32, y: i32) -> char {
    if x < 0 || x >= map[0].len() as i32 || y < 0 || y >= map.len() as i32 {
        return ' ';
    }
    return map[y as usize][x as usize];
}

#[test]
fn test() {
    let input = read_input();
    assert_eq!((task1(&input), task2(&input)), (2549, 2003));
}
