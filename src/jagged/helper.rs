use crate::{Index2, Jagged};

impl<T> Jagged<T> {
    /// Check if a given position is the first row.
    #[must_use]
    pub fn is_first_row<I>(&self, index: I) -> bool
    where
        I: Into<Index2>,
    {
        let index = index.into();
        index.row == 0 && !self.data.is_empty()
    }

    /// Check if a given position is the last row.
    #[must_use]
    pub fn is_last_row<I>(&self, index: I) -> bool
    where
        I: Into<Index2>,
    {
        let index = index.into();
        index.row == self.data.len() - 1 && !self.data.is_empty()
    }

    /// Check if a given position is the first column.
    #[must_use]
    pub fn is_first_col<I>(&self, index: I) -> bool
    where
        I: Into<Index2>,
    {
        let index = index.into();
        match self.data.get(index.row) {
            Some(_) => index.col == 0,
            None => false,
        }
    }

    /// Check if a given position is the last column.
    #[must_use]
    pub fn is_last_col<I>(&self, index: I) -> bool
    where
        I: Into<Index2>,
    {
        let index = index.into();
        match self.data.get(index.row) {
            Some(row) => index.col >= row.len().saturating_sub(1),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_first_position() {
    //     let data: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    //     let lines = Jagged2::new(data);
    //
    //     assert_eq!(lines.first_index(), Some(Index2::new(0, 0)));
    // }

    // #[test]
    // fn test_last_position() {
    //     let data: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    //     let lines = Jagged2::new(data);
    //
    //     assert_eq!(lines.last_index(), Some(Index2::new(2, 2)));
    // }

    #[test]
    fn test_is_first_col() {
        let data: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let lines = Jagged::new(data);

        assert!(lines.is_first_col(Index2::new(0, 0)));
        assert!(lines.is_first_col(Index2::new(1, 0)));
        assert!(!lines.is_first_col(Index2::new(0, 1)));
    }

    #[test]
    fn test_is_last_col() {
        let data: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let lines = Jagged::new(data);

        assert!(!lines.is_last_col(Index2::new(0, 0)));
        assert!(lines.is_last_col(Index2::new(2, 2)));
    }

    #[test]
    fn test_is_first_row() {
        let data: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let lines = Jagged::new(data);

        assert!(lines.is_first_row(Index2::new(0, 0)));
        assert!(!lines.is_first_row(Index2::new(1, 0)));
    }

    #[test]
    fn test_is_last_row() {
        let data: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let lines = Jagged::new(data);

        assert!(!lines.is_last_row(Index2::new(0, 0)));
        assert!(lines.is_last_row(Index2::new(2, 0)));
    }

    // #[test]
    // fn test_get_last() {
    //     let data: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    //     let lines = Jagged2::new(data);
    //
    //     assert_eq!(lines.get_last(1), Some((&6, Index2::new(1, 2))));
    // }
}
