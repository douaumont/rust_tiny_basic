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

use crate::tiny_basic::code_line::{MIN_LINE_NUMBER, MAX_LINE_NUMBER};

#[derive(Debug)]
pub enum Error<'a> {
    InvalidLineNumber,
    UnrecognisedKeyword(&'a str),
    Expected(char)
}

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidLineNumber => write!(f, "Line numbers should be in range [{}; {}]", MIN_LINE_NUMBER, MAX_LINE_NUMBER),
            Error::UnrecognisedKeyword(s) => write!(f, "Unrecognised keyword: {}", s),
            Error::Expected(c) => write!(f, "Expected '{}'", c)
        }
    }
}