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

fn main() -> ExitCode {
    let mut interpreter = tiny_basic::Interpreter::new();
    loop {
        let mut line = String::new();
        print!("tiny_basic> ");
        stdout().flush();
        match stdin().read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    return ExitCode::SUCCESS;
                }
                interpreter.execute(line.trim());
            },
            Err(kind) => {
                eprintln!("{}", kind);
                return ExitCode::FAILURE;
            }
        }
    }
}
