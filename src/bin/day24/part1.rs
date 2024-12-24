use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
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

    #[allow(dead_code)]
    fn get(&self, n: usize) -> &'a str {
        self.inner.get(n).unwrap()
    }

    fn zs(&self) -> Vec<(usize, &'a str)> {
        let mut out = self
            .inner
            .iter()
            .copied()
            .enumerate()
            .filter(|(_idx, s)| s.starts_with('z'))
            .collect::<Vec<_>>();

        out.sort_unstable_by_key(|(_idx, s)| *s);
        out.reverse();

        out
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Gate {
    And { lhs: usize, rhs: usize, out: usize },
    Or { lhs: usize, rhs: usize, out: usize },
    Xor { lhs: usize, rhs: usize, out: usize },
}

impl Gate {
    fn parse<'a>(input: &'a str, pool: &mut Pool<'a>) -> Self {
        let (lhs_plus_rhs, out) = input.split_once(" -> ").unwrap();
        let mut it = lhs_plus_rhs.split(' ');

        let lhs = it.next().unwrap();
        let op = it.next().unwrap();
        let rhs = it.next().unwrap();
        assert_eq!(it.next(), None);

        let lhs = pool.add(lhs);
        let rhs = pool.add(rhs);
        let out = pool.add(out);

        match op {
            "AND" => Self::And { lhs, rhs, out },
            "OR" => Self::Or { lhs, rhs, out },
            "XOR" => Self::Xor { lhs, rhs, out },
            _ => panic!("Wrong op: {op}"),
        }
    }

    fn lhs(self) -> usize {
        match self {
            Gate::And { lhs, .. } => lhs,
            Gate::Or { lhs, .. } => lhs,
            Gate::Xor { lhs, .. } => lhs,
        }
    }

    fn rhs(self) -> usize {
        match self {
            Gate::And { rhs, .. } => rhs,
            Gate::Or { rhs, .. } => rhs,
            Gate::Xor { rhs, .. } => rhs,
        }
    }

    fn out(self) -> usize {
        match self {
            Gate::And { out, .. } => out,
            Gate::Or { out, .. } => out,
            Gate::Xor { out, .. } => out,
        }
    }
}

#[derive(Debug)]
struct System {
    wires: HashMap<usize, bool>,
    gates: HashSet<Gate>,
    all_known_wires: HashSet<usize>,
}

impl System {
    fn parse<'a>(input: &'a str, pool: &mut Pool<'a>) -> Self {
        let (wires_s, gates_s) = input.trim().split_once("\n\n").unwrap();

        let mut wires = HashMap::new();
        let mut gates = HashSet::new();
        let mut all_known_wires = HashSet::new();

        for wire in wires_s.lines() {
            let (wire, value) = wire.split_once(": ").unwrap();
            let wire = pool.add(wire);
            let value = value.parse::<u8>().unwrap();
            assert!(value == 0 || value == 1);
            wires.insert(wire, value == 1);
            all_known_wires.insert(wire);
        }

        for gate in gates_s.lines() {
            let gate = Gate::parse(gate, pool);
            all_known_wires.insert(gate.lhs());
            all_known_wires.insert(gate.rhs());
            all_known_wires.insert(gate.out());
            gates.insert(gate);
        }

        Self {
            wires,
            gates,
            all_known_wires,
        }
    }

    fn gate_that_has_prerequisites(&self) -> Option<Gate> {
        for gate in &self.gates {
            let lhs = gate.lhs();
            let rhs = gate.rhs();
            if self.wires.contains_key(&lhs) && self.wires.contains_key(&rhs) {
                return Some(*gate);
            }
        }
        None
    }

    fn fill(&mut self) {
        while let Some(gate) = self.gate_that_has_prerequisites() {
            match gate {
                Gate::And { lhs, rhs, out } => {
                    let lhs_v = self.wires.get(&lhs).unwrap();
                    let rhs_v = self.wires.get(&rhs).unwrap();
                    let out_v = *lhs_v & *rhs_v;
                    self.wires.insert(out, out_v);
                }
                Gate::Or { lhs, rhs, out } => {
                    let lhs_v = self.wires.get(&lhs).unwrap();
                    let rhs_v = self.wires.get(&rhs).unwrap();
                    let out_v = *lhs_v | *rhs_v;
                    self.wires.insert(out, out_v);
                }
                Gate::Xor { lhs, rhs, out } => {
                    let lhs_v = self.wires.get(&lhs).unwrap();
                    let rhs_v = self.wires.get(&rhs).unwrap();
                    let out_v = *lhs_v ^ *rhs_v;
                    self.wires.insert(out, out_v);
                }
            }

            self.gates.remove(&gate);
        }

        assert!(self.gates.is_empty());
        for wire in &self.all_known_wires {
            assert!(self.wires.contains_key(wire));
        }
    }
}

fn solve(input: &str) -> u64 {
    let mut pool = Pool::new();
    let mut system = System::parse(input, &mut pool);
    system.fill();

    let bytes = pool
        .zs()
        .into_iter()
        .map(|(idx, wire)| {
            let value = *system.wires.get(&idx).unwrap();
            println!("{wire}: {value}");
            if value {
                '1'
            } else {
                '0'
            }
        })
        .collect::<String>();

    u64::from_str_radix(&bytes, 2).unwrap()
}

#[test]
fn test1() {
    let input = include_str!("input_test1.txt");
    let output = solve(input);
    assert_eq!(output, 4);
}

#[test]
fn test2() {
    let input = include_str!("input_test2.txt");
    let output = solve(input);
    assert_eq!(output, 2024);
}
