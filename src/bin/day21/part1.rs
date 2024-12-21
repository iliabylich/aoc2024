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

    fn iteration_using(&self, f: impl Fn(char, char) -> Choice) -> Vec<Code> {
        let mut choices = vec![];
        for (l, r) in format!("A{}", self.value).chars().tuple_windows() {
            choices.push(f(l, r));
        }

        let mut out = vec![String::new()];

        for choice in choices {
            match choice {
                Choice::Single(s) => {
                    for item in &mut out {
                        *item = format!("{item}{s}");
                    }
                }
                Choice::Double(one, two) => {
                    out = out
                        .iter()
                        .flat_map(|item| [format!("{item}{one}"), format!("{item}{two}")])
                        .collect::<Vec<_>>();
                }
            }
        }

        out.into_iter().map(Code::new).collect()
    }

    fn human_iteration(&self) -> Vec<Code> {
        self.iteration_using(best_dir_on_numeric)
    }

    fn robot_iteration(&self) -> Vec<Code> {
        self.iteration_using(best_dir_on_directional)
    }
}

fn solve(input: &str) -> usize {
    let codes = input.trim().lines().map(Code::new).collect::<Vec<_>>();
    let mut out = 0;

    for code in codes {
        println!("==== {}", code.value);
        let codes = code.human_iteration();
        println!("starting codes {:?}", codes);

        let codes1 = codes
            .into_iter()
            .flat_map(|code| code.robot_iteration())
            .collect::<Vec<_>>();

        let codes2 = codes1
            .into_iter()
            .flat_map(|code| code.robot_iteration())
            .collect::<Vec<_>>();

        let lhs = codes2.iter().map(|code| code.value.len()).min().unwrap();
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

fn best_dir_on_numeric(d1: char, d2: char) -> Choice {
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

fn best_dir_on_directional(l: char, r: char) -> Choice {
    let single = Choice::Single;
    let double = |one, two| Choice::Double(one, two);

    match (l, r) {
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

        _ if l == r => single("A"),
        _ => todo!("{l} {r}"),
    }
}
