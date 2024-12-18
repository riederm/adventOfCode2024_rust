use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}};

fn main() {
    let obstacles = include_str!("input.txt")
        .lines()
        .map(|line| line.split_once(",").map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())).unwrap())
        .collect::<Vec<_>>();

    let obs = obstacles.iter().enumerate()
        .map(|(i, (x, y))| ((*x, *y),i))
        .collect::<HashMap<_,_>>();


    let result = dijkstra((0,0), (70,70), &obs, 71, 71, 1024);
    println!("task1: {:?}", result);

    let (mut min, mut max) = (0, obs.len());




    loop {
        let mid = (min + max) / 2;
        let result = dijkstra((0,0), (70,70), &obs, 71, 71, mid);

        match result {
            Some(_) => {
                min = mid;
            },
            None => {
                max = mid;
            }
        }

        if max-min ==1 {
            break;
        }
    }

    println!("task2: {:?}", obstacles[max-1]);

    dbg!(dijkstra((0,0), (70,70), &obs, 71, 71, max-1));
    dbg!(dijkstra((0,0), (70,70), &obs, 71, 71, max));



}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: (i32, i32),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(start: (i32, i32), goal: (i32, i32), obstacles: &HashMap<(i32, i32), usize>, width: i32, height: i32, time: usize) -> Option<i32> {
    let mut dist: HashMap<(i32, i32), i32> = HashMap::new();
    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist.insert(start, 0);
    heap.push(State { cost: 0, position: start });

    // While there are still nodes to be checked
    while let Some(State { cost, position }) = heap.pop() {
        // If we reached the goal, return the cost
        if position == goal {
            return Some(cost);
        }

        // If the cost is greater than the stored cost, skip it
        if cost > *dist.get(&position).unwrap_or(&i32::MAX) {
            continue;
        }

        // For each possible movement (up, down, left, right)
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let next = (position.0 + dx, position.1 + dy);
            let next_cost = cost + 1;

            // If the next position is within bounds and not an obstacle
            if next.0 >= 0 && next.0 < width && next.1 >= 0 && next.1 < height && obstacles.get(&next).unwrap_or(&usize::MAX) >= &time {
                // If the next position is cheaper, update it
                if next_cost < *dist.get(&next).unwrap_or(&i32::MAX) {
                    heap.push(State { cost: next_cost, position: next });
                    dist.insert(next, next_cost);
                }
            }
        }
    }

    // Goal not reachable
    None
}