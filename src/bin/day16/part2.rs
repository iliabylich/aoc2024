use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    row: usize,
    col: usize,
}

impl std::fmt::Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
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

    fn moves(&self, loc: Location, dir: Direction) -> Vec<(Location, Direction, usize)> {
        let mut out = vec![];

        if let Some(next_loc) = self.try_move(loc, dir) {
            out.push((next_loc, dir, 1));
        }

        for turn in [Turn::Clockwise, Turn::Counterclockwise] {
            let next_dir = turn.transition(dir);
            if self.try_move(loc, next_dir).is_some() {
                out.push((loc, next_dir, 1000));
            }
        }

        out
    }

    fn get_best_paths(&self) -> HashSet<Location> {
        let mut queue = VecDeque::new();

        let initial = (self.start_loc, Direction::Right);
        queue.push_back((vec![initial], 0));
        let mut best_score = BestScore::new(initial);

        let mut paths = HashMap::<usize, Vec<HashSet<Location>>>::new();

        while let Some((path, score)) = queue.pop_front() {
            let (loc, dir) = *path.last().unwrap();

            if loc == self.end_loc {
                let bucket = paths.entry(score).or_default();
                let path = HashSet::from_iter(path.iter().map(|(l, _)| *l));
                bucket.push(path);
            }

            for (next_loc, next_dir, cost) in self.moves(loc, dir) {
                if best_score.inc((next_loc, next_dir), score + cost) {
                    let mut next_path = path.clone();
                    next_path.push((next_loc, next_dir));

                    queue.push_back((next_path, score + cost));
                }
            }
        }

        let score = best_score.min_for_loc(self.end_loc).unwrap();
        let best_paths = paths.get(&score).unwrap();

        let mut merged = HashSet::new();
        for path in best_paths {
            for loc in path.iter() {
                merged.insert(*loc);
            }
        }

        merged
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
    map: HashMap<(Location, Direction), usize>,
}

impl BestScore {
    fn new(start: (Location, Direction)) -> Self {
        let mut map = HashMap::new();
        map.insert(start, 0);
        Self { map }
    }

    fn inc(&mut self, key: (Location, Direction), new_value: usize) -> bool {
        if let Some(value) = self.map.get_mut(&key) {
            if new_value <= *value {
                *value = new_value;
                true
            } else {
                // longer path
                false
            }
        } else {
            self.map.insert(key, new_value);
            true
        }
    }

    fn get(&self, key: (Location, Direction)) -> Option<usize> {
        self.map.get(&key).copied()
    }

    fn min_for_loc(&self, loc: Location) -> Option<usize> {
        let up = self.get((loc, Direction::Up)).unwrap_or(usize::MAX);
        let down = self.get((loc, Direction::Down)).unwrap_or(usize::MAX);
        let left = self.get((loc, Direction::Left)).unwrap_or(usize::MAX);
        let right = self.get((loc, Direction::Right)).unwrap_or(usize::MAX);

        [up, down, left, right].into_iter().min()
    }
}

fn solve(input: &str) -> usize {
    let matrix = Matrix::parse(input);
    let path = matrix.get_best_paths();
    path.len()
}

#[test]
fn test1() {
    let input = include_str!("input_test1.txt");
    let output = solve(input);
    assert_eq!(output, 45);
}

#[test]
fn test2() {
    let input = include_str!("input_test2.txt");
    let output = solve(input);
    assert_eq!(output, 64);
}
