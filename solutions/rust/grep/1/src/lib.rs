use anyhow::Error;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
#[derive(Debug)]
pub struct Flags{
    commans: Vec<Command>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)] // 强制枚举在内存中占用 1 个字节
enum Command {
    NUMBER = b'c',
    FILE = b'l',
    INSENSITIVE = b'i',
    INVERT = b'v',
    LINE = b'x',
}

use Command::*;

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut cmds = vec![];
        for &flag in flags {
            if flag.starts_with('-') {
                for c in flag.chars() {
                    // if let Some(cmd) = Command::from_u8(c.to_) {
                    match c {
                        'n' => cmds.push(NUMBER),
                        'l' => cmds.push(FILE),
                        'i' => cmds.push(INSENSITIVE),
                        'v' => cmds.push(INVERT),
                        'x' => cmds.push(LINE),
                         _  => continue,
                    }
                }
            }
        }

        Self { commans: cmds }
    }
}

use std::fs;
// use std::io::Read;

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut answer = vec![];
    let pattern =  if !flags.commans.contains(&INSENSITIVE) {
        pattern.to_string()
    } else {
        pattern.to_ascii_lowercase()
    };

    println!("{:?}", flags.commans);

    for name in files {
        let content = fs::read_to_string(name)?;

        for (i, raw_line) in content.split('\n').enumerate() {
            if raw_line.is_empty() {continue;}
            let line =  if !flags.commans.contains(&INSENSITIVE) {
                raw_line.to_string()
            } else {
                raw_line.to_ascii_lowercase()
            };

            let mut catched = if flags.commans.contains(&LINE) {
                // println!("{}, === {}", line, pattern);
                // line.contains(&pattern) && pattern.contains(&line)
                line == pattern
            } else {
                line.contains(&pattern)
            };
            if flags.commans.contains(&INVERT) {
                  catched = !catched
            } 
            if catched {
                if flags.commans.contains(&FILE) {
                    answer.push(format!("{name}"));
                    break;
                } 
                if flags.commans.contains(&NUMBER) {
                    let n = i + 1;
                    if files.len() > 1 {
                        answer.push(format!("{name}:{n}:{raw_line}"));
                    } else {
                        answer.push(format!("{n}:{raw_line}"));
                    }
                } else {
                    if files.len() > 1 {
                        answer.push(format!("{name}:{raw_line}"));
                    } else {
                        answer.push(format!("{raw_line}"));
                    }
                }
            }
        }
    }

    Ok(answer)
}
