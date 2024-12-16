use std::collections::{HashMap, HashSet};

use itertools::iproduct;

const NEIGHBOURS: [((isize, isize), char); 4] = [
    ((-1, 0), '<'), // left
    ((0, 1), '^'),  // up
    ((1, 0), '>'),  // right
    ((0, -1), 'v'), // down
];

type XY = (usize, usize);
type FieldMemento = (XY, Orientation);
type Field = Vec<Vec<char>>;
type Orientation = char;
fn main() {
    let field = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let (h, w) = (field.len(), field[0].len());

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (x, y) in iproduct!(0..h, 0..w) {
        if field[y][x] == 'S' {
            start = (x, y);
        } else if field[y][x] == 'E' {
            end = (x, y);
        }
    }
    assert_ne!(start, (0, 0));
    assert_ne!(end, (0, 0));

    let mut best = usize::MAX;
    let mut path = Vec::new();
    let mut best_fields = HashSet::new();
    solve(
        start,
        '>',
        &field,
        0,
        &mut HashMap::new(),
        &mut best,
        end,
        &mut path,
        &mut best_fields,
    );

    best_fields.insert(start);

    println!("Best: {}", best);
    println!("Best fields: {:?}", best_fields.len());
}

fn solve(
    pos: XY,
    orientation: Orientation,
    field: &Field,
    score: usize,
    seen: &mut HashMap<FieldMemento, usize>,
    best: &mut usize,
    end: XY,
    path: &mut Vec<XY>,
    best_fields: &mut HashSet<XY>,
) {
    if score > *best {
        return;
    }
    
    let (x, y) = pos;
    //check if we have already seen this position with a better score
    if seen.get(&(pos, orientation))
        .map(|seen_score| *seen_score < score).unwrap_or(false) {
            return;
    }
    seen.insert((pos, orientation), score);
    if field[y][x] == 'E' {
        // found a solution
        if score < *best {
            best_fields.clear();
        }
        best_fields.extend(path.iter());
        *best = score;
        return;
    }

    let mut next = get_neighbours(pos, orientation, field);
    next.sort_by_key(|(pos, _, _)| pos.0.abs_diff(end.0) + pos.1.abs_diff(end.1));
    for ((xx, yy), o, turns) in next {
        path.push((xx, yy));
        solve(
            (xx, yy),
            o,
            field,
            score + (turns * 1000) + 1,
            seen,
            best,
            end,
            path,
            best_fields,
        );
        path.pop();
    }
}

fn get_neighbours(
    pos: XY,
    orientation: Orientation,
    field: &Field,
) -> Vec<(XY, Orientation, usize)> {
    let (x, y) = pos;
    let mut res = Vec::new();
    for ((dx, dy), o) in NEIGHBOURS {
        let (xx, yy) = ((x as isize + dx) as usize, (y as isize + dy) as usize);

        if field[yy][xx] == '#' {
            continue;
        }
        let turns = get_turns(orientation, o);

        res.push(((xx, yy), o, turns));
    }
    res
}

fn get_turns(current: Orientation, target: Orientation) -> usize {
    if current == target {
        return 0;
    }
    match current {
        '>' | '<' => match target {
            '^' => 1,
            'v' => 1,
            _ => 2,
        },
        '^' | 'v' => match target {
            '>' => 1,
            '<' => 1,
            _ => 2,
        },
        _ => unreachable!(),
    }
}
