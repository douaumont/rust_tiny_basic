use std::collections::BTreeMap;
use std::borrow::Cow;

use ascii::{AsciiStr, AsciiString};

use crate::tiny_basic::types;

type Line<'a> = Cow<'a, AsciiStr>;

pub struct ProgramStorage<'a> {
    storage: BTreeMap<types::Number, Line<'a>>
}

impl<'a> ProgramStorage<'a> {
    pub fn new() -> Self {
        Self {
            storage: BTreeMap::new()
        }
    }

    pub fn clear(&mut self) {
        self.storage.clear();
    }

    pub fn get_line(&self, line_index: types::Number) -> Option<Line<'a>> {
        self.storage.get(&line_index).cloned()
    }

    pub fn get_following_line_index(&self, line_index: types::Number) -> Option<types::Number> {
        let pivot_index = line_index;
        let next_line_index = self.storage
        .keys()
        .skip_while(|line_index| **line_index != pivot_index)
        .nth(1);

        next_line_index.and_then(|next_line_index| {
            Some(*next_line_index)
        })
    }

    pub fn erase_line(&mut self, line_index: types::Number) {
        self.storage.remove(&line_index);
    }

    pub fn insert_line(&mut self, line_index: types::Number, line_contents: AsciiString) {
        self.storage.insert(line_index, Cow::Owned(line_contents));
    }

    pub fn get_first_line_index(&self) -> Option<types::Number> {
        self.storage
            .first_key_value()
            .and_then(|(first_line_index, _)| Some(*first_line_index))
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, types::Number, Line<'a>> {
        self.storage.iter()
    }
}