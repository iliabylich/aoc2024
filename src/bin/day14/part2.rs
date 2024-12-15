use crossterm::{
    cursor,
    style::{self},
    terminal, QueueableCommand,
};
use std::{
    collections::HashMap,
    io::{self, Stdout, Write},
};

fn main() {
    let input = include_str!("input.txt");
    solve(input, 103, 101);
}

#[derive(Debug, Clone, Copy)]
struct Location {
    row: i64,
    col: i64,
}

impl Location {
    fn fix(self, rows_count: u64, cols_count: u64) -> Self {
        fn clamp(mut value: i64, max: u64) -> i64 {
            while value < 0 {
                value += max as i64;
            }
            while value >= max as i64 {
                value -= max as i64;
            }
            value
        }

        Self {
            row: clamp(self.row, rows_count),
            col: clamp(self.col, cols_count),
        }
    }

    fn add(self, drow: i64, dcol: i64) -> Self {
        Self {
            row: self.row + drow,
            col: self.col + dcol,
        }
    }
}

#[derive(Debug)]
struct Robot {
    current_location: Location,
    speed: (i64, i64),
    rows_count: u64,
    cols_count: u64,
}

impl Robot {
    fn parse(line: &str, rows_count: u64, cols_count: u64) -> Self {
        let (p, v) = line.split_once(' ').unwrap();
        let p = p.strip_prefix("p=").unwrap();
        let v = v.strip_prefix("v=").unwrap();

        fn parse_i64_i64(s: &str) -> (i64, i64) {
            let (col, row) = s.split_once(',').unwrap();
            (row.parse().unwrap(), col.parse().unwrap())
        }

        let p = parse_i64_i64(p);
        let v = parse_i64_i64(v);

        let starting_loc = Location { row: p.0, col: p.1 };
        let speed = v;

        Self {
            current_location: starting_loc,
            speed,
            rows_count,
            cols_count,
        }
    }

    fn step(&mut self) {
        self.current_location = self
            .current_location
            .add(self.speed.0, self.speed.1)
            .fix(self.rows_count, self.cols_count)
    }
}

#[allow(dead_code)]
fn print_robots(
    robots: &[Robot],
    rows_count: u64,
    cols_count: u64,
    seconds: u64,
    stdout: &mut Stdout,
) {
    let mut text = format!("Iteration: {seconds}\n");

    for row in 0..rows_count {
        for col in 0..cols_count {
            let n = robots
                .iter()
                .filter(|r| {
                    let loc = r.current_location;
                    loc.row == row as i64 && loc.col == col as i64
                })
                .count();

            if n == 0 {
                text.push('.');
            } else {
                text.push('1')
            }
        }
        text.push('\n');
    }

    stdout
        .queue(terminal::Clear(terminal::ClearType::All))
        .unwrap()
        .queue(cursor::MoveTo(0, 0))
        .unwrap()
        .queue(style::Print(text))
        .unwrap();
    stdout.flush().unwrap();
}

fn heuristic1(robots: &[Robot]) -> bool {
    let mut map = HashMap::<i64, Vec<i64>>::new();

    for robot in robots {
        map.entry(robot.current_location.col)
            .or_default()
            .push(robot.current_location.row);
    }

    for (_, mut value) in map.into_iter() {
        value.sort_unstable();

        if value
            .windows(10)
            .any(|w| w.iter().zip(w.iter().skip(1)).all(|(a, b)| *a + 1 == *b))
        {
            return true;
        }
    }

    false
}

fn solve(input: &str, rows_count: u64, cols_count: u64) {
    let mut robots = input
        .trim()
        .lines()
        .map(|l| Robot::parse(l, rows_count, cols_count))
        .collect::<Vec<_>>();

    let mut stdout = io::stdout();

    let mut seconds = 0;
    loop {
        for robot in &mut robots {
            robot.step();
        }
        seconds += 1;

        if !heuristic1(&robots) {
            continue;
        }

        print_robots(&robots, rows_count, cols_count, seconds, &mut stdout);
        break;
    }
}
