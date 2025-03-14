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


use ascii::{AsAsciiStr, AsciiChar, AsciiStr};

use crate::tiny_basic::{result, error::{Error, ErrorKind}};

#[derive(Debug, PartialEq, Eq)]
pub enum Keyword {
    Print,
    If,
    Then,
    Run,
    List,
    Clear,
    Goto,
    Let,
    Gosub,
    Return,
    End,
    Input
}

pub enum Statement {
    Let,
    Print,
    End,
    Goto,
    If,
    Gosub,
    Return,
    Input
}

pub enum Command {
    Run,
    List,
    Clear,
}

#[derive(Debug, PartialEq, Eq)]

pub enum RelationalOperator {
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    NotEqual,
    Equal
}

#[derive(Default, Clone, PartialEq, Copy)]
struct StreamState {
    cur: usize
}

impl StreamState {
    fn advance(mut self) -> Self {
        self.cur += 1;
        self
    }
}

#[derive(Clone, Copy)]
pub struct AsciiCharStream<'a> {
    stream: &'a AsciiStr,
    state: StreamState
}

impl<'a> AsciiCharStream<'a> {
    pub fn from_ascii_str(ascii_str: &'a AsciiStr) -> Self {
        Self {
            stream: ascii_str,
            state: StreamState {
                cur: 0
            }
        }
    }

    pub fn get_stream(&self) -> &'a AsciiStr {
        self.stream
    }

    pub fn get_location(&self) -> usize {
        self.state.cur
    }

    pub fn peek(&self) -> Option<AsciiChar> {
        self.stream.get_ascii(self.state.cur)
    }

    pub fn match_char<F>(&mut self, predicate: F) -> Option<AsciiChar>
    where F: Fn(&AsciiChar) -> bool {
        match self.peek() {
            Some(ch) => if predicate(&ch) {
                Some(ch)
            } else {
                None
            },
            None => None,
        }
    }

    pub fn consume_char_if<F>(&mut self, predicate: F) -> Option<AsciiChar>
    where F: Fn(&AsciiChar) -> bool {
        let match_res = self.match_char(predicate);
        if match_res.is_some() {
            self.advance();
        }
        self.trim_start();
        match_res
    }

    pub fn consume_char(&mut self, ch: AsciiChar) -> Option<()> {
        self.consume_char_if(|tested_ch| {
            *tested_ch == ch
        })
        .and(Some(()))
    }

    pub fn consume_number(&mut self) -> Option<&AsciiStr> {
        let mut number_end = self.clone();
        while number_end.match_char(AsciiChar::is_ascii_digit).is_some() {
            number_end.advance();
        }
        if number_end.state == self.state {
            None
        } else {
            let number_str = &self.stream[self.state.cur..number_end.state.cur];
            *self = number_end.clone();
            self.trim_start();
            Some(number_str)
        }
    }

    pub fn consume_keyword(&mut self) -> Option<Keyword> {
        let mut keyword_end = self.clone();
        keyword_end.advance_while(AsciiChar::is_ascii_alphabetic);
        if keyword_end.state == self.state {
            None
        } else {
            let keyword = &self.stream[self.state.cur..keyword_end.state.cur];
            *self = keyword_end.clone();
            self.trim_start();
            match keyword.as_str() {
                "PRINT" => Some(Keyword::Print),
                "IF" => Some(Keyword::If),
                "THEN" => Some(Keyword::Then),
                "RUN" => Some(Keyword::Run),
                "LIST" => Some(Keyword::List),
                "CLEAR" => Some(Keyword::Clear),
                "GOTO" => Some(Keyword::Goto),
                "LET" => Some(Keyword::Let),
                "GOSUB" => Some(Keyword::Gosub),
                "RETURN" => Some(Keyword::Return),
                "END" => Some(Keyword::End),
                "INPUT" => Some(Keyword::Input),
                _ => None
            }
        }
    }

    pub fn consume_statement(&mut self) -> Option<Statement> {
        match self.consume_keyword()? {
            Keyword::Print => Some(Statement::Print),
            Keyword::If => Some(Statement::If),
            Keyword::Then => None,
            Keyword::Run => None,
            Keyword::List => None,
            Keyword::Clear => None,
            Keyword::Goto => Some(Statement::Goto),
            Keyword::Let => Some(Statement::Let),
            Keyword::Gosub => Some(Statement::Gosub),
            Keyword::Return => Some(Statement::Return),
            Keyword::End => Some(Statement::End),
            Keyword::Input => Some(Statement::Input),
        }
    }

    pub fn consume_command(&mut self) -> Option<Command> {
        match self.consume_keyword()? {
            Keyword::Print => None,
            Keyword::If => None,
            Keyword::Then => None,
            Keyword::Run => Some(Command::Run),
            Keyword::List => Some(Command::List),
            Keyword::Clear => Some(Command::Clear),
            Keyword::Goto => None,
            Keyword::Let => None,
            Keyword::Gosub => None,
            Keyword::Return => None,
            Keyword::End => None,
            Keyword::Input => None,
        }
    }

    pub fn consume_string(&mut self) ->  result::Result<'a, Option<&'a AsciiStr>> {
        if self.consume_char(AsciiChar::Quotation).is_none() {
            return Ok(None);
        }

        let mut string_end = self.clone();
        string_end.advance_while(|ch| {
            ch.is_ascii_printable()
            && *ch != '"'
        });

        let string = &self.stream[self.state.cur..string_end.state.cur];
        string_end
            .consume_char(AsciiChar::Quotation)
            .ok_or(Error::from_context(&string_end, ErrorKind::Expected('"'), None))?;
        *self = string_end.clone();
        self.trim_start();
        Ok(Some(string))
    }

    pub fn consume_var(&mut self) -> Option<&AsciiStr> {
        let mut var_end = self.clone();
        var_end.advance_while(AsciiChar::is_ascii_alphabetic);
        if var_end.state == self.state {
            return None;
        }
        var_end.advance_while(|ch| 
            ch.is_ascii_alphabetic()
            || ch.is_ascii_digit()
            || *ch == AsciiChar::UnderScore
            || *ch == AsciiChar::Minus);

        if var_end.state == self.state {
            None
        } else {
            let var_name = &self.stream[self.state.cur..var_end.state.cur];
            *self = var_end.clone();
            self.trim_start();
            Some(var_name)
        }
    }

    pub fn consume_relop(&mut self) -> Option<RelationalOperator> {
        if self.consume_char(AsciiChar::LessThan).is_some() {
            if self.consume_char(AsciiChar::Equal).is_some() {
                Some(RelationalOperator::LessEqual)
            } else if self.consume_char(AsciiChar::GreaterThan).is_some() {
                Some(RelationalOperator::NotEqual)
            } else {
                Some(RelationalOperator::Less)
            }
        } else if self.consume_char(AsciiChar::GreaterThan).is_some() {
            if self.consume_char(AsciiChar::Equal).is_some() {
                Some(RelationalOperator::GreaterEqual)
            } else if self.consume_char(AsciiChar::LessThan).is_some() {
                Some(RelationalOperator::NotEqual)
            } else {
                Some(RelationalOperator::Greater)
            }
        } else if self.consume_char(AsciiChar::Equal).is_some() {
            Some(RelationalOperator::Equal)
        } else {
            None
        }
    }

    pub fn flush(&mut self) -> &'a AsciiStr {
        let remaining = &self.stream[self.state.cur..];
        self.state.cur = self.stream.len();
        remaining
    }

    pub fn is_empty(&self) -> bool {
        self.state.cur >= self.stream.len()
    }

    fn advance_while<F>(&mut self, predicate: F)
    where F: Fn(&AsciiChar) -> bool {
        while self.match_char(&predicate).is_some() {
            self.advance();
        }
    }

    fn trim_start(&mut self) {
        while self.match_char(AsciiChar::is_ascii_whitespace).is_some()  {
            self.advance();
        }
    }

    fn advance(&mut self) {
        self.state = self.state.clone().advance();
    }
}

#[cfg(test)]
mod tests {
    use crate::tiny_basic::char_stream::Keyword;

    use super::AsciiCharStream;

    #[test]
    fn test_consume_number() {
        {
            let mut stream = AsciiCharStream::from_ascii_str(ascii::AsciiStr::from_ascii(b"10123 1232").unwrap());
            assert_eq!(stream.consume_number().unwrap().as_str().parse::<i32>().unwrap(), 10123);
            assert_eq!(stream.consume_number().unwrap().as_str().parse::<i32>().unwrap(), 1232);
            assert!(stream.consume_number().is_none());
        }
    }

    #[test]
    fn test_consume_keyword() {
        {
            let mut stream = AsciiCharStream::from_ascii_str(ascii::AsciiStr::from_ascii(b"PRINT IF 10123 1232").unwrap());
            assert_eq!(stream.consume_keyword().unwrap(), Keyword::Print);
            assert_eq!(stream.consume_keyword().unwrap(), Keyword::If);
            assert!(stream.consume_keyword().is_none());
        }
    }

    #[test]
    fn test_consume_string() {
        {
            let mut stream = AsciiCharStream::from_ascii_str(ascii::AsciiStr::from_ascii(b"PRINT \"Hello world\"").unwrap());
            assert_eq!(stream.consume_keyword().unwrap(), Keyword::Print);
            assert_eq!(stream.consume_string().unwrap().unwrap().as_str(), "Hello world");
            assert!(stream.is_empty());
        }

        {
            let mut stream = AsciiCharStream::from_ascii_str(ascii::AsciiStr::from_ascii(b"PRINT \"\"").unwrap());
            assert_eq!(stream.consume_keyword().unwrap(), Keyword::Print);
            assert_eq!(stream.consume_string().unwrap().unwrap().as_str(), "");
        }
    }

    #[test]
    fn test_consume_var() {
        {
            let mut stream = AsciiCharStream::from_ascii_str(ascii::AsciiStr::from_ascii(b"PRINT A").unwrap());
            assert_eq!(stream.consume_keyword().unwrap(), Keyword::Print);
            assert_eq!(stream.consume_var().unwrap().as_str(), "A");
        }
    }

    #[test]
    fn test_is_empty() {
        {
            let mut stream = AsciiCharStream::from_ascii_str(ascii::AsciiStr::from_ascii(b"PRINT A").unwrap());
            assert_eq!(stream.consume_keyword().unwrap(), Keyword::Print);
            assert_eq!(stream.consume_var().unwrap().as_str(), "A");
            assert!(stream.is_empty());
        }
    }
}