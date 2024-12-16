use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    row: usize,
    col: usize,
}

impl Location {
    fn step(self, dir: Direction, rows_count: usize, cols_count: usize) -> Option<Location> {
        let (drow, dcol) = dir.drow_dcol();
        let row = self.row.checked_add_signed(drow)?;
        let col = self.col.checked_add_signed(dcol)?;
        if row >= rows_count || col >= cols_count {
            return None;
        }
        Some(Self { row, col })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn drow_dcol(self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }

    fn turn_clockwise(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }

    fn turn_counterclockwise(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Free,
    Start,
    End,
    Wall,
}

impl Cell {
    fn parse(b: u8) -> Self {
        match b {
            b'#' => Self::Wall,
            b'.' => Self::Free,
            b'S' => Self::Start,
            b'E' => Self::End,
            _ => panic!("wrong cell input: {}", b as char),
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Wall => '#',
                Self::Free => '.',
                Self::Start => 'S',
                Self::End => 'E',
            }
        )
    }
}

#[derive(Debug)]
struct Matrix {
    data: Vec<Vec<Cell>>,
    rows_count: usize,
    cols_count: usize,
    start_loc: Location,
    end_loc: Location,
}

impl Matrix {
    fn parse(input: &str) -> Self {
        let mut rows = vec![];
        let mut start_loc = None;
        let mut end_loc = None;

        for (rowno, line) in input.trim().lines().enumerate() {
            let mut row = vec![];
            for (colno, b) in line.bytes().enumerate() {
                let cell = Cell::parse(b);
                match cell {
                    Cell::Start => {
                        start_loc = Some(Location {
                            row: rowno,
                            col: colno,
                        })
                    }
                    Cell::End => {
                        end_loc = Some(Location {
                            row: rowno,
                            col: colno,
                        })
                    }
                    _ => {}
                }
                row.push(cell);
            }
            rows.push(row);
        }

        let rows_count = rows.len();
        let cols_count = rows[0].len();

        Self {
            data: rows,
            rows_count,
            cols_count,
            start_loc: start_loc.unwrap(),
            end_loc: end_loc.unwrap(),
        }
    }

    fn get(&self, loc: Location) -> Cell {
        *self.data.get(loc.row).unwrap().get(loc.col).unwrap()
    }

    fn get_best_path(&self) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back((self.start_loc, Direction::Right, 0));

        let mut best_score = BestScore::new(self.start_loc);

        while let Some((loc, dir, score)) = queue.pop_front() {
            if let Some(next_loc) = self.try_move(loc, dir) {
                let next_dir = dir;
                let next_score = score + 1;

                if best_score.inc(next_loc, next_score) {
                    queue.push_back((next_loc, next_dir, next_score));
                }
            }

            for turn in [Turn::Clockwise, Turn::Counterclockwise] {
                let next_dir = turn.transition(dir);
                if let Some(next_loc) = self.try_move(loc, next_dir) {
                    let next_score = score + 1001;

                    if best_score.inc(next_loc, next_score) {
                        queue.push_back((next_loc, next_dir, next_score));
                    }
                }
            }
        }

        best_score.get(self.end_loc).unwrap()
    }

    fn try_move(&self, loc: Location, dir: Direction) -> Option<Location> {
        let next_loc = loc.step(dir, self.rows_count, self.cols_count)?;
        let next_cell = self.get(next_loc);

        if next_cell == Cell::Wall {
            return None;
        }

        Some(next_loc)
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows_count {
            for col in 0..self.cols_count {
                write!(f, "{}", self.data[row][col]).unwrap()
            }
            writeln!(f).unwrap();
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Turn {
    Clockwise,
    Counterclockwise,
}

impl Turn {
    fn transition(self, dir: Direction) -> Direction {
        match self {
            Self::Clockwise => dir.turn_clockwise(),
            Self::Counterclockwise => dir.turn_counterclockwise(),
        }
    }
}

struct BestScore {
    map: HashMap<Location, usize>,
}

impl BestScore {
    fn new(start_loc: Location) -> Self {
        let mut map = HashMap::new();
        map.insert(start_loc, 0);
        Self { map }
    }

    fn inc(&mut self, loc: Location, new_value: usize) -> bool {
        let value = self.map.get(&loc).copied().unwrap_or(usize::MAX);

        if new_value < value {
            self.map.insert(loc, new_value);
            true
        } else {
            false
        }
    }

    fn get(&self, loc: Location) -> Option<usize> {
        self.map.get(&loc).copied()
    }
}

fn solve(input: &str) -> usize {
    let matrix = Matrix::parse(input);
    println!("{}", matrix);

    matrix.get_best_path()
}

#[test]
fn test1() {
    let input = include_str!("input_test1.txt");
    let output = solve(input);
    assert_eq!(output, 7036);
}

#[test]
fn test2() {
    let input = include_str!("input_test2.txt");
    let output = solve(input);
    assert_eq!(output, 11048);
}
