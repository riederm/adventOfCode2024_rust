use std::{collections::VecDeque, u64};

#[derive(Debug, Clone)]
struct Block {
    start: u64,
    size: u64,
    id: Option<u64>,
}

impl Block {
    fn new(start: u64, size: u64, id: Option<u64>) -> Self {
        Block { start, size, id }
    }
}

fn main() {
    let input = include_str!("input.txt");

    let mut free = Vec::with_capacity(input.len() / 2);
    let mut files = Vec::with_capacity(input.len() / 2);

    let mut index = 0u64;
    for (i, n) in input
        .chars()
        .map(|it| it.to_digit(10).unwrap() as u64)
        .enumerate()
    {
        if i % 2 == 0 {
            files.push(Block::new(index, n, Some((i / 2) as u64)))
        } else {
            free.push(Block::new(index, n, None))
        }
        index += n;
    }

    task1(VecDeque::from(files.clone()), VecDeque::from(free.clone()));

    task2(files, free);
}

fn task1(mut files_list: VecDeque<Block>, mut free_list: VecDeque<Block>) {
    let mut defrag: Vec<Block> = Vec::with_capacity(files_list.len() * 2);

    while !files_list.is_empty() {
        if files_list.front().unwrap().start
            < free_list.front().map(|it| it.start).unwrap_or(u64::MAX)
        {
            defrag.push(files_list.pop_front().unwrap());
        } else {
            let next = files_list.pop_back().unwrap();
            let mut free = free_list.pop_front().unwrap();
            let remaining: i64 = (next.size as i64) - (free.size as i64);

            if remaining == 0 {
                //perfrect match
                defrag.push(Block::new(free.start, next.size, next.id));
            } else if remaining > 0 {
                // not everything fits
                defrag.push(Block::new(free.start, free.size, next.id));
                files_list.push_back(Block::new(next.start, remaining as u64, next.id));
            } else {
                // remaining <
                // plenty space left
                defrag.push(Block::new(free.start, next.size, next.id));
                free.size = remaining.abs() as u64;
                free.start += next.size;
                free_list.push_front(free);
            }
        }
    }

    let mut sum: u64 = 0;
    for file in defrag {
        let id = file.id.unwrap() as u64;
        for j in file.start..file.start + file.size {
            sum += ((j as u64) * id) as u64;
        }
    }
    println!("task1: {}", sum);
}

fn task2(mut files: Vec<Block>, mut free: Vec<Block>) {
    //try each file
    for f in files.iter_mut().rev() {
        // find the first free block that is big enough and has a lower index
        if let Some(free) = free
            .iter_mut()
            .find(|it| it.size >= f.size && it.start < f.start)
        {
            // move the file to the free block
            f.start = free.start;
            // reduce size of the free block
            free.size -= f.size;
            free.start += f.size as u64;
        }
    }

    let mut sum: u64 = 0;
    for file in files {
        let id = file.id.unwrap() as u64;
        for j in file.start..file.start + file.size as u64 {
            sum += ((j as u64) * id) as u64;
        }
    }
    println!("task2: {}", sum);
}
