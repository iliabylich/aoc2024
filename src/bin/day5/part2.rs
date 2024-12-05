use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Debug)]
struct Graph {
    edges: [[bool; 100]; 100],
    nodes: HashSet<usize>,
}

impl Graph {
    fn new(input: &str) -> Self {
        let mut edges = [[false; 100]; 100];
        let mut nodes = HashSet::new();
        for line in input.lines() {
            let (before, after) = line.split_once('|').unwrap();
            let before = before.parse::<usize>().unwrap();
            let after = after.parse::<usize>().unwrap();
            edges[before][after] = true;
            nodes.insert(before);
            nodes.insert(after);
        }
        Self { edges, nodes }
    }

    fn connected(&self, before: usize, after: usize) -> bool {
        self.edges[before][after]
    }

    fn subgraph(&self, nodes_to_take: &[usize]) -> Self {
        let mut edges = [[false; 100]; 100];
        let nodes = HashSet::from_iter(nodes_to_take.iter().copied());
        for node1 in nodes_to_take {
            for node2 in nodes_to_take {
                edges[*node1][*node2] = self.edges[*node1][*node2];
                edges[*node2][*node1] = self.edges[*node2][*node1];
            }
        }
        Self { edges, nodes }
    }

    fn incoming_edges(&self, node: usize) -> Vec<usize> {
        let mut out = vec![];
        for other in self.nodes.iter() {
            if self.edges[*other][node] {
                out.push(*other);
            }
        }
        out
    }

    fn outcoming_edges(&self, node: usize) -> Vec<usize> {
        let mut out = vec![];
        for other in self.nodes.iter() {
            if self.edges[node][*other] {
                out.push(*other);
            }
        }
        out
    }

    fn has_incoming_edges(&self, node: usize) -> bool {
        !self.incoming_edges(node).is_empty()
    }

    fn nodes_with_no_incoming_edges(&self) -> Vec<usize> {
        self.nodes
            .iter()
            .filter(|node| !self.has_incoming_edges(**node))
            .copied()
            .collect()
    }

    fn tsort(mut self) -> Vec<usize> {
        let mut l = vec![];
        let mut s = HashSet::from_iter(self.nodes_with_no_incoming_edges());

        while let Some(n) = s.sample() {
            l.push(n);

            for m in self.outcoming_edges(n) {
                self.edges[n][m] = false;
                if self.incoming_edges(m).is_empty() {
                    s.insert(m);
                }
            }
        }

        for from in 0..100 {
            for to in 0..100 {
                if self.edges[from][to] {
                    panic!("cycle detected, there's still an edge {from} -> {to}");
                }
            }
        }

        l
    }
}

trait HashSetExt<T: Copy + Eq + std::hash::Hash> {
    fn sample(&mut self) -> Option<T>;
}

impl<T: Copy + Eq + std::hash::Hash> HashSetExt<T> for HashSet<T> {
    fn sample(&mut self) -> Option<T> {
        let elem = *self.iter().next()?;
        self.remove(&elem);
        Some(elem)
    }
}

#[derive(Debug)]
struct Pages {
    pages: Vec<usize>,
}

impl Pages {
    fn new(line: &str) -> Self {
        Self {
            pages: line
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect(),
        }
    }

    fn is_valid(&self, graph: &Graph) -> bool {
        self.pages
            .iter()
            .zip(self.pages.iter().skip(1))
            .all(|(before, after)| graph.connected(*before, *after))
    }

    fn fix(&mut self, graph: &Graph) {
        let subgraph = graph.subgraph(&self.pages);
        let sorted = subgraph.tsort();
        self.pages = sorted;
    }

    fn middle(&self) -> usize {
        assert!(self.pages.len() % 2 != 0);
        self.pages[self.pages.len() / 2]
    }
}

fn solve(input: &str) -> usize {
    let (ordering, pages) = input.trim().split_once("\n\n").unwrap();

    let graph = Graph::new(ordering);

    let mut out = 0;
    for pages in pages.lines() {
        let mut pages = Pages::new(pages);
        if !pages.is_valid(&graph) {
            pages.fix(&graph);
            out += pages.middle();
        }
    }

    out
}

#[test]
fn test() {
    let input = include_str!("input_test.txt");
    let output = solve(input);
    assert_eq!(output, 123);
}
