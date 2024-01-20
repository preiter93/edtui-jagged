#![allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
use crate::{Index2, Jagged};
use core::slice::Iter;

pub struct JaggedIterator<'a, T> {
    pub(super) data: &'a Jagged<T>,
    pub(super) row: isize,
    pub(super) col: usize,
    pub(super) end: Option<Index2>,
    pub(super) stop: bool,
}

impl<'a, T> JaggedIterator<'a, T> {
    /// Instantiates a new [`LinesIterator`] that starts from a given position.
    #[must_use]
    pub fn new(data: &'a Jagged<T>) -> Self {
        Self {
            data,
            row: 0,
            col: 0,
            end: None,
            stop: false,
        }
    }
    /// A [`LinesIterator`] that starts from a given position.
    #[must_use]
    pub fn from(self, index: Index2) -> Self {
        Self {
            data: self.data,
            row: index.row as isize,
            col: index.col,
            end: None,
            stop: self.stop,
        }
    }
    /// A [`LinesIterator`] that end at a given position.
    #[must_use]
    pub fn to(self, index: Index2) -> Self {
        Self {
            data: self.data,
            row: self.row,
            col: self.col,
            end: Some(index),
            stop: self.stop,
        }
    }
}

impl<'a, T> Iterator for JaggedIterator<'a, T> {
    type Item = (Option<&'a T>, Index2);

    fn next(&mut self) -> Option<Self::Item> {
        let current_index = Index2::new(self.row as usize, self.col);
        if self.stop || current_index.out_of_bounds(self.data) {
            return None;
        }
        if let Some((_, index)) = self.data.next(current_index) {
            self.row = index.row as isize;
            self.col = index.col;
        } else {
            self.stop = true;
        }
        if Some(current_index) == self.end {
            self.stop = true;
        }
        Some((self.data.get(current_index), current_index))
    }
}

impl<'a, T> DoubleEndedIterator for JaggedIterator<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.stop || self.row < 0 {
            return None;
        }
        let current_index = Index2::new(self.row as usize, self.col);
        if let Some((_, index)) = self.data.prev(current_index) {
            self.row = index.row as isize;
            self.col = index.col;
        } else {
            self.stop = true;
        }
        if Some(current_index) == self.end {
            self.stop = true;
        }
        Some((self.data.get(current_index), current_index))
    }
}

impl<T> Jagged<T> {
    /// Returns an iterator that yields the element of a jagged
    /// array along with its current index.
    #[must_use]
    pub fn iter(&self) -> JaggedIterator<'_, T> {
        JaggedIterator::new(self)
    }

    /// Returns an iterator over the rows of the jagged array.
    pub fn iter_row(&self) -> Iter<'_, Vec<T>> {
        self.data.iter()
    }
}

impl<'a, T: Clone> FromIterator<<JaggedIterator<'a, T> as Iterator>::Item> for Jagged<T> {
    /// Collects the elements from the iterator into a new Jagged array.
    ///
    /// Returns a new Jagged array and collects the indices of each element.
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = <JaggedIterator<'a, T> as Iterator>::Item>,
    {
        let mut result = Jagged::default();
        let mut current_row = 0;

        for (value, index) in iter {
            if result.is_empty() || index.row != current_row {
                current_row = index.row;
                result.push(Vec::new());
            }

            if let Some(value) = value {
                result.push(value.clone());
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_obj_long() -> Jagged<char> {
        Jagged::from("h world!\n\n123.")
    }

    #[test]
    fn test_iter() {
        // given
        let jagged = test_obj_long();
        // when
        let got: Jagged<char> = jagged.iter().collect();
        //then
        assert_eq!(got, jagged)
    }

    #[test]
    fn test_iter_from() {
        // given
        let jagged = test_obj_long();
        // when
        let start = Index2::new(0, 3);
        let got: Jagged<char> = jagged.iter().from(start).collect();
        //then
        let exp = Jagged::from("orld!\n\n123.");
        assert_eq!(got, exp)
    }

    #[test]
    fn test_iter_to() {
        // given
        let jagged = test_obj_long();
        // when
        let start = Index2::new(0, 3);
        let stop = Index2::new(2, 1);
        let got: Jagged<char> = jagged.iter().from(start).to(stop).collect();
        //then
        let exp = Jagged::from("orld!\n\n12");
        assert_eq!(got, exp)
    }

    #[test]
    fn test_iter_rev() {
        // given
        let jagged = test_obj_long();
        // when
        let start = Index2::new(2, 4);
        let got: Jagged<char> = jagged.iter().from(start).rev().collect();
        //then
        let exp = Jagged::from(".321\n\n!dlrow h");
        assert_eq!(got, exp)
    }
}
