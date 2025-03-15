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

use ascii::{AsAsciiStr, AsciiChar, AsciiString};

use crate::tiny_basic;
use crate::tiny_basic::types;
use crate::tiny_basic::error::{Error as TinyBasicError, ErrorKind as TinyBasicErrorKind};
use crate::tiny_basic::program_storage::ProgramStorage;


use crate::tiny_basic::char_stream::AsciiCharStream;

use crate::tiny_basic::char_stream::Keyword;

use super::char_stream::Statement;

type Environment = HashMap<AsciiString, types::Number>;
type ReturnStack = Vec<types::LineIndex>;

pub struct Interpreter {
    next_line_to_execute: Option<types::LineIndex>,
    current_line_number: Option<types::LineIndex>,
    environment: Environment,
    return_stack: ReturnStack
}

impl<'line_source> Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
            next_line_to_execute: None,
            current_line_number: None,
            return_stack: ReturnStack::new()
        }
    }

    pub fn run(&mut self, program: &'line_source ProgramStorage) -> tiny_basic::Result<'line_source, ()> {
        match program.get_first_line_index() {
            Some(index) => {
                self.next_line_to_execute = Some(index);
            },
            None => return Ok(()),
        }
        
        while let Some(current_line) = self.next_line_to_execute {
            self.current_line_number = Some(current_line);
            self.next_line_to_execute = program.get_following_line_index(current_line);

            if let Some(line) = program.get_line(current_line) {
                self.execute(&mut AsciiCharStream::from_ascii_str(line))?;
            }
        }

        Ok(())
    }

    pub fn execute(&mut self, stmt: &mut AsciiCharStream<'line_source>) -> tiny_basic::Result<'line_source, ()> {
        let statement = 
            stmt
            .consume_statement()
            .ok_or(TinyBasicError::from_context(stmt, TinyBasicErrorKind::ExpectedStatement, self.current_line_number))?;

        match statement {
            Statement::Print => self.print_stmt(stmt),
            Statement::If => self.if_stmt(stmt),
            Statement::Goto => self.goto_stmt(stmt),
            Statement::Let => self.let_stmt(stmt),
            Statement::Gosub => self.gosub_stmt(stmt),
            Statement::Return => self.return_stmt(),
            Statement::End => self.end_stmt(),
            Statement::Input => self.input_stmt(stmt),
        }.or_else(|error| {
            match error.get_kind() {
                TinyBasicErrorKind::ExecutionReachedEnd => Ok(()),
                _ => Err(error)
            }
        }.and_then(|_| stmt
            .is_empty()
            .then_some(())
            .ok_or(TinyBasicError::from_context(stmt, TinyBasicErrorKind::UnexpectedTokensAtEndOfLine, self.current_line_number))))

    }

    fn print_stmt(&mut self, expr_list: &mut AsciiCharStream<'line_source>) -> tiny_basic::Result<'line_source, ()> {
        if let Some(string) = expr_list.consume_string()? {
            print!("{} ", string);
        } else {
            let expr_value = self.expression(expr_list)?;
            print!("{} ", expr_value);
        }

        while expr_list.consume_char(ascii::AsciiChar::Comma).is_some() {
            if let Some(string) = expr_list.consume_string()? {
                print!("{} ", string);
            } else {
                let expr_value = self.expression(expr_list)?;
                print!("{} ", expr_value);
            }
        }

        println!();

        Ok(())
    }

    fn if_stmt(&mut self, stmt: &mut AsciiCharStream<'line_source>) -> tiny_basic::Result<'line_source, ()> {
        let lhs = self.expression(stmt)?;
        let relop = stmt
            .consume_relop()
            .ok_or(TinyBasicError::from_context(stmt, TinyBasicErrorKind::ExpectedRelationalOperator, self.current_line_number))?;
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
                .ok_or(TinyBasicError::from_context(stmt, TinyBasicErrorKind::ExpectedKeyword, self.current_line_number))?;
            self.execute(stmt)
        } else {
            stmt.flush();
            Ok(())
        }
    }

    fn goto_stmt(&mut self, stmt: &mut AsciiCharStream<'line_source>) -> tiny_basic::Result<'line_source, ()> {
        let next_line_index: types::LineIndex = self.expression(stmt)?.try_into()?;
        self.next_line_to_execute = Some(next_line_index);
        Ok(())
    }

    fn let_stmt(&mut self, stmt: &mut AsciiCharStream<'line_source>) -> tiny_basic::Result<'line_source, ()> {
        let var_name = 
            stmt
            .consume_var();

        if var_name.is_none() {
            return Err(TinyBasicError::from_context(stmt, TinyBasicErrorKind::ExpectedVariableName, self.current_line_number));
        }
        let var_name = var_name.unwrap().to_owned();

        stmt
            .consume_char(AsciiChar::Equal)
            .ok_or(TinyBasicError::from_context(stmt, TinyBasicErrorKind::Expected('='), self.current_line_number))?;
        let value = self.expression(stmt)?;
        self.environment.insert(var_name, value);
        Ok(())
    }

    fn gosub_stmt(&mut self, stmt: &mut AsciiCharStream<'line_source>) -> tiny_basic::Result<'line_source, ()> {
        let subroutine_address: types::LineIndex = self.expression(stmt)?.try_into()?;
        let return_address = 
            self.next_line_to_execute
            .ok_or(TinyBasicError::from_context(stmt, TinyBasicErrorKind::CommandNotUsableInInteractiveMode, self.current_line_number))?;

        self.return_stack.push(return_address);
        self.next_line_to_execute = Some(subroutine_address);
        Ok(())
    }

    fn return_stmt(&mut self) -> tiny_basic::Result<'line_source, ()> {
        let return_address = self
            .return_stack
            .pop()
            .ok_or(TinyBasicError::from( TinyBasicErrorKind::ReturnOnEmptyStack))?;
        self.next_line_to_execute = Some(return_address);
        Ok(())
    }

    fn input_stmt(&mut self, var_list: &mut AsciiCharStream<'line_source>) -> tiny_basic::Result<'line_source, ()> {
        self.input_var(var_list)?;
        while var_list.consume_char(AsciiChar::Comma).is_some() {
            self.input_var(var_list)?;
        }
        Ok(())
    }

    fn input_var(&mut self,  var_list: &mut AsciiCharStream<'line_source>) -> tiny_basic::Result<'line_source, ()> {
        let var_name = var_list.consume_var();

        if var_name.is_none() {
            return Err(TinyBasicError::from_context(var_list, TinyBasicErrorKind::ExpectedVariableName, self.current_line_number));
        }
        let var_name = var_name.unwrap();

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

    fn get_user_input() -> tiny_basic::Result<'line_source, AsciiString> {
        let mut user_input = String::new();
        while let Ok(read_bytes) = stdin().read_line(&mut user_input) {
            if read_bytes > 0 {
                break;
            }
        }
        let user_input = user_input
            .trim()
            .as_ascii_str()
            .map_err(|error| TinyBasicError::from(TinyBasicErrorKind::from(error)))?;
        Ok(user_input.to_owned())
    }

    fn end_stmt(&mut self) -> tiny_basic::Result<'line_source, ()> {
        Err(TinyBasicError::from(TinyBasicErrorKind::ExecutionReachedEnd))
    }

    fn expression(&self, stmt: &mut AsciiCharStream<'line_source>) -> tiny_basic::Result<'line_source, types::Number> {
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

    fn term(&self, stmt: &mut AsciiCharStream<'line_source>) -> tiny_basic::Result<'line_source, types::Number> {
        let mut total_factor = self.factor(stmt)?;
        if let Some(op) = stmt.consume_char_if(is_slash_or_asterisk) {
            let other = self.factor(stmt)?;
            match op {
                ascii::AsciiChar::Slash => total_factor /= other,
                ascii::AsciiChar::Asterisk => total_factor *= other,
                _ => return Err(TinyBasicError::from_context(stmt, TinyBasicErrorKind::UnexpectedOperator, self.current_line_number)),
            }
        }
        Ok(total_factor)
    }

    fn factor(&self, stmt: &mut AsciiCharStream<'line_source>) -> tiny_basic::Result<'line_source, types::Number>  {
        if let Some(var_name) = stmt.consume_var() {
            Ok(self.environment
                .get(var_name)
                .cloned()
                .unwrap_or(0))
        } else if let Some(number) = stmt.consume_number() {
            let number: types::Number = 
                number
                .as_str()
                .parse()
                .map_err(|error| TinyBasicError::from_context(stmt, TinyBasicErrorKind::from(error), self.current_line_number))?;
            Ok(number)
        } else if stmt.consume_char(AsciiChar::ParenOpen).is_some() {
            let expr_value = self.expression(stmt)?;
            stmt
                .consume_char(AsciiChar::ParenClose)
                .ok_or(TinyBasicError::from_context(stmt, TinyBasicErrorKind::Expected(')'), self.current_line_number))?;
            Ok(expr_value)
        } else {
            Err(TinyBasicError::from_context(stmt, TinyBasicErrorKind::FactorCouldNotBeParsed, self.current_line_number))
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