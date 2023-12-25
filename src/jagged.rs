mod helper;
mod iter;
use crate::{
    index::RowIndex,
    traits::{JaggedRemove, JaggedSlice},
    Index2, JaggedIndex,
};
use std::fmt::Debug;

/// A generic container for working with an object, where each element is organized
/// into lines (rows).
///
/// The [`Jagged`] struct wraps a vector of vectors, where the outer vector represents
/// rows and the inner vectors represent the elements within each row.
///
/// # Generic Parameters
///
/// - `T`: The data type of elements stored within the jagged array.
///
/// # Examples
///
/// ```
/// use edtui_jagged::Jagged;
///
/// let data = vec![
///     vec![1, 2, 3],
///     vec![4, 5, 6],
///     vec![7, 8, 9],
///     vec![0],
/// ];
///
/// let lines = Jagged::new(data);
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Jagged<T> {
    pub(crate) data: Vec<Vec<T>>,
}

impl<T> Default for Jagged<T> {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl<T> Jagged<T> {
    /// Instantiates a new [`Jagged`] object.
    ///
    /// # Arguments
    ///
    /// - `data`: Data of the jagged array. Must be convertable into vec of vecs.
    ///
    /// # Examples
    ///
    /// ```
    /// use edtui_jagged::Jagged;
    ///
    /// let data = vec![
    ///     vec![1, 2, 3],
    ///     vec![4, 5, 6],
    ///     vec![7, 8, 9],
    ///     vec![0],
    /// ];
    ///
    /// let lines = Jagged::new(data);
    /// ```
    #[must_use]
    pub fn new<U>(data: U) -> Self
    where
        U: Into<Vec<Vec<T>>>,
    {
        Jagged { data: data.into() }
    }

    /// Clears the jagged array, removing all values.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Appends an element to the back of the array.
    pub fn push<U>(&mut self, slice: U)
    where
        U: JaggedSlice<T>,
    {
        slice.push_into(self);
    }

    /// Inserts an element at `position` within the rows, shifting all
    /// elements after it.
    pub fn insert<I, U>(&mut self, index: I, slice: U)
    where
        I: JaggedIndex<T>,
        U: JaggedSlice<T, Index = I>,
    {
        // let index = index.into();
        slice.insert_into(index, self);
        // if let Some(line) = self.get_mut(RowIndex::new(index.row)) {
        //     line.insert(index.col, element)
        // }
    }

    /// Removes and returns the element at position index within the jagged array.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    pub fn remove<I>(&mut self, index: I) -> I::Output
    where
        I: JaggedRemove<T>,
    {
        index.remove(self)
    }

    /// Moves all the elements of `other` into `self`, leaving `other` empty.
    ///
    /// Use [`Self::merge`] if the arrays should be fused at tail and head instead.
    pub fn append(&mut self, other: &mut Self) {
        self.data.append(&mut other.data);
    }

    /// Moves all the elements of `other` into `self`, leaving `other` empty.
    ///
    /// Similar to [`Self::append`] but fuses the last vector of `self` with the
    /// first vector of `other`.
    pub fn merge(&mut self, other: &mut Self) {
        if other.data.is_empty() {
            return;
        }
        let last_row = self.len().saturating_sub(1);
        self.data[last_row].append(&mut other.data.remove(0));
        self.data.append(&mut other.data);
    }

    /// Truncate lines up to the specified position.
    pub fn truncate<I>(&mut self, index: I)
    where
        I: Into<Index2>,
    {
        let index = index.into();
        if let Some(current_row) = self.get_mut(RowIndex::new(index.row)) {
            current_row.truncate(index.col);
        }
        self.data.truncate(index.row + 1);
    }

    /// Splits a `Jagged` array into two at the given index.
    ///
    /// Returns a newly allocated `Jagged` containing the elements in the range
    /// `[at, end)`. After the call, the original vector will be left containing
    /// the elements `[0, at)`.
    #[must_use]
    pub fn split_off<I>(&mut self, at: I) -> Self
    where
        I: Into<Index2>,
    {
        let at = at.into();
        if at.col == 0 {
            Self::new(self.data.split_off(at.row))
        } else {
            let mut lines = self.data.remove(at.row);
            let rest = lines.split_off(at.col);

            self.data.insert(at.row, lines);
            self.data.insert(at.row + 1, rest);

            Self::new(self.data.split_off(at.row + 1))
        }
    }

    /// Returns `true` if the object contains no elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get the number of rows.
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Get the number of columns of a given row.
    ///
    /// # Panics
    ///
    /// Panics if `row` is out of bounds.
    #[must_use]
    pub fn len_col(&self, row: usize) -> usize {
        self.data[row].len()
    }

    // /// Find the first index.
    // /// Returns `Some(Index2)` if the matrix is not empty, otherwise `None`.
    // #[must_use]
    // pub fn first_index(&self) -> Option<Index2> {
    //     self.data
    //         .get(0)
    //         .and_then(|row| row.get(0))
    //         .map(|_| Index2::new(0, 0))
    // }

    /// Get a reference to the element at a specific position.
    /// Returns `Some(& T)` if the position is valid, otherwise `None`.
    #[must_use]
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: JaggedIndex<T>,
    {
        index.get(self)
    }

    /// Get a mutable reference to the element at a specific position.
    /// Returns `Some(&mut T)` if the position is valid, otherwise `None`.
    #[must_use]
    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: JaggedIndex<T>,
    {
        index.get_mut(self)
    }

    /// Get the next value and index based on the current position.
    /// Returns `Some((Some(&T), Index2))` if a next position exists, otherwise `None`.
    /// Returns the next value as `None` indicating an empty row.
    #[must_use]
    pub fn next<I>(&self, index: I) -> Option<(Option<&T>, Index2)>
    where
        I: Into<Index2>,
    {
        let index = index.into();
        match (self.is_last_row(index), self.is_last_col(index)) {
            (true, true) => None,
            (false, true) => {
                let p = Index2::new(index.row + 1, 0);
                self.get(p).map_or(Some((None, p)), |v| Some((Some(v), p)))
            }
            _ => {
                let p = Index2::new(index.row, index.col + 1);
                self.get(p).map(|v| (Some(v), p))
            }
        }
    }

    /// Find the next position based on the current position.
    /// Returns `Some((Some(&T), Index2))` if a next position exists, otherwise `None`.
    /// Returns the next value as `None` indicating an empty row.
    #[must_use]
    pub fn next_mut<I>(&mut self, index: I) -> Option<(Option<&mut T>, Index2)>
    where
        I: Into<Index2>,
    {
        let index = index.into();
        match (self.is_last_row(index), self.is_last_col(index)) {
            (true, true) => None,
            (false, true) => {
                let p = Index2::new(index.row + 1, 0);
                self.get_mut(p)
                    .map_or(Some((None, p)), |v| Some((Some(v), p)))
            }
            _ => {
                let p = Index2::new(index.row, index.col + 1);
                self.get_mut(p).map(|v| (Some(v), p))
            }
        }
    }

    /// Find the previous position based on the current position.
    /// Returns `Some((Some(&T), Index2))` if a next position exists, otherwise `None`.
    /// Returns the next value as `None` indicating an empty row.
    #[must_use]
    pub fn prev<I>(&self, index: I) -> Option<(Option<&T>, Index2)>
    where
        I: Into<Index2>,
    {
        let index = index.into();
        match (self.is_first_row(index), self.is_first_col(index)) {
            (true, true) => None,
            (false, true) => {
                let row = index.row - 1;
                let index = Index2::new(row, self.len_col(row).saturating_sub(1));
                self.get(index)
                    .map_or(Some((None, index)), |val| Some((Some(val), index)))
            }
            _ => {
                let index = Index2::new(index.row, index.col - 1);
                self.get(index).map(|v| (Some(v), index))
            }
        }
    }

    /// Find the previous position based on the current position.
    /// Returns `Some((&mut T, Index2))` if a previous position exists, otherwise `None`.
    #[must_use]
    pub fn prev_mut<I>(&mut self, index: I) -> Option<(&mut T, Index2)>
    where
        I: Into<Index2>,
    {
        let index = index.into();
        match (self.is_first_row(index), self.is_first_col(index)) {
            (true, true) => None,
            (false, true) => {
                let row = index.row - 1;
                let index = Index2::new(row, self.len_col(row).saturating_sub(1));
                self.get_mut(index).map(|val| (val, index))
            }
            _ => {
                let p = Index2::new(index.row, index.col - 1);
                self.get_mut(p).map(|v| (v, p))
            }
        }
    }

    /// Find the next position that satisfies a given predicate.
    /// Returns `Some((Option<&T>, Index2))` if a position is found that satisfies the
    /// predicate, otherwise `None`.
    #[must_use]
    pub fn next_predicate<F, I>(&self, index: I, f: F) -> Option<(Option<&T>, Index2)>
    where
        F: Fn(Option<&T>) -> bool,
        I: Into<Index2>,
    {
        let mut index = index.into();
        while let Some((val, pos)) = self.next(index) {
            if f(val) {
                return Some((val, pos));
            }
            index = pos;
        }
        None
    }

    /// Find the next position that satisfies a given predicate.
    /// Returns `Some((Option<&mut T>, Index2))` if a position is found that satisfies the
    /// predicate, otherwise `None`.
    #[must_use]
    pub fn next_predicate_mut<F, I>(&mut self, index: I, f: F) -> Option<(Option<&mut T>, Index2)>
    where
        F: Fn(Option<&T>) -> bool,
        I: Into<Index2>,
    {
        let mut index = index.into();
        while let Some((val, pos)) = self.next(index) {
            if f(val) {
                return Some((self.get_mut(pos), pos));
            }
            index = pos;
        }
        None
    }

    /// Find the previous position that satisfies a given predicate.
    /// Returns `Some((Option<&T>, Index2))` if a satisfying position is found, otherwise `None`.
    #[must_use]
    pub fn prev_predicate<F, I>(&self, index: I, f: F) -> Option<(Option<&T>, Index2)>
    where
        F: Fn(Option<&T>) -> bool,
        I: Into<Index2>,
    {
        let mut index = index.into();
        while let Some((val, next)) = self.prev(index) {
            if f(val) {
                return Some((val, next));
            }
            index = next;
        }
        None
    }

    /// Find the previous position that satisfies a given predicate.
    /// Returns `Some((Option<&mut T>, Index2))` if a satisfying position is found, otherwise `None`.
    #[must_use]
    pub fn prev_predicate_mut<F, I>(&mut self, index: I, f: F) -> Option<(Option<&mut T>, Index2)>
    where
        F: Fn(Option<&T>) -> bool,
        I: Into<Index2>,
    {
        let mut index = index.into();
        while let Some((val, pos)) = self.prev(index) {
            if f(val) {
                return Some((self.get_mut(pos), pos));
            }
            index = pos;
        }
        None
    }
}

impl<T: Clone> Jagged<T> {
    // Flattens the jagged array into a single vector with optional line breaks.
    ///
    /// Returns a flattened `Vec<T>` where each row from the original structure is
    /// concatenated into a single vector. If provided, the `line_break` parameter
    /// is inserted between rows.
    pub fn flatten(&self, line_break: &Option<T>) -> Vec<T> {
        let mut flattened = Vec::new();

        for (i, row) in self.data.iter().enumerate() {
            flattened.extend_from_slice(row);
            if i < self.data.len() - 1 {
                if let Some(item) = line_break.clone() {
                    flattened.push(item);
                }
            }
        }

        flattened
    }
}

impl From<&str> for Jagged<char> {
    /// Instantiate a [`Jagged<char>`] from a string. Iterates over the lines
    /// of the string, i.e. a multiline string will be parsed to multiple
    /// inner vectors.
    fn from(value: &str) -> Self {
        let mut data: Vec<Vec<char>> = Vec::new();
        for line in value.lines() {
            data.push(line.chars().collect());
        }
        if let Some(last) = value.chars().last() {
            if last == '\n' {
                data.push(Vec::new());
            }
        }
        Self { data }
    }
}

impl From<Jagged<char>> for String {
    /// Construct a string from a [`Jagged<char>`].
    fn from(value: Jagged<char>) -> String {
        value.flatten(&Some('\n')).into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::RowSlice;

    use super::*;

    #[test]
    fn test_push() {
        let mut a = Jagged::new(vec![vec![1, 2, 3]]);
        a.push(4);

        assert_eq!(a, Jagged::new(vec![vec![1, 2, 3, 4]]));
    }

    #[test]
    fn test_push_row() {
        let mut a = Jagged::new(vec![vec![1, 2, 3]]);
        a.push(RowSlice::from(vec![4]));

        assert_eq!(a, Jagged::new(vec![vec![1, 2, 3], vec![4]]));
    }

    #[test]
    fn test_push_vec() {
        let mut a = Jagged::new(vec![vec![1, 2, 3]]);
        a.push(vec![4]);

        assert_eq!(a, Jagged::new(vec![vec![1, 2, 3], vec![4]]));
    }

    #[test]
    fn test_append() {
        let mut a = Jagged::new(vec![vec![1, 2, 3]]);
        let mut b = Jagged::new(vec![vec![4, 5, 6]]);
        a.append(&mut b);

        assert_eq!(a, Jagged::new(vec![vec![1, 2, 3], vec![4, 5, 6]]));
    }

    #[test]
    fn test_merge() {
        let mut a = Jagged::new(vec![vec![1, 2]]);
        let mut b = Jagged::new(vec![vec![3], vec![4, 5, 6]]);
        a.merge(&mut b);

        assert_eq!(a, Jagged::new(vec![vec![1, 2, 3], vec![4, 5, 6]]));
    }

    #[test]
    fn test_flatten() {
        // given
        let a = Jagged::new(vec![vec![1], vec![], vec![2]]);
        // when
        let flattened = a.flatten(&Some(0));
        // then
        assert_eq!(flattened, vec![1, 0, 0, 2]);
    }

    #[test]
    fn test_iter() {
        let lines = Jagged::from(
            "Hello\n\
            World",
        );
        let mut iter = lines.iter_row();

        assert_eq!(iter.next(), Some(&"Hello".chars().collect()));
        assert_eq!(iter.next(), Some(&"World".chars().collect()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_split_off() {
        let data: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let mut a = Jagged::new(data);

        let b = a.split_off(Index2::new(1, 1));
        assert_eq!(a, Jagged::new(vec![vec![1, 2, 3], vec![4]]));
        assert_eq!(b, Jagged::new(vec![vec![5, 6], vec![7, 8, 9]]));
    }

    #[test]
    fn test_next() {
        let data: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![], vec![7, 8, 9]];
        let lines = Jagged::new(data);

        assert_eq!(
            lines.next(Index2::new(0, 0)),
            Some((Some(&2), Index2::new(0, 1)))
        );
        assert_eq!(
            lines.next(Index2::new(0, 2)),
            Some((None, Index2::new(1, 0)))
        );
        assert_eq!(lines.next(Index2::new(2, 2)), None,);
    }

    #[test]
    fn test_prev() {
        let data: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let lines = Jagged::new(data);

        assert_eq!(
            lines.prev(Index2::new(1, 1)),
            Some((Some(&4), Index2::new(1, 0)))
        );
        assert_eq!(
            lines.prev(Index2::new(1, 0)),
            Some((Some(&3), Index2::new(0, 2)))
        );
        assert_eq!(lines.prev(Index2::new(0, 0)), None,);
    }

    #[test]
    fn test_next_predicate() {
        let data: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![], vec![4, 5, 6], vec![7, 8, 9]];
        let lines = Jagged::new(data);

        assert_eq!(
            lines.next_predicate(Index2::new(0, 2), |val| val == Some(&5)),
            Some((Some(&5), Index2::new(2, 1)))
        );
        assert_eq!(
            lines.next_predicate(Index2::new(0, 0), |val| val == Some(&99)),
            None,
        );
    }

    #[test]
    fn test_prev_predicate() {
        let data: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let lines = Jagged::new(data);

        assert_eq!(
            lines.prev_predicate(Index2::new(2, 2), |val| val == Some(&5)),
            Some((Some(&5), Index2::new(1, 1)))
        );
        assert_eq!(
            lines.prev_predicate(Index2::new(2, 2), |val| val == Some(&99)),
            None,
        );
    }

    #[test]
    fn test_from_str() {
        let lines = Jagged::from("H\n");

        assert_eq!(lines, Jagged::new(vec![vec!['H'], vec![]]));
    }
}
