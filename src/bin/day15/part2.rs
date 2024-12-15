use std::collections::HashSet;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Free,
    Robot,
    BoxLeft,
    BoxRight,
    Wall,
}

impl Cell {
    fn parse(b: u8) -> (Self, Self) {
        match b {
            b'#' => (Self::Wall, Self::Wall),
            b'.' => (Self::Free, Self::Free),
            b'O' => (Self::BoxLeft, Self::BoxRight),
            b'@' => (Self::Robot, Self::Free),
            _ => panic!("wrong cell input: {}", b as char),
        }
    }

    fn counterpart(self, loc: Location) -> Location {
        match self {
            Self::BoxLeft => Location {
                row: loc.row,
                col: loc.col + 1,
            },
            Self::BoxRight => Location {
                row: loc.row,
                col: loc.col - 1,
            },
            _ => panic!("not a box side"),
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
                Self::BoxLeft => '[',
                Self::BoxRight => ']',
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
                let (cell1, cell2) = Cell::parse(b);
                if cell1 == Cell::Robot {
                    robot_loc = Some(Location {
                        row: rowno,
                        col: colno * 2,
                    })
                }
                row.push(cell1);
                row.push(cell2);
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
        let mut layers = vec![Layer::starting(self.robot_loc, dir)];

        loop {
            let prev_layer = layers.last().unwrap();
            let next_layer = prev_layer.next(self);

            if next_layer.is_empty() {
                // Found empty spot, pushing
                break;
            } else if next_layer.contains_wall(self) {
                // Found wall, aborting
                return;
            } else {
                layers.push(next_layer);
            }
        }

        for layer in layers.into_iter().rev() {
            self.push_layer(layer);
        }
        self.robot_loc = self
            .robot_loc
            .step(dir, self.rows_count, self.cols_count)
            .unwrap();
    }

    fn push_layer(&mut self, layer: Layer) {
        for loc in layer.to_locations() {
            let next = loc
                .step(layer.direction(), self.rows_count, self.cols_count)
                .unwrap();

            assert_eq!(self.data[next.row][next.col], Cell::Free);
            self.data[next.row][next.col] = self.data[loc.row][loc.col];
            self.data[loc.row][loc.col] = Cell::Free;
        }
    }

    fn score(&self) -> usize {
        let mut out = 0;
        for row in 0..self.rows_count {
            for col in 0..self.cols_count {
                if self.data[row][col] == Cell::BoxLeft {
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

#[derive(Debug)]
enum Layer {
    Vertical {
        row: usize,
        cols: HashSet<usize>,
        dir: VerticalDirection,
    },
    Horizontal {
        col: usize,
        rows: HashSet<usize>,
        dir: HorizontalDirection,
    },
}

impl Layer {
    fn starting(robot_loc: Location, dir: Direction) -> Self {
        match dir {
            Direction::Vertical(dir) => Self::Vertical {
                row: robot_loc.row,
                cols: HashSet::from([robot_loc.col]),
                dir,
            },
            Direction::Horizontal(dir) => Self::Horizontal {
                col: robot_loc.col,
                rows: HashSet::from([robot_loc.row]),
                dir,
            },
        }
    }

    fn direction(&self) -> Direction {
        match self {
            Layer::Vertical { dir, .. } => Direction::Vertical(*dir),
            Layer::Horizontal { dir, .. } => Direction::Horizontal(*dir),
        }
    }

    fn to_locations(&self) -> Vec<Location> {
        match self {
            Layer::Vertical { row, cols, .. } => cols
                .iter()
                .map(|col| Location {
                    row: *row,
                    col: *col,
                })
                .collect(),
            Layer::Horizontal { col, rows, .. } => rows
                .iter()
                .map(|row| Location {
                    row: *row,
                    col: *col,
                })
                .collect(),
        }
    }

    fn contains_wall(&self, matrix: &Matrix) -> bool {
        self.to_locations()
            .into_iter()
            .any(|loc| matrix.data[loc.row][loc.col] == Cell::Wall)
    }

    fn is_empty(&self) -> bool {
        match self {
            Layer::Vertical { cols, .. } => cols.is_empty(),
            Layer::Horizontal { rows, .. } => rows.is_empty(),
        }
    }

    fn next(&self, matrix: &Matrix) -> Self {
        let mut locations = HashSet::new();

        for loc in self.to_locations() {
            let next = loc
                .step(self.direction(), matrix.rows_count, matrix.cols_count)
                .unwrap();
            let cell = matrix.data[next.row][next.col];
            match cell {
                Cell::Free => {}
                Cell::Robot => panic!("there's only one robot"),
                Cell::BoxLeft | Cell::BoxRight => {
                    locations.insert(next);
                    if matches!(self.direction(), Direction::Vertical(_)) {
                        locations.insert(cell.counterpart(next));
                    }
                }
                Cell::Wall => {
                    locations.insert(next);
                }
            }
        }

        match self {
            Layer::Vertical { row, dir, .. } => {
                let row = row.checked_add_signed(dir.drow()).unwrap();
                Layer::Vertical {
                    row,
                    cols: locations
                        .into_iter()
                        .map(|loc| {
                            assert_eq!(loc.row, row);
                            loc.col
                        })
                        .collect(),
                    dir: *dir,
                }
            }
            Layer::Horizontal { col, dir, .. } => {
                let col = col.checked_add_signed(dir.dcol()).unwrap();
                Layer::Horizontal {
                    col,
                    rows: locations
                        .into_iter()
                        .map(|loc| {
                            assert_eq!(loc.col, col);
                            loc.row
                        })
                        .collect(),
                    dir: *dir,
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Vertical(VerticalDirection),
    Horizontal(HorizontalDirection),
}

impl TryFrom<u8> for Direction {
    type Error = ();

    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'\n' => Err(()),
            b'^' => Ok(Self::Vertical(VerticalDirection::Up)),
            b'v' => Ok(Self::Vertical(VerticalDirection::Down)),
            b'<' => Ok(Self::Horizontal(HorizontalDirection::Left)),
            b'>' => Ok(Self::Horizontal(HorizontalDirection::Right)),
            _ => panic!("wrong insn: {}", b as char),
        }
    }
}

impl Direction {
    fn drow_dcol(self) -> (isize, isize) {
        match self {
            Self::Vertical(dir) => (dir.drow(), 0),
            Self::Horizontal(dir) => (0, dir.dcol()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum VerticalDirection {
    Up,
    Down,
}
impl VerticalDirection {
    fn drow(self) -> isize {
        match self {
            Self::Up => -1,
            Self::Down => 1,
        }
    }
}
#[derive(Debug, Clone, Copy)]
enum HorizontalDirection {
    Left,
    Right,
}
impl HorizontalDirection {
    fn dcol(self) -> isize {
        match self {
            Self::Left => -1,
            Self::Right => 1,
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
fn test2() {
    let input = include_str!("input_test2.txt");
    let output = solve(input);
    assert_eq!(output, 9021);
}

#[test]
fn test3() {
    let input = include_str!("input_test3.txt");
    let output = solve(input);
    assert_eq!(output, 618);
}
