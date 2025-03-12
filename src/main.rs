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

mod tiny_basic;

use tiny_basic::char_stream::AsciiCharStream;

use crate::tiny_basic::*;
use std::{io::{stdin, stdout, Write}, process::ExitCode};

fn main() -> ExitCode {
    print_program_info();
    match repl() {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{}", error);
            ExitCode::FAILURE
        },
    }
}


fn print_program_info() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("Copyright (C) 2025 {}", env!("CARGO_PKG_AUTHORS"));
    println!("License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>");
    println!("This is free software: you are free to change and redistribute it.");
    println!("There is NO WARRANTY, to the extent permitted by law.");
    println!();
}

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
                continue;
            }
        }
    };
}

/// Read, Evaluate, Print, Loop
/// of the interpreter
fn repl() -> std::io::Result<()> {
    let mut interpreter = interpreter::Interpreter::new();
    let mut program_storage = program_storage::ProgramStorage::new();

    println!("READY");

    loop {
        let line = match read_line()? {
            Some(line) => line,
            None => return Ok(()),
        };

        let line = unwrap_or_continue!(ascii::AsciiStr::from_ascii(&line));
        let line = unwrap_or_continue!(code_line::Line::try_from(line.trim()));

        match line.index {
            Some(i) => {
                if line.statement.is_empty() {
                    program_storage.erase_line(i);
                } else {
                    program_storage.insert_line(i, line.statement);
                }
            },
            None => {
                if let Some(command) = AsciiCharStream::from_ascii_str(line.statement).consume_command() {
                    match command {
                        char_stream::Command::Run => show_outcome!(interpreter.run(&program_storage)),
                        char_stream::Command::List => {
                            for (i, line) in program_storage.iter() {
                                println!("{} {}", i, line);
                            }
                        },
                        char_stream::Command::Clear => program_storage.clear(),
                    }
                } else {
                    show_outcome!(interpreter.execute(&mut AsciiCharStream::from_ascii_str(line.statement)));
                }
            }
        }
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