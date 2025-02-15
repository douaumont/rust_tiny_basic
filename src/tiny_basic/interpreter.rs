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

use ascii::{AsciiChar, AsciiStr};

use crate::tiny_basic::result;
use crate::tiny_basic::code_line::parse_line;
use crate::tiny_basic::types;
use crate::tiny_basic::error::Error as TinyBasicError;

use std::collections::{BTreeMap, HashMap};

use crate::tiny_basic::char_stream::AsciiCharStream; 
use crate::tiny_basic::code_line::LineNumber;

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

    pub fn execute<'a>(&mut self, line: &'a str) -> result::Result<()> {
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

    fn run_line<'a>(&mut self, line: &'a str) -> result::Result<()> {
        let line = AsciiStr::from_ascii(line.as_bytes())
        .expect("Expected line to be ASCII");
        let mut line = AsciiCharStream::from_ascii_str(line);

        let keyword = line.consume_keyword().ok_or(TinyBasicError::ExpectedKeyword)?.as_str();

        match keyword {
            "PRINT" => {
                self.print(&mut line)
            },
            _ => Err(TinyBasicError::UnrecognisedKeyword)
        }
    }

    fn print<'a>(&self, expr_list: &mut AsciiCharStream) -> result::Result<()> {
        if let Some(string) = expr_list.consume_string() {
            print!("{} ", string);
        } else {
            let expr_value = self.expression(expr_list)?;
            print!("{} ", expr_value);
        }

        while expr_list.consume_char(ascii::AsciiChar::Comma).is_some() {
            if let Some(string) = expr_list.consume_string() {
                print!("{} ", string);
            } else {
                let expr_value = self.expression(expr_list)?;
                print!("{} ", expr_value);
            }
        }

        println!();

        Ok(())
    }

    fn expression(&self, line: &mut AsciiCharStream) -> result::Result<types::Number> {
        let sign = line.consume_char_if(is_plus_or_minus);
        let sign: types::Number = match sign {
            Some(sign) => {
                get_sign_value(sign)
            },
            None => 1,
        };
        
        let mut total_term = sign * self.term(line)?;
        while let Some(sign) = line.consume_char_if(is_plus_or_minus) {
            let sign = get_sign_value(sign);
            let other = self.term(line)?;
            total_term += sign * other;
        }
        Ok(total_term)
    }

    fn term(&self, line: &mut AsciiCharStream) -> result::Result<types::Number> {
        let mut total_factor = self.factor(line)?;
        if let Some(op) = line.consume_char_if(is_slash_or_asterisk) {
            let other = self.factor(line)?;
            match op {
                ascii::AsciiChar::Slash => total_factor /= other,
                ascii::AsciiChar::Asterisk => total_factor *= other,
                _ => return Err(TinyBasicError::UnexpectedOperator),
            }
        }
        Ok(total_factor)
    }

    fn factor(&self, line: &mut AsciiCharStream) -> result::Result<types::Number>  {
        if let Some(var_name) = line.consume_var() {
            todo!("Variables will be implemented later");
        } else if let Some(number) = line.consume_number() {
            let number: types::Number = number.as_str().parse()?;
            Ok(number)
        } else if line.consume_char(AsciiChar::ParenOpen).is_some() {
            let expr_value = self.expression(line)?;
            line
                .consume_char(AsciiChar::ParenClose)
                .ok_or(TinyBasicError::Expected(')'))?;
            Ok(expr_value)
        } else {
            Err(TinyBasicError::FactorCouldNotBeParsed)
        }
    }
}

fn is_plus_or_minus(ch: &AsciiChar) -> bool {
    match *ch {
        ascii::AsciiChar::Plus | ascii::AsciiChar::Minus => true,
        _ => false
    }
}

fn is_slash_or_asterisk(ch: &AsciiChar) -> bool {
    match *ch {
        ascii::AsciiChar::Slash | ascii::AsciiChar::Asterisk => true,
        _ => false
    }
}

fn get_sign_value(ch: AsciiChar) -> types::Number {
    assert!(is_plus_or_minus(&ch));
    match ch {
        ascii::AsciiChar::Plus => 1,
        ascii::AsciiChar::Minus => -1,
        _ => unreachable!()
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