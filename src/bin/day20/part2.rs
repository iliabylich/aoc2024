use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Wall,
    Free,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    row: usize,
    col: usize,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.row, self.col)
    }
}

impl Location {
    fn cheating_candidates(self, rows_count: usize, cols_count: usize) -> Vec<Self> {
        let mut out = vec![];

        for drow in -20_isize..=20 {
            for dcol in -20_isize..=20 {
                if drow.abs() + dcol.abs() <= 20 {
                    if let Some(next) = self.add(drow, dcol, rows_count, cols_count) {
                        out.push(next);
                    }
                }
            }
        }

        out
    }

    fn add(self, drow: isize, dcol: isize, rows_count: usize, cols_count: usize) -> Option<Self> {
        let row = self.row.checked_add_signed(drow)?;
        let col = self.col.checked_add_signed(dcol)?;
        if row >= rows_count || col >= cols_count {
            return None;
        }
        Some(Self { row, col })
    }

    fn distance(self, other: Self) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

#[derive(Debug, Clone)]
struct Matrix {
    data: Vec<Vec<Cell>>,
    rows_count: usize,
    cols_count: usize,
    start: Location,
    end: Location,
}

impl Matrix {
    fn parse(input: &str) -> Self {
        let mut data = vec![];
        let mut start = None;
        let mut end = None;

        for (rowno, line) in input.trim().lines().enumerate() {
            let mut row = vec![];
            for (colno, b) in line.bytes().enumerate() {
                match b {
                    b'.' => row.push(Cell::Free),
                    b'#' => row.push(Cell::Wall),
                    b'S' => {
                        row.push(Cell::Free);
                        start = Some(Location {
                            row: rowno,
                            col: colno,
                        });
                    }
                    b'E' => {
                        row.push(Cell::Free);
                        end = Some(Location {
                            row: rowno,
                            col: colno,
                        });
                    }
                    _ => unreachable!("{}", b as char),
                }
            }
            data.push(row);
        }

        Self {
            rows_count: data.len(),
            cols_count: data[0].len(),
            data,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }

    fn get(&self, loc: Location) -> Cell {
        *self.data.get(loc.row).unwrap().get(loc.col).unwrap()
    }
}

const MAX_PATH: u32 = 50_000;

fn find_all_fastest_paths_uncached(matrix: &Matrix, locations: &[Location]) -> Vec<Vec<u32>> {
    let mut distance = vec![vec![MAX_PATH; locations.len()]; locations.len()];

    for (left_id, left) in locations.iter().enumerate() {
        for (right_id, right) in locations.iter().enumerate() {
            if matrix.get(*left) != Cell::Wall
                && matrix.get(*right) != Cell::Wall
                && left.distance(*right) == 1
            {
                distance[left_id][right_id] = 1;
                distance[right_id][left_id] = 1;
            }
        }
    }

    for loc_id in 0..locations.len() {
        distance[loc_id][loc_id] = 0;
    }

    for k in 0..locations.len() {
        for i in 0..locations.len() {
            for j in 0..locations.len() {
                if distance[i][j] > distance[i][k] + distance[k][j] {
                    distance[i][j] = distance[i][k] + distance[k][j]
                }
            }
        }
    }

    distance
}

fn solve(input: &str) -> usize {
    let win_to_count = build_win_to_count_map(input);

    let mut out = 0;
    for (win, count) in win_to_count {
        if win >= 100 {
            out += count;
        }
    }
    out
}

struct FsCache;

impl FsCache {
    const FILENAME: &str = if cfg!(test) {
        "cache.test"
    } else {
        "cache.release"
    };

    fn write(data: &[Vec<u32>]) {
        let mut bytes = vec![];
        for row in data {
            for col in row {
                for byte in col.to_le_bytes() {
                    bytes.push(byte);
                }
            }
        }
        std::fs::write(Self::FILENAME, bytes).unwrap()
    }

    fn read(matrix_size: usize) -> Vec<Vec<u32>> {
        let bytes = std::fs::read(Self::FILENAME).unwrap();
        let mut out = vec![];

        for batch_u8 in bytes.chunks(matrix_size * 4) {
            let mut buf = Vec::with_capacity(matrix_size);
            for four in batch_u8.chunks(4) {
                buf.push(u32::from_le_bytes([four[0], four[1], four[2], four[3]]));
            }
            out.push(buf);
        }

        out
    }

    fn fetch(matrix_size: usize, f: impl Fn() -> Vec<Vec<u32>>) -> Vec<Vec<u32>> {
        if std::fs::exists(Self::FILENAME).is_ok_and(|v| v) {
            Self::read(matrix_size)
        } else {
            let data = f();
            Self::write(&data);
            data
        }
    }
}

fn find_all_fastest_paths(matrix: &Matrix, locations: &[Location]) -> Vec<Vec<u32>> {
    FsCache::fetch(locations.len(), || {
        find_all_fastest_paths_uncached(matrix, locations)
    })
}

fn build_win_to_count_map(input: &str) -> HashMap<u32, usize> {
    let matrix = Matrix::parse(input);

    let locations = (0..matrix.rows_count)
        .cartesian_product(0..matrix.cols_count)
        .map(|(row, col)| Location { row, col })
        .filter(|loc| matrix.get(*loc) == Cell::Free)
        .collect::<Vec<_>>();

    let start_id = locations
        .iter()
        .position(|loc| *loc == matrix.start)
        .unwrap();
    let end_id = locations.iter().position(|loc| *loc == matrix.end).unwrap();

    let all_paths = find_all_fastest_paths(&matrix, &locations);

    let initial_score = all_paths[start_id][end_id];
    if initial_score == MAX_PATH {
        panic!("no initial path");
    }

    let mut map = HashMap::<u32, usize>::new();

    for (loc_id, loc) in locations.iter().enumerate() {
        if matrix.get(*loc) != Cell::Free {
            continue;
        }

        for teleports_to in loc.cheating_candidates(matrix.rows_count, matrix.cols_count) {
            let Some(teleports_to_id) = locations.iter().position(|loc| *loc == teleports_to)
            else {
                continue;
            };
            let d1 = all_paths[start_id][loc_id];
            if d1 == MAX_PATH {
                continue;
            };
            let d2 = all_paths[teleports_to_id][end_id];
            if d2 == MAX_PATH {
                continue;
            };
            let cheated_score = d1 + d2 + loc.distance(teleports_to) as u32;
            let Some(win) = initial_score.checked_sub(cheated_score) else {
                continue;
            };
            if win == 0 {
                continue;
            }

            *map.entry(win).or_default() += 1;
        }
    }

    map
}

#[test]
fn test1() {
    let input = include_str!("input_test.txt");
    let output = build_win_to_count_map(input);

    let mut pairs = output
        .into_iter()
        .map(|(win, count)| (count, win))
        .filter(|(_, win)| *win >= 50)
        .collect::<Vec<_>>();
    pairs.sort_unstable_by_key(|(_count, win)| *win);

    assert_eq!(
        pairs,
        [
            (32, 50),
            (31, 52),
            (29, 54),
            (39, 56),
            (25, 58),
            (23, 60),
            (20, 62),
            (19, 64),
            (12, 66),
            (14, 68),
            (12, 70),
            (22, 72),
            (4, 74),
            (3, 76),
        ]
    );

    let output = solve(input);
    assert_eq!(output, 0);
}
