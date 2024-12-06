use rayon::prelude::*;
use std::collections::HashSet;

fn main() {
    let input = include_bytes!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Blocked,
    Empty,
    Visited,
}

impl From<u8> for Cell {
    fn from(byte: u8) -> Self {
        match byte {
            b'.' => Self::Empty,
            b'#' => Self::Blocked,
            b'^' => Self::Visited,
            _ => unimplemented!("wrong cell byte: {}", byte as char),
        }
    }
}

#[derive(Debug, Clone)]
struct Matrix {
    rows: Vec<Vec<Cell>>,
    rows_count: usize,
    cols_count: usize,
}

impl Matrix {
    fn parse(bytes: &[u8]) -> (Self, Location) {
        let mut rows = vec![];
        let mut startrow = None;
        let mut startcol = None;

        for (rowno, row_bytes) in bytes.split(|b| *b == b'\n').enumerate() {
            if row_bytes.is_empty() {
                continue;
            }
            let mut row = vec![];
            for (colno, byte) in row_bytes.iter().enumerate() {
                row.push(Cell::from(*byte));
                if *byte == b'^' {
                    startrow = Some(rowno);
                    startcol = Some(colno);
                }
            }
            rows.push(row);
        }

        (
            Self {
                rows_count: rows.len(),
                cols_count: rows.first().unwrap().len(),
                rows,
            },
            Location {
                dir: Direction::Up,
                row: startrow.unwrap() as isize,
                col: startcol.unwrap() as isize,
            },
        )
    }

    fn visit(&mut self, row: usize, col: usize) {
        match self.rows[row][col] {
            Cell::Blocked => panic!("bug"),
            Cell::Empty | Cell::Visited => self.rows[row][col] = Cell::Visited,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Self::Left => Self::Up,
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
        }
    }

    fn drow_dcol(self) -> (isize, isize) {
        match self {
            Self::Left => (0, -1),
            Self::Up => (-1, 0),
            Self::Right => (0, 1),
            Self::Down => (1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    dir: Direction,
    row: isize,
    col: isize,
}

impl Location {
    fn next(self, matrix: &Matrix) -> Option<(usize, usize, Direction)> {
        if let Some(answer) = self.next_in_the_same_direction().validate(matrix) {
            return Some(answer);
        }

        // try 4 rotations
        for i in 0..4 {
            if let Some(answer) = self.next_turning_right(i).validate(matrix) {
                return Some(answer);
            }
        }

        None
    }

    fn next_in_the_same_direction(self) -> Self {
        let (drow, dcol) = self.dir.drow_dcol();
        Self {
            dir: self.dir,
            row: self.row + drow,
            col: self.col + dcol,
        }
    }

    fn next_turning_right(self, n: u8) -> Self {
        let mut dir = self.dir;
        for _ in 0..n {
            dir = dir.turn_right();
        }
        let (drow, dcol) = dir.drow_dcol();
        Self {
            dir,
            row: self.row + drow,
            col: self.col + dcol,
        }
    }

    fn validate(self, matrix: &Matrix) -> Option<(usize, usize, Direction)> {
        let Self { row, col, dir } = self;
        if row < 0 || col < 0 {
            return None;
        }
        let row = row as usize;
        let col = col as usize;
        if row >= matrix.rows_count || col >= matrix.cols_count {
            return None;
        }
        match matrix.rows[row][col] {
            Cell::Blocked => None,
            Cell::Empty | Cell::Visited => Some((row, col, dir)),
        }
    }

    fn is_dead_end(self, matrix: &Matrix) -> bool {
        match self {
            Self {
                dir: Direction::Left,
                col: 0,
                ..
            } => true,
            Self {
                dir: Direction::Up,
                row: 0,
                ..
            } => true,
            Self {
                dir: Direction::Right,
                col,
                ..
            } if col == (matrix.cols_count - 1) as isize => true,
            Self {
                dir: Direction::Down,
                row,
                ..
            } if row == (matrix.rows_count - 1) as isize => true,
            _ => false,
        }
    }
}

#[allow(dead_code)]
fn print(matrix: &Matrix, location: Location) {
    println!("\n\n");
    for (rowno, row) in matrix.rows.iter().enumerate() {
        for (colno, col) in row.iter().enumerate() {
            if location.row == rowno as isize && location.col == colno as isize {
                match location.dir {
                    Direction::Left => print!("<"),
                    Direction::Up => print!("^"),
                    Direction::Right => print!(">"),
                    Direction::Down => print!("V"),
                }
            } else {
                match col {
                    Cell::Blocked => print!("#"),
                    Cell::Empty => print!("."),
                    Cell::Visited => print!("X"),
                }
            }
        }
        println!()
    }
    println!("\n\n");
}

fn is_loop(mut matrix: Matrix, mut location: Location, rowno: usize, colno: usize) -> bool {
    if location.row as usize == rowno && location.col as usize == colno {
        // can't have at the starting point
        return false;
    }
    matrix.rows[rowno][colno] = Cell::Blocked;

    let mut visited = HashSet::new();
    visited.insert(location);

    while let Some((row, col, dir)) = location.next(&matrix) {
        matrix.visit(row, col);
        location = Location {
            dir,
            row: row as isize,
            col: col as isize,
        };
        if location.is_dead_end(&matrix) {
            break;
        }
        if !visited.insert(location) {
            // LOOP
            return true;
        }
        // print!("{}[2J", 27 as char);
        // print(&matrix, location);
        // std::thread::sleep(std::time::Duration::from_millis(10));
    }

    false
}

fn solve(input: &[u8]) -> usize {
    let (matrix, location) = Matrix::parse(input);

    let mut candidates = vec![];
    for row in 0..matrix.rows_count {
        for col in 0..matrix.cols_count {
            candidates.push((row, col));
        }
    }

    let locs = candidates
        .par_iter()
        .filter(|(row, col)| {
            let matrix = matrix.clone();
            is_loop(matrix, location, *row, *col)
        })
        .collect::<Vec<_>>();

    locs.len()
}

#[test]
fn test() {
    let input = include_bytes!("input_test.txt");
    let output = solve(input);
    assert_eq!(output, 6);
}
