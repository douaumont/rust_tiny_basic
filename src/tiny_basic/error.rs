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

use crate::tiny_basic::char_stream::AsciiCharStream;
use crate::tiny_basic::types;

#[derive(Debug)]
pub struct Error<'a> {
    line_number: Option<types::Number>,
    context: &'a AsciiStr,
    location: usize,
    kind: ErrorKind
}

impl<'a> Error<'a> {
    pub fn from(context: &'a AsciiCharStream, kind: ErrorKind, line_number: Option<types::Number>) -> Self {
        Self {
            line_number: line_number,
            context: context.get_stream(),
            location: context.get_location(),
            kind: kind
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    UnrecognisedKeyword,
    Expected(char),
    ExpectedKeyword,
    ExpectedStringOrExpression,
    UnexpectedOperator,
    NumberCouldNotBeParsed,
    FactorCouldNotBeParsed,
    ExpectedEndOfLine,
    ExpectedRelationalOperator,
    UnexpectedKeyword,
    ExpectedVariableName,
    NumberParseError(std::num::IntErrorKind),
    CommandNotUsableInInteractiveMode,
    ReturnOnEmptyStack,
    ExecutionReachedEnd,
    ExpectedAsciiInput
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
            ErrorKind::UnrecognisedKeyword => todo!(),
            ErrorKind::Expected(c) => write!(f, "Expected '{}'", c),
            ErrorKind::ExpectedKeyword => todo!(),
            ErrorKind::ExpectedStringOrExpression => todo!(),
            ErrorKind::UnexpectedOperator => todo!(),
            ErrorKind::NumberCouldNotBeParsed => todo!(),
            ErrorKind::FactorCouldNotBeParsed => todo!(),
            ErrorKind::ExpectedEndOfLine => todo!(),
            ErrorKind::ExpectedRelationalOperator => todo!(),
            ErrorKind::UnexpectedKeyword => todo!(),
            ErrorKind::ExpectedVariableName => todo!(),
            ErrorKind::NumberParseError(int_error_kind) => todo!(),
            ErrorKind::CommandNotUsableInInteractiveMode => todo!(),
            ErrorKind::ReturnOnEmptyStack => todo!(),
            ErrorKind::ExecutionReachedEnd => todo!(),
            ErrorKind::ExpectedAsciiInput => todo!(),
        }
    }
}