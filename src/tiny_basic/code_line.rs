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

use std::{iter::Peekable, str::Chars};
use substring::Substring;

use crate::tiny_basic::error::Error as TinyBasicError;

pub type LineNumber = i32;

pub const MIN_LINE_NUMBER: LineNumber = 1;
pub const MAX_LINE_NUMBER: LineNumber = 10000;

pub type CodeLine<'a> = (Option<LineNumber>, &'a str);

pub fn parse_line<'a>(line: &'a str) -> Result<CodeLine<'a>, TinyBasicError::<'a>> {
    if let Some(line_number_end) = get_line_number_end(line) {
        let line_number: LineNumber = line[0..line_number_end].parse().expect("Line number should be parsed.");

        if (MIN_LINE_NUMBER..=MAX_LINE_NUMBER).contains(&line_number) {
            Ok((
                Some(line_number),
                line[line_number_end..line.len()].trim_ascii_start()
            ))
        } else {
            Err(TinyBasicError::InvalidLineNumber)
        }
    } else {
        Ok((
            None, line
        ))
    }
}

pub struct CharStream<'a> {
    stream: &'a str,
    current_pos: usize
}

impl<'a> CharStream<'a> {
    pub fn from_str(s: &'a str) -> Self {
        Self {
            stream: s,
            current_pos: 0
        }
    }

    /// Tries to consume character at the front of the stream
    /// if this character matches one of those in the `char_set`.
    /// In this case the stream is advanced furtther by one
    /// and the matched char is returned.
    /// `None` otherwise
    pub fn try_consume_char(&mut self, char_set: &[char]) -> Option<char> {
        if let Some(current_char) = self.stream.chars().nth(self.current_pos) {
            if char_set.contains(&current_char) {
                self.current_pos += 1;
                return Some(current_char);
            }
        }
        None
    }

    /// Consume string consisting of uppercase letters
    pub fn get_keyword(&mut self) -> &'a str {
        let keyword_start = self.current_pos;
        let keyword_past_end = loop {
            match self.stream.chars().nth(self.current_pos) {
                Some(c) => if !c.is_alphabetic() {
                    break self.current_pos;
                } else {
                    self.current_pos += 1;
                },
                None => break self.current_pos
            }
        };

        let keyword = self.stream.substring(keyword_start, keyword_past_end);
        self.trim_start();
        &keyword
    }

    pub fn is_empty(&mut self) -> bool {
        self.current_pos == self.stream.len()
    }

    fn trim_start(&mut self) {
        let offset = self.stream
            .chars()
            .skip(self.current_pos)
            .position(|c| !c.is_whitespace());
        match offset {
            Some(offset) => self.current_pos += offset,
            None => self.current_pos = self.stream.len(),
        }
    }
}

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

#[cfg(test)]
mod char_stream_tests {
    use super::CharStream;

    #[test]
    fn test_empty() {
        {
            let mut stream = CharStream::from_str("");
            assert!(stream.is_empty());
        }

        {
            let mut stream = CharStream::from_str("something");
            assert!(!stream.is_empty());
        }
    }

    #[test]
    fn test_get_keyword() {
        {
            let mut stream = CharStream::from_str("PRINT");
            assert_eq!(stream.get_keyword(), "PRINT");
        }

        {
            let mut stream = CharStream::from_str("PRINT IF");
            assert_eq!(stream.get_keyword(), "PRINT");
            assert_eq!(stream.get_keyword(), "IF");
        }
    }

    #[test]
    fn test_try_consume_char() {
        {
            let mut stream = CharStream::from_str("123H");
            assert_eq!(stream.try_consume_char(&['1', '2', '3']), Some('1'));
            assert_eq!(stream.try_consume_char(&['1', '2', '3']), Some('2'));
            assert_eq!(stream.try_consume_char(&['1', '2', '3']), Some('3'));
            assert_eq!(stream.try_consume_char(&['1', '2', '3']), None);
            assert_eq!(stream.try_consume_char(&['H']), Some('H'));
        }
    }
}