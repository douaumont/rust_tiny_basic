/*
    Tiny BASIC interpreter written in Rust
    Copyright (C) 2025 Artyom Makarov

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/


use std::collections::BTreeSet;

type LineIndex = i32;

const MIN_LINE_NUMBER: LineIndex = 1;
const MAX_LINE_NUMBER: LineIndex = 10000;

fn get_line_number_end(line: &str) -> Option<usize> {
    if line.starts_with(|c: char| c.is_ascii_digit()) {
        let mut line_number_end = 0;
        for (i, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                line_number_end = i;
            } else {
                break;
            }
        }
        Some(line_number_end + 1)
    } else {
        None
    }
}

#[derive(PartialEq, Eq, PartialOrd)]
struct Line {
    index: Option<LineIndex>,
    line: String
}

impl Ord for Line {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.index {
            Some(i) => write!(f, "{} {}", i, self.line),
            None => write!(f, "{}", self.line)
        }
    }
}

impl TryFrom<&str> for Line {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(line_number_end) = get_line_number_end(value) {
            let line_number: LineIndex = value[0..line_number_end].parse().expect("Line number should be parsed.");

            if (MIN_LINE_NUMBER..=MAX_LINE_NUMBER).contains(&line_number) {
                Ok(Line {
                    index: Some(line_number),
                    line: value[line_number_end..value.len()].trim_ascii_start().to_string()
                })
            } else {
                Err(Error::InvalidLineNumber)
            }
        } else {
            Ok(Line {
                index: None,
                line: value.to_string()
            })
        }
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidLineNumber,
    UnrecognisedKeyword(String),
    UnmatchedQuote,
    Expected(char)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidLineNumber => write!(f, "Line numbers should be in range [{}; {}]", MIN_LINE_NUMBER, MAX_LINE_NUMBER),
            Error::UnrecognisedKeyword(s) => write!(f, "Unrecognised keyword: {}", s),
            Error::UnmatchedQuote => todo!(),
            Error::Expected(_) => todo!(),
        }
    }
}

pub struct Interpreter {
    lines: BTreeSet<Line>
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            lines: BTreeSet::new()
        }
    }

    pub fn execute(&mut self, line: &str) -> Result<(), Error> {
        assert!(!line.starts_with(' ') && !line.ends_with(' '));

        let line = Line::try_from(line)?;
        
        match line.index {
            Some(_) => { self.lines.insert(line); },
            None => self.run_line(&line.line)?
        }
        
        Ok(())
    }

    fn run_line(&mut self, line: &str) -> Result<(), Error> {
        let mut line_chars = line.chars().enumerate();

        let keyword = loop {
            match line_chars.next() {
                Some((i, c)) => {
                    if !c.is_alphabetic() {
                        break &line[0..i];
                    }
                }
                None => break line
            }
        };

        let remainder = line[keyword.len()..line.len()].trim();

        match keyword {
            "PRINT" => {
                self.print(remainder)?;
            },
            "LIST" => {
                for line in self.lines.iter() {
                    println!("{}", line);
                }
            },
            _ => {
                return Err(Error::UnrecognisedKeyword(keyword.to_string()));
            }
        }

        Ok(())
    }

    fn print(&self, expr_list: &str) -> Result<(), Error> {
        let string = Self::get_string(expr_list)?.unwrap();
        println!("{}", string);
        let expr_list = Self::consume_chars(expr_list, string.len()).trim();

        if expr_list.is_empty() {
            return Ok(());
        }

        loop {
            let expr_list = Self::consume_char(expr_list, ',')?.trim();
            let string = Self::get_string(expr_list)?.unwrap();
            println!("{}", string);
            let expr_list = Self::consume_chars(expr_list, string.len()).trim();

            if expr_list.is_empty() {
                return Ok(());
            }
        }
    }

    fn get_string(expr_list: &str) -> Result<Option<&str>, Error> {
        if expr_list.starts_with('"') {
            let str_end = expr_list[1..expr_list.len()].find('"');
            match str_end {
                Some(str_end) => Ok(Some(&expr_list[0..=(str_end + 1)])),
                None => Err(Error::UnmatchedQuote)
            }
            
        } else {
            Ok(None)
        }
    }

    fn consume_chars(s: &str, char_count_to_consume: usize) -> &str {
        &s[char_count_to_consume..s.len()]
    }

    fn consume_char(s: &str, char_to_consume: char) -> Result<&str, Error> {
        let found_char_pos = s.find(char_to_consume);
        match found_char_pos {
            Some(pos) => Ok(&s[(pos + 1)..s.len()]),
            None => Err(Error::Expected(char_to_consume))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Line;

    #[test]
    fn test_line_parsing() {
        {
            let line = Line::try_from("10   PRINT").unwrap();
            assert!(line.index == Some(10));
            assert!(line.line == "PRINT");
        }

        {
            let line = Line::try_from("10").unwrap();
            assert!(line.index == Some(10));
            assert!(line.line == "");
        }
    }
}