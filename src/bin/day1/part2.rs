use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

fn solve(input: &str) -> usize {
    let mut left = vec![];
    let mut right = HashMap::<usize, usize>::new();
    for line in input.lines() {
        let (l, r) = line.split_once("   ").unwrap();
        let l = l.parse::<usize>().unwrap();
        let r = r.parse::<usize>().unwrap();
        left.push(l);

        *right.entry(r).or_default() += 1;
    }

    left.into_iter()
        .map(|l| l * right.get(&l).copied().unwrap_or_default())
        .sum()
}

#[test]
fn test() {
    let input = include_str!("input2_test.txt");
    let output = solve(input);
    assert_eq!(output, 31);
}
