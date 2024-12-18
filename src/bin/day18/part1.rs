use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[cfg(test)]
const SIZE: usize = 7;
#[cfg(not(test))]
const SIZE: usize = 71;

#[cfg(test)]
const BYTES: usize = 12;
#[cfg(not(test))]
const BYTES: usize = 1024;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Free,
    Blocked,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Blocked => "#",
                Self::Free => ".",
            }
        )
    }
}

#[derive(Debug)]
struct Matrix {
    data: [[Cell; SIZE]; SIZE],
}

impl Matrix {
    fn parse(input: &str) -> Self {
        let mut data = [[Cell::Free; SIZE]; SIZE];
        for line in input.trim().lines().take(BYTES) {
            let (col, row) = line.split_once(',').unwrap();
            let col = col.parse::<usize>().unwrap();
            let row = row.parse::<usize>().unwrap();
            data[row][col] = Cell::Blocked
        }
        Self { data }
    }

    fn shortest_path(&mut self) -> usize {
        let start = (0_usize, 0_usize);
        let end = (SIZE - 1, SIZE - 1);

        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();
        queue.push_back(start);
        visited.insert(start, 0);

        while let Some((row, col)) = queue.pop_front() {
            const DS: [[isize; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];
            let score = *visited.get(&(row, col)).unwrap();
            let next_score = score + 1;

            for [drow, dcol] in DS {
                let Some(next_row) = row.checked_add_signed(drow) else {
                    continue;
                };
                let Some(next_col) = col.checked_add_signed(dcol) else {
                    continue;
                };
                if next_row >= SIZE || next_col >= SIZE {
                    continue;
                }
                let next_cell = self.data[next_row][next_col];
                if matches!(next_cell, Cell::Blocked) {
                    continue;
                }

                let candidate = (next_row, next_col);

                match visited.get(&candidate).copied() {
                    Some(prev_score) if prev_score <= next_score => {}
                    _ => {
                        queue.push_back(candidate);
                        visited.insert(candidate, next_score);
                    }
                }
            }
        }

        visited.get(&end).copied().unwrap()
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn solve(input: &str) -> usize {
    let mut matrix = Matrix::parse(input);

    matrix.shortest_path()
}

#[test]
fn test1() {
    let input = include_str!("input_test.txt");
    let output = solve(input);
    assert_eq!(output, 22);
}
