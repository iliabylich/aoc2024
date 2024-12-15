fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Free,
    Robot,
    Box,
    Wall,
}

impl Cell {
    fn parse(b: u8) -> Self {
        match b {
            b'#' => Self::Wall,
            b'.' => Self::Free,
            b'O' => Self::Box,
            b'@' => Self::Robot,
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
                Self::Box => 'O',
                Self::Robot => '@',
            }
        )
    }
}

#[derive(Debug)]
struct Matrix {
    data: Vec<Vec<Cell>>,
    rows_count: usize,
    cols_count: usize,
    robot_loc: Location,
}

impl Matrix {
    fn parse(input: &str) -> Self {
        let mut rows = vec![];
        let mut robot_loc = None;

        for (rowno, line) in input.trim().lines().enumerate() {
            let mut row = vec![];
            for (colno, b) in line.bytes().enumerate() {
                let cell = Cell::parse(b);
                if cell == Cell::Robot {
                    robot_loc = Some(Location {
                        row: rowno,
                        col: colno,
                    })
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
            robot_loc: robot_loc.unwrap(),
        }
    }

    fn step(&mut self, dir: Direction) {
        let mut moved_cells = vec![self.robot_loc];
        loop {
            let last = *moved_cells.last().unwrap();
            if let Some(next) = last.step(dir, self.rows_count, self.cols_count) {
                let cell = self.data[next.row][next.col];
                match cell {
                    Cell::Free => {
                        moved_cells.push(next);
                        break;
                    }
                    Cell::Robot => panic!("can't see two robots"),
                    Cell::Box => {
                        moved_cells.push(next);
                    }
                    Cell::Wall => return,
                }
            } else {
                panic!("we must've seen a wall at this point");
            }
        }

        assert!(moved_cells.len() >= 2);

        for (move_to, move_from) in moved_cells
            .iter()
            .rev()
            .zip(moved_cells.iter().rev().skip(1))
        {
            self.data[move_to.row][move_to.col] = self.data[move_from.row][move_from.col];
            self.data[move_from.row][move_from.col] = Cell::Free;
        }

        self.robot_loc = moved_cells[1];
    }

    fn score(&self) -> usize {
        let mut out = 0;
        for row in 0..self.rows_count {
            for col in 0..self.cols_count {
                if self.data[row][col] == Cell::Box {
                    out += 100 * row + col;
                }
            }
        }
        out
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

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<u8> for Direction {
    type Error = ();

    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'\n' => Err(()),
            b'^' => Ok(Self::Up),
            b'v' => Ok(Self::Down),
            b'<' => Ok(Self::Left),
            b'>' => Ok(Self::Right),
            _ => panic!("wrong insn: {}", b as char),
        }
    }
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
}

fn solve(input: &str) -> usize {
    let (matrix, insns) = input.split_once("\n\n").unwrap();
    let mut matrix = Matrix::parse(matrix);

    for dir in insns.bytes().filter_map(|b| Direction::try_from(b).ok()) {
        matrix.step(dir);
    }

    matrix.score()
}

#[test]
fn test1() {
    let input = include_str!("input_test1.txt");
    let output = solve(input);
    assert_eq!(output, 2028);
}

#[test]
fn test2() {
    let input = include_str!("input_test2.txt");
    let output = solve(input);
    assert_eq!(output, 10092);
}
