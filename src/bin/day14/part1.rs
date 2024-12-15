use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("input.txt");
    let output = solve(input, 103, 101);
    println!("{}", output);
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
    starting_loc: Location,
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
            starting_loc,
            speed,
            rows_count,
            cols_count,
        }
    }

    fn location_after_seconds(&self, n: u64) -> Location {
        self.starting_loc
            .add(self.speed.0 * n as i64, self.speed.1 * n as i64)
            .fix(self.rows_count, self.cols_count)
    }
}

#[allow(dead_code)]
fn print_robots(robots: &[Robot], rows_count: u64, cols_count: u64, seconds: u64) {
    for row in 0..rows_count {
        for col in 0..cols_count {
            let n = robots
                .iter()
                .filter(|r| {
                    let loc = r.location_after_seconds(seconds);
                    loc.row == row as i64 && loc.col == col as i64
                })
                .count();
            if n == 0 {
                print!(".")
            } else {
                print!("{n}");
            }
        }
        println!();
    }
}

struct Quadrant {
    rows_spawn: RangeInclusive<u64>,
    cols_spawn: RangeInclusive<u64>,
}

impl Quadrant {
    fn split(n: u64) -> (u64, u64) {
        if n % 2 == 0 {
            (n / 2 - 1, n / 2)
        } else {
            (n / 2 - 1, n / 2 + 1)
        }
    }
    fn all(rows_count: u64, cols_count: u64) -> [Self; 4] {
        let (r1, r2) = Self::split(rows_count);
        let (c1, c2) = Self::split(cols_count);
        let (r3, c3) = (rows_count - 1, cols_count - 1);

        // 0     c1   c2 c3
        //    .....   .....
        // r1 .....   .....
        //
        // r2 .....   .....
        // r3 .....   .....

        [
            Self {
                rows_spawn: 0..=r1,
                cols_spawn: 0..=c1,
            },
            Self {
                rows_spawn: 0..=r1,
                cols_spawn: c2..=c3,
            },
            Self {
                rows_spawn: r2..=r3,
                cols_spawn: 0..=c1,
            },
            Self {
                rows_spawn: r2..=r3,
                cols_spawn: c2..=c3,
            },
        ]
    }

    fn contains(&self, loc: Location) -> bool {
        assert!(loc.row >= 0);
        let row = loc.row as u64;
        assert!(loc.col >= 0);
        let col = loc.col as u64;
        self.rows_spawn.contains(&row) && self.cols_spawn.contains(&col)
    }
}

#[test]
fn test_split() {
    // .....
    //  ^ ^
    assert_eq!(Quadrant::split(5), (1, 3));

    // ......
    //   ^^
    assert_eq!(Quadrant::split(6), (2, 3));
}

fn solve(input: &str, rows_count: u64, cols_count: u64) -> u64 {
    let robots = input
        .trim()
        .lines()
        .map(|l| Robot::parse(l, rows_count, cols_count))
        .collect::<Vec<_>>();

    let quadrants = Quadrant::all(rows_count, cols_count);
    let mut count = [0; 4];
    for robot in robots {
        let loc = robot.location_after_seconds(100);
        for i in 0..4 {
            if quadrants[i].contains(loc) {
                count[i] += 1;
            }
        }
    }
    // for seconds in 0..10 {
    //     print_robots(&robots, rows_count, cols_count, seconds);
    //     println!()
    // }

    count[0] * count[1] * count[2] * count[3]
}

#[test]
fn test() {
    let input = include_str!("input_test.txt");
    let output = solve(input, 7, 11);
    assert_eq!(output, 12);
}
