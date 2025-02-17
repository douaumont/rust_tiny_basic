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

use std::{io::{stdin, stdout, Write}, process::ExitCode};

const PROMPT: &'static str = "tiny_basic> ";

/// Read, Evaluate, Print, Loop
/// of the interpreter
fn repl() -> std::io::Result<()> {
    let mut interpreter = tiny_basic::interpreter::Interpreter::new();
    loop {
        let mut line = String::new();
        
        print!("{}", PROMPT);
        stdout().flush()?;

        let bytes_read = stdin().read_line(&mut line)?;
        if bytes_read == 0 {
            return Ok(());
        }

        let line = match ascii::AsciiStr::from_ascii(line.as_bytes()) {
            Ok(line) => line,
            Err(_) => {
                eprintln!("Current implementation requires all input to be ASCII-only");
                continue;
            },
        };
        
        match interpreter.execute(line) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("{:?}", error);
            },
        }
    }
}

fn main() -> ExitCode {
    match repl() {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{}", error);
            ExitCode::FAILURE
        },
    }
}
