use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

fn solve(input: &str) -> usize {
    let input = input.trim();
    let mut edges = HashSet::<(&str, &str)>::new();
    let mut nodes = HashSet::<&str>::new();

    for line in input.lines() {
        let (n1, n2) = line.split_once('-').unwrap();
        edges.insert((n1, n2));
        edges.insert((n2, n1));
        nodes.insert(n1);
        nodes.insert(n2);
    }

    let has_edge = |n1: &str, n2: &str| edges.contains(&(n1, n1)) || edges.contains(&(n2, n1));

    println!("nodes count: {:?}", nodes.len());
    println!("edges count: {:?}", edges.len());

    nodes
        .iter()
        .tuple_combinations()
        .filter(|(n1, n2, n3)| n1.starts_with('t') || n2.starts_with('t') || n3.starts_with('t'))
        .filter(|(n1, n2, n3)| has_edge(n1, n2) && has_edge(n2, n3) && has_edge(n3, n1))
        .unique()
        .count()
}

#[test]
fn test1() {
    let input = include_str!("input_test.txt");
    let output = solve(input);
    assert_eq!(output, 7);
}
