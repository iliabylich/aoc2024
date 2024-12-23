use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[cfg(test)]
const N: usize = 16;
#[cfg(not(test))]
const N: usize = 520;

type Graph = [[bool; N]; N];

fn has_edge(graph: &Graph, n1: usize, n2: usize) -> bool {
    graph[n1][n2] || graph[n2][n1]
}

fn extend_largest_full_subgraph0(
    graph: &Graph,
    current_subgraph: &mut Vec<usize>,
    best_subgraph: &mut Vec<usize>,
) {
    if current_subgraph.len() > best_subgraph.len() {
        *best_subgraph = current_subgraph.clone();
    }

    let start_from_node = current_subgraph.iter().copied().last().unwrap_or(0);

    for new_node in start_from_node..N {
        if current_subgraph
            .iter()
            .copied()
            .all(|existing_node| has_edge(graph, new_node, existing_node))
        {
            current_subgraph.push(new_node);
            extend_largest_full_subgraph0(graph, current_subgraph, best_subgraph);
            current_subgraph.pop();
        }
    }
}

fn largest_full_subgraph(graph: &[[bool; N]; N]) -> Vec<usize> {
    let mut buf = vec![];
    let mut best = vec![];

    extend_largest_full_subgraph0(graph, &mut buf, &mut best);

    best
}

struct Pool<'a> {
    inner: Vec<&'a str>,
}

impl<'a> Pool<'a> {
    fn new() -> Self {
        Self { inner: vec![] }
    }

    fn add(&mut self, s: &'a str) -> usize {
        if let Some(idx) = self.inner.iter().position(|e| *e == s) {
            idx
        } else {
            self.inner.push(s);
            self.inner.len() - 1
        }
    }

    fn get(&self, n: usize) -> &'a str {
        self.inner.get(n).unwrap()
    }
}

fn solve(input: &str) -> String {
    let input = input.trim();

    let mut pool = Pool::new();

    let mut graph = [[false; N]; N];

    for line in input.lines() {
        let (n1, n2) = line.split_once('-').unwrap();
        let n1 = pool.add(n1);
        let n2 = pool.add(n2);

        graph[n1][n2] = true;
        graph[n2][n1] = true;
    }

    let largest = largest_full_subgraph(&graph);

    largest.into_iter().map(|n| pool.get(n)).sorted().join(",")
}

#[test]
fn test1() {
    let input = include_str!("input_test.txt");
    let output = solve(input);
    assert_eq!(output, "co,de,ka,ta");
}
