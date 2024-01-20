use std::{collections::VecDeque, fmt::Debug};

use crate::{Index2, Jagged};

/// An iterator over the disjoint matches of a pattern within this array.
pub struct MatchIndices<'a, 'b, T> {
    /// The array to be search through.
    data: &'a Jagged<T>,

    /// The pattern that is to be seached for
    pattern: &'b [T],

    /// The index of the start position.
    start_index: Option<Index2>,
}

impl<'a, 'b, T: PartialEq> MatchIndices<'a, 'b, T> {
    /// Instantiates a new [`MatchIndices`] that starts from a given position.
    #[must_use]
    pub(super) fn new(data: &'a Jagged<T>, pattern: &'b [T]) -> Self {
        Self {
            data,
            pattern,
            start_index: Some(Index2::default()),
        }
    }

    fn match_found(&self, other: &VecDeque<&T>) -> bool {
        if self.pattern.len() != other.len() {
            return false;
        }
        for (a, b) in self.pattern.iter().zip(other.iter()) {
            if &a != b {
                return false;
            }
        }
        true
    }
}

impl<'a, 'b, T: PartialEq + Debug> Iterator for MatchIndices<'a, 'b, T> {
    type Item = (&'b [T], Index2);

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() | self.pattern.is_empty() {
            return None;
        }
        let Some(start_index) = self.start_index else {
            return None;
        };
        let mut stack = VecDeque::<&T>::new();
        let pattern_len = self.pattern.len();
        for (value, index) in self.data.iter().from(start_index) {
            let Some(value) = value else { continue };
            if index.col == 0 {
                stack.clear();
            }
            if stack.len() >= pattern_len {
                stack.pop_front();
            }
            stack.push_back(value);
            if self.match_found(&stack) {
                self.start_index = self.data.next(index).map(|(_, index)| index);
                let mut index = index;
                index.col -= pattern_len.saturating_sub(1);
                return Some((self.pattern, index));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_obj_long() -> Jagged<char> {
        Jagged::from("aabcaabc\n\naabc.")
    }

    #[test]
    fn test_match_indices() {
        let jagged = test_obj_long();
        let pattern: Vec<char> = vec!['a', 'b', 'c'];

        let mut match_indices = jagged.match_indices(&pattern);
        let index = match_indices.next().map(|(_, index)| index);
        assert_eq!(index, Some(Index2::new(0, 1)));

        let index = match_indices.next().map(|(_, index)| index);
        assert_eq!(index, Some(Index2::new(0, 5)));

        let index = match_indices.next().map(|(_, index)| index);
        assert_eq!(index, Some(Index2::new(2, 1)));

        let index = match_indices.next().map(|(_, index)| index);
        assert_eq!(index, None);
    }
}
