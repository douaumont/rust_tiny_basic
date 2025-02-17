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

use ascii::AsciiStr;

use crate::tiny_basic::error::Error as TinyBasicError;
use crate::tiny_basic::char_stream::AsciiCharStream;
use crate::tiny_basic::types;

pub struct Line<'a> {
    pub index: Option<types::Number>,
    pub statement: &'a AsciiStr
}

impl<'a> TryFrom<&'a AsciiStr> for Line<'a> {
    type Error = TinyBasicError;

    fn try_from(value: &'a AsciiStr) -> Result<Self, Self::Error> {
        let mut char_stream = AsciiCharStream::from_ascii_str(value);
        if let Some(line_index) = char_stream.consume_number() {
            let line_index = line_index.as_str().parse::<types::Number>()?;
            Ok(Self{
                index: Some(line_index),
                statement: char_stream.flush()
            })
        } else {
            Ok(Self{
                index: None,
                statement: char_stream.flush()
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use ascii::{AsciiString, AsAsciiStr};

    use super::Line;

    #[test]
    fn test_line_no_number() {
        {
            let input = AsciiString::from_ascii(b"PRINT H").unwrap();
            let line = Line::try_from(input.as_ascii_str().unwrap()).unwrap();
            assert!(line.index.is_none());
            assert_eq!(line.statement, input.as_ascii_str().unwrap());
        }
    }

    #[test]
    fn test_line_with_number() {
        {
            let input = AsciiString::from_ascii(b"220 PRINT H").unwrap();
            let line = Line::try_from(input.as_ascii_str().unwrap()).unwrap();
            assert_eq!(line.index, Some(220));
            assert_eq!(line.statement.as_str(), "PRINT H");
        }
    }

    #[test]
    fn test_line_with_empty_statement() {
        {
            let input = AsciiString::from_ascii(b"220").unwrap();
            let line = Line::try_from(input.as_ascii_str().unwrap()).unwrap();
            assert_eq!(line.index, Some(220));
            assert!(line.statement.is_empty());
        }
    }
}