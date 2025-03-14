use std::collections::BTreeMap;

use ascii::{AsciiStr, AsciiString};

use crate::tiny_basic::types;

pub struct ProgramStorage {
    storage: BTreeMap<types::LineIndex, AsciiString>
}

impl ProgramStorage {
    pub fn new() -> Self {
        Self {
            storage: BTreeMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.storage.clear();
    }

    pub fn get_line(&self, line_index: types::LineIndex) -> Option<&AsciiStr> {
        self
            .storage
            .get(&line_index)
            .map(|line| &**line)
    }

    pub fn get_following_line_index(&self, line_index: types::LineIndex) -> Option<types::LineIndex> {
        let pivot_index = line_index;
        let next_line_index = self
            .storage
            .keys()
            .skip_while(|line_index| **line_index != pivot_index)
            .nth(1);

        next_line_index.and_then(|next_line_index| {
            Some(*next_line_index)
        })
    }

    pub fn erase_line(&mut self, line_index: types::LineIndex) {
        self.storage.remove(&line_index);
    }

    pub fn insert_line(&mut self, line_index: types::LineIndex, line_contents: &AsciiStr) {
        self.storage.insert(line_index, line_contents.to_owned());
    }

    pub fn get_first_line_index(&self) -> Option<types::LineIndex> {
        self.storage
            .first_key_value()
            .and_then(|(first_line_index, _)| Some(*first_line_index))
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, types::LineIndex, AsciiString> {
        self.storage.iter()
    }
}