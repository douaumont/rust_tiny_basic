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

use std::cell::OnceCell;

use ascii::{AsciiStr, AsciiString};

use crate::tiny_basic::char_stream::AsciiCharStream;
use crate::tiny_basic::types;

#[derive(Debug)]
pub struct Error<'ctx> {
    line_number: Option<types::Number>,
    context: OnceCell<&'ctx AsciiStr>,
    location: OnceCell<usize>,
    kind: ErrorKind
}

impl<'ctx> Error<'ctx> {
    pub fn from_context(context: &AsciiCharStream<'ctx>, kind: ErrorKind, line_number: Option<types::Number>) -> Self {
        Self {
            line_number: line_number,
            context: OnceCell::from(context.get_stream()),
            location: OnceCell::from(context.get_location()),
            kind: kind
        }
    }

    pub fn set_context(self, context: &AsciiCharStream<'ctx>) -> Self {
        self.context.set(context.get_stream());
        self
    }

    pub fn set_line_number(mut self, line_number: Option<types::Number>) -> Self {
        self.line_number = line_number;
        self
    }

    pub fn get_kind(&self) -> ErrorKind {
        self.kind.clone()
    }
}

impl<'ctx> From<ErrorKind> for Error<'ctx> {
    fn from(value: ErrorKind) -> Self {
        Self {
            line_number: None,
            context: OnceCell::new(),
            location: OnceCell::new(),
            kind: value
        }
    }
}

impl<'ctx> std::fmt::Display for Error<'ctx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error: {}", self.kind)?;
        writeln!(f)?;
        let error_location = match self.line_number {
            Some(i) => {
                write!(f, "{} ", i)?;
                // The length of the line number in digits 
                // (which is its log10 + 1) and the space char
                i.checked_ilog10().expect("Line number should be greater than zero") + 1 + 1
            },
            None => 0,
        } as usize + self.location.get_or_init(|| 0);
        let context = self.context.get().expect("Error context should be set");
        let context_length = context.len() + error_location;

        writeln!(f, "{}", context)?;

        for _ in 0..error_location {
            write!(f, " ")?;
        }
        
        const UNDERSCORING_CHAR: char = '^';

        if error_location < context_length {
            for _ in error_location..context_length {
                write!(f, "{}", UNDERSCORING_CHAR)?;
            }
        } else {
            for _ in context_length..(context_length + 3) {
                write!(f, "{}", UNDERSCORING_CHAR)?
            }
        }

        Ok(())
    }
}

// Not really tests, it's just easier for me to see how the errors are printed
#[cfg(test)]
mod error_test {
    use ascii::AsAsciiStr;
    use crate::tiny_basic::{char_stream::AsciiCharStream, error::ErrorKind};

    #[test]
    fn test_error_formatting_1() {
        let mut ctx = AsciiCharStream::from_ascii_str("PRINT 2 +".as_ascii_str().unwrap());
        ctx.consume_keyword();
        let error = super::Error::from_context(&ctx, super::ErrorKind::ExpectedKeyword, None);
        println!("{}", error);
    }

    #[test]
    fn test_error_formatting_2() {
        let mut ctx = AsciiCharStream::from_ascii_str("RETURN".as_ascii_str().unwrap());
        ctx.consume_keyword();
        let error = super::Error::from(ErrorKind::ReturnOnEmptyStack);
        println!("{}", error.set_context(&ctx).set_line_number(Some(10)));
    }


    #[test]
    fn test_error_formatting_on_empty_with_line_lumber() {
        let mut ctx = AsciiCharStream::from_ascii_str("PRINT VAR".as_ascii_str().unwrap());
        ctx.consume_keyword();
        ctx.consume_var();
        let error = super::Error::from_context(&ctx, super::ErrorKind::ExpectedKeyword, Some(123));
        println!("{}", error);
    }
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Expected(char),
    ExpectedKeyword,
    UnexpectedOperator,
    FactorCouldNotBeParsed,
    UnexpectedTokensAtEndOfLine,
    ExpectedRelationalOperator,
    UnexpectedKeyword,
    ExpectedVariableName,
    NumberParseError(std::num::IntErrorKind),
    CommandNotUsableInInteractiveMode,
    ReturnOnEmptyStack,
    ExecutionReachedEnd,
    ExpectedAsciiInput,
    ExpectedStatement,
    ExpectedCommand
}

impl From<std::num::ParseIntError> for ErrorKind {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::NumberParseError(value.kind().clone())
    }
}

impl From<ascii::AsAsciiStrError> for ErrorKind {
    fn from(_value: ascii::AsAsciiStrError) -> Self {
        ErrorKind::ExpectedAsciiInput
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::Expected(c) => write!(f, "Expected {}", c),
            ErrorKind::ExpectedKeyword => write!(f, "Expected keyword"),
            ErrorKind::UnexpectedOperator => write!(f, "Unexpected operator"),
            ErrorKind::FactorCouldNotBeParsed => write!(f, "Factor could not be parsed"),
            ErrorKind::UnexpectedTokensAtEndOfLine => write!(f, "Unexpected tokens at the end of line"),
            ErrorKind::ExpectedRelationalOperator => write!(f, "Expected relational operator"),
            ErrorKind::UnexpectedKeyword => write!(f, "Unexpected keyword"),
            ErrorKind::ExpectedVariableName => write!(f, "Expected variable name"),
            ErrorKind::NumberParseError(_) => write!(f, "Number could not be parsed"),
            ErrorKind::CommandNotUsableInInteractiveMode => write!(f, "This command is not intended to be used in interactive mode"),
            ErrorKind::ReturnOnEmptyStack => write!(f, "Attempt to RETURN while the return stack is empty"),
            ErrorKind::ExecutionReachedEnd => unreachable!("Should not display ExecutionReachedEnd"),
            ErrorKind::ExpectedAsciiInput => write!(f, "All input is expected to be ASCII-only"),
            ErrorKind::ExpectedStatement => write!(f, "Expected statement"),
            ErrorKind::ExpectedCommand => write!(f, "Expected command"),
        }
    }
}