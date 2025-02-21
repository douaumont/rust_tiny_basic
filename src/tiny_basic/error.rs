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

#[derive(Debug)]
pub enum Error {
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
    GosubCannotBeUsedInInteractiveMode,
    ReturnOnEmptyStack,
    ExecutionReachedEnd,
    ExpectedAsciiInput
}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::NumberParseError(value.kind().clone())
    }
}

impl From<ascii::AsAsciiStrError> for Error {
    fn from(_value: ascii::AsAsciiStrError) -> Self {
        Error::ExpectedAsciiInput
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnrecognisedKeyword => todo!(),
            Error::Expected(c) => write!(f, "Expected '{}'", c),
            Error::ExpectedKeyword => todo!(),
            Error::ExpectedStringOrExpression => todo!(),
            Error::UnexpectedOperator => todo!(),
            Error::NumberCouldNotBeParsed => todo!(),
            Error::FactorCouldNotBeParsed => todo!(),
            Error::ExpectedEndOfLine => todo!(),
            Error::ExpectedRelationalOperator => todo!(),
            Error::UnexpectedKeyword => todo!(),
            Error::ExpectedVariableName => todo!(),
            Error::NumberParseError(int_error_kind) => todo!(),
            Error::GosubCannotBeUsedInInteractiveMode => todo!(),
            Error::ReturnOnEmptyStack => todo!(),
            Error::ExecutionReachedEnd => todo!(),
            Error::ExpectedAsciiInput => todo!(),
        }
    }
}