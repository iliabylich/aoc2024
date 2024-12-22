use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Debug, Clone, Copy)]
struct Secret(u64);

fn mix(n: u64, other: u64) -> u64 {
    n ^ other
}

fn prune(n: u64) -> u64 {
    n % 16777216
}

impl Secret {
    fn next(self) -> Self {
        let n = self.0;

        let n = prune(mix(n, n * 64));
        let n = prune(mix(n, n / 32));
        let n = prune(mix(n, n * 2048));

        Self(n)
    }

    fn price(&self) -> Price {
        Price(self.0 % 10)
    }

    fn sequence_of_prices_and_changes(self) -> (Vec<Price>, Vec<Option<isize>>) {
        let mut secrets = vec![self];
        let mut prices = vec![self.price()];
        let mut changes = vec![None];

        let mut secret = self;
        for _ in 0..2000 {
            let next_secret = secret.next();
            let next_price = next_secret.price();
            let last_price = prices.last().copied().unwrap().0;
            let change = next_price.0 as isize - last_price as isize;

            secrets.push(next_secret);
            prices.push(next_price);
            changes.push(Some(change));

            secret = next_secret;
        }
        (prices, changes)
    }
}

type Seq = [isize; 4];
type SeqToFirstPrice = HashMap<Seq, u64>;

fn index_prices(prices: &[Price], changes: &[Option<isize>]) -> SeqToFirstPrice {
    let mut seq_to_first_price = SeqToFirstPrice::new();

    for (i, price) in prices.iter().enumerate() {
        let price = price.0;

        let get_change_at = |d: isize| -> Option<isize> {
            let idx = i.checked_add_signed(d)?;
            *changes.get(idx)?
        };
        let Some(change1) = get_change_at(-3) else {
            continue;
        };
        let Some(change2) = get_change_at(-2) else {
            continue;
        };
        let Some(change3) = get_change_at(-1) else {
            continue;
        };
        let Some(change4) = get_change_at(0) else {
            continue;
        };

        let seq: Seq = [change1, change2, change3, change4];

        seq_to_first_price.entry(seq).or_insert(price);
    }

    seq_to_first_price
}

#[derive(Debug, Clone, Copy)]
struct Price(u64);

fn solve(input: &str) -> u64 {
    let secrets = input
        .trim()
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .map(Secret)
        .collect::<Vec<_>>();

    let mut seq_to_best_price_indexes = vec![];
    let mut all_known_seqs = HashSet::<Seq>::new();

    for secret in secrets {
        let (prices, changes) = secret.sequence_of_prices_and_changes();
        let seq_to_best_price = index_prices(&prices, &changes);

        for seq in seq_to_best_price.keys() {
            all_known_seqs.insert(*seq);
        }

        seq_to_best_price_indexes.push(seq_to_best_price);
    }

    let mut max = 0;
    for seq in all_known_seqs {
        let sum = seq_to_best_price_indexes
            .iter()
            .map(|idx| idx.get(&seq).copied().unwrap_or_default())
            .sum::<u64>();

        if sum > max {
            max = sum;
        }
    }

    max
}

#[test]
fn test1() {
    let input = include_str!("input_test2.txt");
    let output = solve(input);
    assert_eq!(output, 23);
}
