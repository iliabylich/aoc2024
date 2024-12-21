use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Code {
    value: String,
}

impl Code {
    fn new(s: impl Into<String>) -> Self {
        Self { value: s.into() }
    }

    fn numeric_part(&self) -> usize {
        self.value
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap()
    }

    fn human_iteration(&self) -> Vec<RobotCode> {
        fn best_choice_on_numeric_pad(d1: char, d2: char) -> Choice {
            let single = Choice::Single;
            let double = |one, two| Choice::Double(one, two);

            match (d1, d2) {
                ('A', '0') => single("<A"),
                ('0', '2') => single("^A"),
                ('2', '9') => double("^^>A", ">^^A"),
                ('9', 'A') => single("vvvA"),

                ('A', '9') => single("^^^A"),
                ('9', '8') => single("<A"),
                ('8', '0') => single("vvvA"),
                ('0', 'A') => single(">A"),

                ('A', '1') => single("^<<A"),
                ('1', '7') => single("^^A"),
                ('7', '9') => single(">>A"),

                ('A', '4') => single("^^<<A"),
                ('4', '5') => single(">A"),
                ('5', '6') => single(">A"),
                ('6', 'A') => single("vvA"),

                ('A', '3') => single("^A"),
                ('3', '7') => double("^^<<A", "<<^^A"),
                // ('3', '7') => single("<<^^A"),

                // actual test
                ('4', '8') => double(">^A", "^>A"),
                ('A', '6') => single("^^A"),
                ('6', '8') => double("^<A", "<^A"),
                ('8', '2') => single("vvA"),
                ('2', 'A') => double("v>A", ">vA"),
                ('1', '4') => single("^A"),
                ('4', '0') => single(">vvA"),
                ('A', '2') => double("^<A", "<^A"),
                ('2', '4') => double("^<A", "<^A"),
                ('4', '6') => single(">>A"),
                ('9', '3') => single("vvA"),
                ('3', '8') => double("^^<A", "<^^A"),
                ('8', 'A') => double("vvv>A", ">vvvA"),

                _ => todo!("{d1} {d2}"),
            }
        }

        let mut choices = vec![];
        for (l, r) in format!("A{}", self.value).chars().tuple_windows() {
            choices.push(best_choice_on_numeric_pad(l, r));
        }

        let mut out = vec![RobotCode(vec![])];

        for choice in choices {
            match choice {
                Choice::Single(s) => {
                    for item in &mut out {
                        item.0.push(Fragment(s));
                    }
                }
                Choice::Double(one, two) => {
                    let copy = out.clone();
                    out.clear();
                    for item in copy {
                        let mut version1 = item.clone();
                        version1.0.push(Fragment(one));

                        let mut version2 = item;
                        version2.0.push(Fragment(two));
                        out.push(version1);
                        out.push(version2);
                    }
                }
            }
        }

        out
    }
}

#[derive(Debug, Clone)]
struct RobotCode(Vec<Fragment>);

impl RobotCode {
    fn min_length_after_n_generations(&self, n: usize, cache: &mut Cache) -> usize {
        self.0
            .iter()
            .map(|fragment| {
                min_length_of_fragment_after_n_generations_assuming_it_goes_after(
                    *fragment, n, cache,
                )
            })
            .sum()
    }
}

fn solve(input: &str) -> usize {
    let codes = input.trim().lines().map(Code::new).collect::<Vec<_>>();
    let mut out = 0;

    let mut cache = Cache::new();

    for code in codes {
        println!("==== {}", code.value);
        let codes = code.human_iteration();
        println!("starting codes {:?}", codes);

        let lhs = codes
            .into_iter()
            .map(|code| code.min_length_after_n_generations(25, &mut cache))
            .min()
            .unwrap();
        let rhs = code.numeric_part();
        let score = lhs * rhs;
        println!("{:?}: {} * {} = {}", code, lhs, rhs, score);
        out += score;
    }
    out
}

#[test]
fn test1() {
    let input = include_str!("input_test.txt");
    let output = solve(input);
    assert_eq!(output, 126384);
}

#[derive(Debug)]
enum Choice {
    Single(&'static str),
    Double(&'static str, &'static str),
}

fn best_choice_on_directional_pad(from: char, to: char) -> Choice {
    let single = Choice::Single;
    let double = |one, two| Choice::Double(one, two);

    match (from, to) {
        ('<', 'A') => single(">>^A"),
        ('A', '^') => single("<A"),
        ('^', 'A') => single(">A"),
        ('^', '>') => double("v>A", ">vA"),
        ('>', 'A') => single("^A"),
        ('A', 'v') => double("v<A", "<vA"),
        ('v', 'A') => double("^>A", ">^A"),
        ('A', '<') => single("v<<A"),
        ('A', '>') => single("vA"),
        ('>', '^') => double("<^A", "^<A"),
        ('^', '<') => single("v<A"),
        ('<', '^') => single(">^A"),
        ('v', '<') => single("<A"),
        ('v', '>') => single(">A"),
        ('<', 'v') => single(">A"),
        ('>', 'v') => single("<A"),

        _ if from == to => single("A"),
        _ => todo!("{from} {to}"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Fragment(&'static str);

impl Fragment {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn next_gen(&self) -> Vec<Vec<Self>> {
        let mut choices = vec![];
        for (l, r) in format!("A{}", self.0).chars().tuple_windows() {
            choices.push(best_choice_on_directional_pad(l, r));
        }

        let mut out = vec![vec![]];

        for choice in choices {
            match choice {
                Choice::Single(s) => {
                    for version in &mut out {
                        version.push(Fragment(s));
                    }
                }
                Choice::Double(one, two) => {
                    let copy = out.clone();
                    out.clear();
                    for item in copy {
                        let mut version1 = item.clone();
                        version1.push(Fragment(one));

                        let mut version2 = item;
                        version2.push(Fragment(two));
                        out.push(version1);
                        out.push(version2);
                    }
                }
            }
        }

        out
    }
}

type Cache = HashMap<(Fragment, usize), usize>;

fn min_length_of_fragment_after_n_generations_assuming_it_goes_after(
    fragment: Fragment,
    n: usize,
    cache: &mut Cache,
) -> usize {
    if n == 0 {
        return fragment.len();
    }

    if let Some(length) = cache.get(&(fragment, n)) {
        return *length;
    }

    let mut min_length = usize::MAX;
    for gen in fragment.next_gen() {
        let mut length = 0;
        for fragment in gen {
            length += min_length_of_fragment_after_n_generations_assuming_it_goes_after(
                fragment,
                n - 1,
                cache,
            );
        }
        if length < min_length {
            min_length = length;
        }
    }

    cache.insert((fragment, n), min_length);

    min_length
}
