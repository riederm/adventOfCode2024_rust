use std::collections::HashSet;

use multimap::MultiMap;

fn main() {
    let buyers = include_str!("input.txt")
        .lines()
        .map(|it| Buyer::new(it.parse::<i64>().unwrap(), 2000))
        .collect::<Vec<_>>();

    println!(
        "Sum of secrets: {}",
        buyers.iter().map(|b| b.secret).sum::<i64>()
    );

    // part b
    // collect a multimap with 4 changes sequence as key, and first price after that sequence as value
    let sequences = &mut MultiMap::new();
    buyers.iter().for_each(|b| b.collect_sequences(sequences));

    // find the max sum of bananas
    let max = sequences
        .iter_all()
        .map(|(_, v)| v.iter().map(|&it| it as u32).sum::<u32>())
        .max()
        .unwrap();

    println!("Max bananas: {}", max);
}

fn calculate_secret(secret: i64) -> i64 {
    fn mix(value: i64, secret: i64) -> i64 {
        return value ^ secret;
    }

    fn prune(value: i64) -> i64 {
        return value % 16777216;
    }

    let secret = prune(mix(secret * 64, secret));
    let secret = prune(mix(secret / 32, secret));
    return prune(mix(secret * 2048, secret));
}

struct Buyer {
    prev_secret: i64,
    secret: i64,
    history_changes: Vec<i8>,
    history_prices: Vec<u8>,
}

impl Buyer {
    fn new(secret: i64, times: usize) -> Buyer {
        let mut b = Buyer {
            secret,
            prev_secret: secret,
            history_changes: Vec::with_capacity(times),
            history_prices: Vec::with_capacity(times),
        };
        for _ in 0..times {
            b.prev_secret = b.secret;
            b.secret = calculate_secret(b.secret);
            b.history_prices.push((b.secret % 10) as u8);
            b.history_changes.push((b.secret % 10 - b.prev_secret % 10) as i8);
        }
        b
    }

    fn collect_sequences(&self, sequences: &mut MultiMap<(i8, i8, i8, i8), u8>) {
        let mut my_sequences = HashSet::new();
        self.history_changes
            .windows(4)
            .enumerate()
            .for_each(|(i, window)| {
                let k = (window[0], window[1], window[2], window[3]);
                // make sure we only insert the first price after the sequence
                if my_sequences.insert(k) {
                    sequences.insert(k, self.history_prices[i + 3]);
                }
            });
    }
}
