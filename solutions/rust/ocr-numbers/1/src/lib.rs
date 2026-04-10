// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const ZERO: &[&str] = &[
    " _ ",
    "| |",
    "|_|",
    "   ",
];

const ONE: &[&str] = &[
    "   ",
    "  |",
    "  |",
    "   ",
];

const TWO: &[&str] = &[
    " _ ",
    " _|",
    "|_ ",
    "   ",
];

const THREE: &[&str] = &[
    " _ ",
    " _|",
    " _|",
    "   ",
];

const FOUR: &[&str] = &[
    "   ",
    "|_|",
    "  |",
    "   ",
];

const FIVE: &[&str] = &[
    " _ ",
    "|_ ",
    " _|",
    "   ",
];

const SIX: &[&str] = &[
    " _ ",
    "|_ ",
    "|_|",
    "   ",
];

const SEVEN: &[&str] = &[
    " _ ",
    "  |",
    "  |",
    "   ",
];

const EIGHT: &[&str] = &[
    " _ ",
    "|_|",
    "|_|",
    "   ",
];

const NINE: &[&str] = &[
    " _ ",
    "|_|",
    " _|",
    "   ",
];

const DIGITS: &[&[&str]] = &[
    ZERO, ONE, TWO, THREE, FOUR,
    FIVE, SIX, SEVEN, EIGHT, NINE,
];


#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

use Error::*;

pub fn convert(input: &str) -> Result<String, Error> {
    let lines = input.lines().collect::<Vec<&str>>();
    
    if lines.len() % 4 != 0 {
        return Err(InvalidRowCount(lines.len()));
    }

    for line in &lines {
        if line.len() % 3 != 0 {
            return Err(InvalidColumnCount(line.len()))
        }
    }

    let rows = lines.len() / 4;
    let cols = lines[0].len() / 3;
    let mut s = String::with_capacity(rows * (cols + 1));
    for (chunk_idx,chunk) in lines.chunks(4).enumerate() {
        for i in 0..cols {
            let cell  = chunk.iter()
                                        .map(|&line| &line[i*3..i*3+3])
                                        .collect::<Vec<&str>>();

            let digit = DIGITS.iter()
                               .position(|&number|number == cell)
                               .map(|n| char::from(b'0' + n as u8))
                               .unwrap_or_else(|| '?');

            s.push(digit);
        }
        if chunk_idx < rows - 1 {
            s.push(',');
        } 
    }

   Ok(s)
} 
