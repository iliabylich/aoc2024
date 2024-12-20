use core::fmt::Write;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

pub struct Writer<'a> {
    buf: &'a mut [u8],
    offset: usize,
}

impl<'a> Writer<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Writer { buf, offset: 0 }
    }

    pub fn as_str(self) -> Option<&'a str> {
        if self.offset <= self.buf.len() {
            // only successful concats of str - must be a valid str.
            Some(core::str::from_utf8(&self.buf[..self.offset]).unwrap())
        } else {
            None
        }
    }
}

impl core::fmt::Write for Writer<'_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();

        // Skip over already-copied data
        let remainder = &mut self.buf[self.offset..];
        // Check if there is space remaining (return error instead of panicking)
        if remainder.len() < bytes.len() {
            return Err(core::fmt::Error);
        }
        // Make the two slices the same length
        let remainder = &mut remainder[..bytes.len()];
        // Copy
        remainder.copy_from_slice(bytes);

        // Update offset to avoid overwriting
        self.offset += bytes.len();

        Ok(())
    }
}

pub fn write_to<'a>(
    buf: &'a mut [u8],
    args: core::fmt::Arguments,
) -> Result<&'a str, core::fmt::Error> {
    let mut w = Writer::new(buf);
    core::fmt::write(&mut w, args)?;
    w.as_str().ok_or(core::fmt::Error)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Stone(usize);

impl Stone {
    fn blink(self) -> (Stone, Option<Stone>) {
        if self.0 == 0 {
            return (Stone(1), None);
        }

        let mut buf = [0; 20];
        let mut writer = Writer::new(&mut buf);
        write!(&mut writer, "{}", self.0).unwrap();
        let s = writer.as_str().unwrap();
        if s.len() % 2 == 0 {
            let (l, r) = s.split_at(s.len() / 2);
            return (Stone(l.parse().unwrap()), Some(Stone(r.parse().unwrap())));
        }

        (Stone(self.0 * 2024), None)
    }
}

#[derive(Debug)]
struct Line {
    stones: HashMap<Stone, usize>,
}

impl Line {
    fn parse(input: &str) -> Self {
        let mut stones = HashMap::new();
        for line in input.trim().split(' ') {
            *stones.entry(Stone(line.parse().unwrap())).or_default() += 1;
        }
        Self { stones }
    }

    fn blink(self) -> Self {
        let mut out = HashMap::new();

        for (stone, count) in self.stones {
            let (s1, s2) = stone.blink();

            *out.entry(s1).or_default() += count;
            if let Some(s2) = s2 {
                *out.entry(s2).or_default() += count;
            }
        }

        Self { stones: out }
    }

    fn len(&self) -> usize {
        self.stones.values().sum()
    }
}

fn solve(input: &str) -> usize {
    let mut line = Line::parse(input);

    for _ in 0..75 {
        line = line.blink()
    }

    line.len()
}

#[test]
fn test() {
    let input = include_str!("input_test.txt");
    let output = solve(input);
    assert_eq!(output, 65601038650482);
}
