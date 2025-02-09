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
use crate::tiny_basic::code_line::parse_line;

use std::collections::{BTreeMap, HashMap};

use super::code_line::{CharStream, LineNumber};
use super::error;
use super::types::{self, Number};

const ACCEPTABLE_VAR_NAMES: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];


pub struct Interpreter {
    lines: BTreeMap<LineNumber, String>,
    environment: HashMap<char, types::Number>
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            lines: BTreeMap::new(),
            environment: HashMap::new()
        }
    }

    pub fn execute<'a>(&mut self, line: &'a str) -> Result<(), TinyBasicError::<'a>> {
        assert!(!line.starts_with(' ') && !line.ends_with(' '));

        let parsed_line = parse_line(line)?;
        let line_index = parsed_line.0;

        match line_index {
            Some(i) => {
                if parsed_line.1.is_empty() {
                    self.erase_line(i);
                } else {
                    self.lines.insert(i, parsed_line.1.to_string());
                }
            },
            None => self.run_line(line)?
        }
        
        Ok(())
    }

    fn erase_line(&mut self, line_number: LineNumber) {
        self.lines.remove(&line_number);
    }

    fn run_line<'a>(&mut self, line: &'a str) -> Result<(), TinyBasicError::<'a>> {
        let mut line = CharStream::from_str(line);

        Ok(())
    }

    fn print<'a>(&self, expr_list: &mut CharStream) -> Result<(), TinyBasicError::<'a>> {
        Ok(())
    }

    fn string<'a>(expr_list: &mut CharStream) -> Result<&'a str, TinyBasicError<'a>> {
        Ok("")
    }

    fn get_string(expr_list: &str) -> Result<Option<&str>, TinyBasicError> {
        if expr_list.starts_with('"') {
            let str_end = expr_list[1..expr_list.len()].find('"');
            match str_end {
                Some(str_end) => Ok(Some(&expr_list[0..=(str_end + 1)])),
                None => Err(TinyBasicError::Expected('"'))
            }
            
        } else {
            Ok(None)
        }
    }

    fn expression(&self, line: &mut CharStream) -> types::Number {
        let sign: types::Number = match line.try_consume_char(&['+', '-']) {
            Some('+') | None => 1,
            Some('-') => -1,
            Some(_) => panic!()
        };
        
        self.term(line)
    }

    fn term(&self, line: &mut CharStream) -> types::Number {
        self.factor(line)
    }

    fn factor(&self, line: &mut CharStream) -> types::Number {
        if let Some(var) = line.try_consume_char(&ACCEPTABLE_VAR_NAMES) {
            *self.environment.get(&var).unwrap()
        } else if let Some(first_digit) = line.try_consume_char(&DIGITS) {
            Self::number(first_digit, line)
        } else if let Some(_bracket) = line.try_consume_char(&['(']) {
            let res = self.expression(line);
            line.try_consume_char(&[')']).ok_or(error::Error::Expected(')')).unwrap();
            res
        } else {
            panic!();
        }
    }

    fn number(first_digit: char, line: &mut CharStream) -> types::Number {
        let mut number = first_digit
            .to_digit(10)
            .unwrap() as types::Number;
        while let Some(digit) = line.try_consume_char(&DIGITS) {
            number *= 10;
            number += digit.to_digit(10).unwrap() as types::Number;
        }
        number
    }

    fn consume_chars(s: &str, char_count_to_consume: usize) -> &str {
        &s[char_count_to_consume..s.len()]
    }

    fn consume_char(s: &str, char_to_consume: char) -> Result<&str, TinyBasicError> {
        let found_char_pos = s.find(char_to_consume);
        match found_char_pos {
            Some(pos) => Ok(&s[(pos + 1)..s.len()]),
            None => Err(TinyBasicError::Expected(char_to_consume))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse_line;

    #[test]
    fn test_line_parsing() {
        {
            let (i, line) =parse_line("10   PRINT").unwrap();
            assert!(i == Some(10));
            assert!(line == "PRINT");
        }

        {
            let (i, line) = parse_line("10").unwrap();
            assert!(i == Some(10));
            assert!(line.is_empty());
        }

        {
            let (i, line) = parse_line("PRINT \"HELLO\"").unwrap();
            assert!(i == None);
            assert!(line == "PRINT \"HELLO\"");
        }
    }
}