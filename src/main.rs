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

fn print_program_info() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("Copyright (C) 2025 {}", env!("CARGO_PKG_AUTHORS"));
    println!("License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>");
    println!("This is free software: you are free to change and redistribute it.");
    println!("There is NO WARRANTY, to the extent permitted by law.");
    println!();
}

/// Read, Evaluate, Print, Loop
/// of the interpreter
fn repl() -> std::io::Result<()> {
    let mut interpreter = tiny_basic::interpreter::Interpreter::new();
    println!("READY");
    loop {
        let mut line = String::new();
        let bytes_read = stdin().read_line(&mut line)?;
        if bytes_read == 0 {
            return Ok(());
        }

        let line = match ascii::AsciiStr::from_ascii(&line) {
            Ok(line) => line,
            Err(_) => {
                eprintln!("Current implementation requires all input to be ASCII-only");
                continue;
            },
        };
        
        match interpreter.execute(line) {
            Ok(_) => println!("OK"),
            Err(error) => {
                eprintln!("{}", error);
            },
        }
    }
}

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
