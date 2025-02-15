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

use crate::tiny_basic::error::Error as TinyBasicError;

pub type LineNumber = i32;

pub const MIN_LINE_NUMBER: LineNumber = 1;
pub const MAX_LINE_NUMBER: LineNumber = 10000;

pub type CodeLine<'a> = (Option<LineNumber>, &'a str);

pub fn parse_line<'a>(line: &'a str) -> Result<CodeLine<'a>, TinyBasicError> {
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