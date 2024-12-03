fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

fn solve(input: &str) -> usize {
    let re = regex::Regex::new(r#"mul\((\d+),(\d+)\)"#).unwrap();
    re.captures_iter(input)
        .map(|c| {
            let lhs = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let rhs = c.get(2).unwrap().as_str().parse::<usize>().unwrap();
            lhs * rhs
        })
        .sum()
}

#[test]
fn test() {
    let input = include_str!("input1_test.txt");
    let output = solve(input);
    assert_eq!(output, 161);
}
