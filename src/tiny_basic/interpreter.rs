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

use std::io::{stdin, stdout, Write};
use std::collections::HashMap;

use ascii::{AsAsciiStr, AsciiChar, AsciiStr, AsciiString};

use crate::tiny_basic::result;
use crate::tiny_basic::code_line;
use crate::tiny_basic::types;
use crate::tiny_basic::error::{ErrorKind as TinyBasicError, Error};
use crate::tiny_basic::program_storage::ProgramStorage;


use crate::tiny_basic::char_stream::AsciiCharStream;

use crate::tiny_basic::char_stream::Keyword;

type Environment = HashMap<AsciiString, types::Number>;
type ReturnStack = Vec<types::Number>;

pub struct Interpreter<'a> {
    lines: ProgramStorage<'a>,
    next_line_to_execute: Option<types::Number>,
    environment: Environment,
    return_stack: ReturnStack
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Interpreter {
            lines: ProgramStorage::new(),
            environment: Environment::new(),
            next_line_to_execute: None,
            return_stack: ReturnStack::new()
        }
    }

    pub fn execute(&mut self, line: &AsciiStr) -> result::Result<()> {
        let line = 
            code_line::Line::try_from(line.trim())?;

        match line.index {
            Some(i) => {
                if line.statement.is_empty() {
                    self.erase_line(i);
                } else {
                    self.lines.insert_line(i, line.statement.to_owned());
                }
            },
            None => self.run_line(line.statement)?
        }
        
        Ok(())
    }

    fn erase_line(&mut self, line_index: types::Number) {
        self.lines.erase_line(line_index);
    }

    fn run_line(&mut self, line: &AsciiStr) -> result::Result<()> {
        let mut line = AsciiCharStream::from_ascii_str(line);

        self.statement(&mut line)?;

        match line.is_empty() {
            true => Ok(()),
            false => Err(TinyBasicError::ExpectedEndOfLine)
        }
    }

    fn statement(&mut self, stmt: &mut AsciiCharStream) -> result::Result<()> {
        let keyword = stmt.consume_keyword().ok_or(TinyBasicError::ExpectedKeyword)?;

        match keyword {
            Keyword::Print => self.print_stmt(stmt),
            Keyword::If => self.if_stmt(stmt),
            Keyword::Run => self.run_stmt(),
            Keyword::List => self.list_stmt(),
            Keyword::Clear => self.clear_stmt(),
            Keyword::Goto => self.goto_stmt(stmt),
            Keyword::Then => Err(TinyBasicError::UnexpectedKeyword),
            Keyword::Let => self.let_stmt(stmt),
            Keyword::Gosub => self.gosub_stmt(stmt),
            Keyword::Return => self.return_stmt(),
            Keyword::End => self.end_stmt(),
            Keyword::Input => self.input_stmt(stmt),
        }
    }

    fn print_stmt(&mut self, expr_list: &mut AsciiCharStream) -> result::Result<()> {
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

    fn if_stmt(&mut self, stmt: &mut AsciiCharStream) -> result::Result<()> {
        let lhs = self.expression(stmt)?;
        let relop = stmt
            .consume_relop()
            .ok_or(TinyBasicError::ExpectedRelationalOperator)?;
        let rhs = self.expression(stmt)?;

        let condition = match relop {
            super::char_stream::RelationalOperator::Less => lhs < rhs,
            super::char_stream::RelationalOperator::Greater => lhs > rhs,
            super::char_stream::RelationalOperator::LessEqual => lhs <= rhs,
            super::char_stream::RelationalOperator::GreaterEqual => lhs >= rhs,
            super::char_stream::RelationalOperator::NotEqual => lhs != rhs,
            super::char_stream::RelationalOperator::Equal => lhs == rhs,
        };

        if condition {
            stmt
                .consume_keyword()
                .and_then(|keyword| {
                    match keyword {
                        Keyword::Then => Some(()),
                        _ => None
                    }
                })
                .ok_or(TinyBasicError::ExpectedKeyword)?;
            self.statement(stmt)
        } else {
            stmt.flush();
            Ok(())
        }
    }

    fn goto_stmt(&mut self, stmt: &mut AsciiCharStream) -> result::Result<()> {
        let next_line_index = self.expression(stmt)?;
        self.next_line_to_execute = Some(next_line_index);
        Ok(())
    }

    fn let_stmt(&mut self, stmt: &mut AsciiCharStream) -> result::Result<()> {
        let var_name = 
            stmt
            .consume_var()
            .ok_or(TinyBasicError::ExpectedVariableName)?.to_owned();
        stmt
            .consume_char(AsciiChar::Equal)
            .ok_or(TinyBasicError::Expected('='))?;
        let value = self.expression(stmt)?;
        self.environment.insert(var_name, value);
        Ok(())
    }

    fn gosub_stmt(&mut self, stmt: &mut AsciiCharStream) -> result::Result<()> {
        let subroutine_address = self.expression(stmt)?;
        let current_line_index = 
            self.next_line_to_execute
            .ok_or(TinyBasicError::CommandNotUsableInInteractiveMode)?;

        self.return_stack.push(current_line_index);
        self.next_line_to_execute = Some(subroutine_address);
        Ok(())
    }

    fn return_stmt(&mut self) -> result::Result<()> {
        let return_address = self
            .return_stack
            .pop()
            .ok_or(TinyBasicError::ReturnOnEmptyStack)?;
        self.next_line_to_execute = Some(return_address);
        Ok(())
    }

    fn input_stmt(&mut self, var_list: &mut AsciiCharStream) -> result::Result<()> {
        self.input_var(var_list)?;
        while var_list.consume_char(AsciiChar::Comma).is_some() {
            self.input_var(var_list)?;
        }
        Ok(())
    }

    fn input_var(&mut self,  var_list: &mut AsciiCharStream) -> result::Result<()> {
        let var_name = var_list
            .consume_var()
            .ok_or(TinyBasicError::ExpectedVariableName)?;

        print!("{}? ", var_name);
        stdout().flush();
        let user_input = Self::get_user_input()?;
        if let Some(number) = user_input.as_str().parse::<types::Number>().ok() {
            self.environment.insert(var_name.to_owned(), number);
        } else {
            let first_char_code = *user_input.as_bytes().iter().nth(0).expect("User input should not be empty");
            self.environment.insert(var_name.to_owned(), first_char_code as types::Number);
        }
        Ok(())
    } 

    fn get_user_input() -> result::Result<AsciiString> {
        let mut user_input = String::new();
        while let Ok(read_bytes) = stdin().read_line(&mut user_input) {
            if read_bytes > 0 {
                break;
            }
        }
        Ok(user_input.trim().as_ascii_str()?.to_owned())
    }

    fn end_stmt(&mut self) -> result::Result<()> {
        Err(TinyBasicError::ExecutionReachedEnd)
    }

    fn run_stmt(&mut self) -> result::Result<()> {
        let execution_res = self.run_lines();
        self.next_line_to_execute = None;
        execution_res
    }

    fn run_lines(&mut self) -> result::Result<()> {
        match self.lines.get_first_line_index() {
            Some(index) => {
                self.next_line_to_execute = Some(index);
            },
            None => return Ok(()),
        }
        
        while let Some(current_line) = self.next_line_to_execute {
            self.next_line_to_execute = self.lines.get_following_line_index(current_line);

            if let Some(line) = self.lines.get_line(current_line) {
                self.run_line(&line)?;   
            }
        }

        Ok(())
    }

    fn list_stmt(&self) -> result::Result<()> {
        for (i, line) in self.lines.iter() {
            println!("{} {}", i, line);
        }
        Ok(())
    }

    fn clear_stmt(&mut self) -> result::Result<()> {
        self.lines.clear();
        Ok(())
    }

    fn expression(&self, stmt: &mut AsciiCharStream) -> result::Result<types::Number> {
        let sign = stmt.consume_char_if(is_plus_or_minus);
        let sign: types::Number = match sign {
            Some(sign) => {
                get_sign_value(sign)
            },
            None => 1,
        };
        
        let mut total_term = sign * self.term(stmt)?;
        while let Some(sign) = stmt.consume_char_if(is_plus_or_minus) {
            let sign = get_sign_value(sign);
            let other = self.term(stmt)?;
            total_term += sign * other;
        }
        Ok(total_term)
    }

    fn term(&self, stmt: &mut AsciiCharStream) -> result::Result<types::Number> {
        let mut total_factor = self.factor(stmt)?;
        if let Some(op) = stmt.consume_char_if(is_slash_or_asterisk) {
            let other = self.factor(stmt)?;
            match op {
                ascii::AsciiChar::Slash => total_factor /= other,
                ascii::AsciiChar::Asterisk => total_factor *= other,
                _ => return Err(TinyBasicError::UnexpectedOperator),
            }
        }
        Ok(total_factor)
    }

    fn factor(&self, stmt: &mut AsciiCharStream) -> result::Result<types::Number>  {
        if let Some(var_name) = stmt.consume_var() {
            Ok(self.environment
                .get(var_name)
                .cloned()
                .unwrap_or(0))
        } else if let Some(number) = stmt.consume_number() {
            let number: types::Number = number.as_str().parse()?;
            Ok(number)
        } else if stmt.consume_char(AsciiChar::ParenOpen).is_some() {
            let expr_value = self.expression(stmt)?;
            stmt
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