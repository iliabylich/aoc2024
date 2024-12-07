fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Plus,
    Multiply,
}

#[derive(Debug)]
struct Equation {
    total: usize,
    numbers: Vec<usize>,
}

impl Equation {
    fn parse(line: &str) -> Self {
        let (total, rest) = line.split_once(": ").unwrap();
        let total = total.parse::<usize>().unwrap();
        let numbers = rest
            .split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Self { numbers, total }
    }

    fn solve(&self) -> Option<Vec<Operator>> {
        let mut combinations = vec![];
        all_combinations(self.numbers.len() - 1, vec![], &mut combinations);

        combinations
            .into_iter()
            .find(|combination| self.is_valid(combination))
    }

    fn is_valid(&self, operators: &[Operator]) -> bool {
        let mut result = self.numbers[0];
        assert_eq!(operators.len(), self.numbers.len() - 1);
        for (op, number) in operators.iter().zip(self.numbers.iter().skip(1)) {
            match op {
                Operator::Plus => result += *number,
                Operator::Multiply => result *= number,
            }
        }
        result == self.total
    }
}

fn all_combinations(n: usize, mut buf: Vec<Operator>, out: &mut Vec<Vec<Operator>>) {
    if buf.len() == n {
        return;
    }

    for next_operator in [Operator::Multiply, Operator::Plus] {
        buf.push(next_operator);
        if buf.len() == n {
            out.push(buf.clone());
        }
        all_combinations(n, buf.clone(), out);
        buf.pop();
    }
}

fn solve(input: &str) -> usize {
    let equations = input
        .trim()
        .split("\n")
        .map(Equation::parse)
        .collect::<Vec<_>>();

    let mut out = 0;

    for equation in equations {
        if let Some(_solution) = equation.solve() {
            out += equation.total
        }
    }

    out
}

#[test]
fn test() {
    let input = include_str!("input1_test.txt");
    let output: usize = solve(input);
    assert_eq!(output, 3749);
}
