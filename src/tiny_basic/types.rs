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

use crate::tiny_basic;

pub type Number = i16;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct LineIndex(Number);

impl LineIndex {
    pub const MIN: Number = 1;
    pub const MAX: Number = Number::MAX;
}

impl TryFrom<Number> for LineIndex {
    type Error = tiny_basic::error::ErrorKind;

    fn try_from(value: Number) -> Result<Self, Self::Error> {
        (Self::MIN..=Self::MAX)
            .contains(&value)
            .then_some(LineIndex(value))
            .ok_or(tiny_basic::error::ErrorKind::InvalidLineIndex)
    }
}

impl Into<Number> for LineIndex {
    fn into(self) -> Number {
        let LineIndex(i) = self;
        i 
    }
}

impl std::fmt::Display for LineIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<Number>::into(*self))
    }
}