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

use crate::tiny_basic::{
    interpreter::Interpreter, 
    code_line::Line, 
    char_stream,
    program_storage::ProgramStorage,
    types,
};

use std::io::stdin;

macro_rules! unwrap_or_continue {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(error) => {
                eprintln!("{}", error);
                continue;
            },
        }
    };
}

macro_rules! show_outcome {
    ($result:expr) => {
        match $result {
            Ok(_) => println!("OK"),
            Err(error) => {
                eprintln!("{}", error);
                return;
            }
        }
    };
}

/// Read, Evaluate, Print, Loop
pub struct Repl {
    interpreter: Interpreter,
    program: ProgramStorage
}

impl Repl {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
            program: ProgramStorage::new()
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        println!("READY");
        loop {
            let line = match Self::read_line()? {
                Some(line) => line,
                None => return Ok(()),
            };
    
            let line = unwrap_or_continue!(ascii::AsciiStr::from_ascii(&line));
            let line = unwrap_or_continue!(Line::try_from(line.trim()));
    
            match line.index {
                Some(i) => {
                    self.insert_or_erase_line(i, line.statement);
                },
                None => {
                    self.process_line(line.statement);
                }
            }
        }
    }

    fn insert_or_erase_line(&mut self, index: types::Number, contents: &AsciiStr) {
        if contents.is_empty() {
            self.program.erase_line(index);
        } else {
            self.program.insert_line(index, contents);
        }
    }

    fn process_line(&mut self, line: &AsciiStr) {
        let line = char_stream::AsciiCharStream::from_ascii_str(line);
        if let Some(command) = line.clone().consume_command() {
            match command {
                char_stream::Command::Run => show_outcome!(self.interpreter.run(&self.program)),
                char_stream::Command::List => {
                    for (i, line) in self.program.iter() {
                        println!("{} {}", i, line);
                    }
                },
                char_stream::Command::Clear => self.program.clear(),
            }
        } else if let Some(_) = line.clone().consume_statement() {
            show_outcome!(self.interpreter.execute(&mut line.clone()));
        }
    }

    fn read_line() -> std::io::Result<Option<String>> {
        let mut line = String::new();
        let bytes_read = stdin().read_line(&mut line)?;
        if bytes_read == 0 {
            Ok(None)
        } else {
            Ok(Some(line))
        }
    }
}