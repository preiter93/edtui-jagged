use crate::{index::RowIndex, Index2, Jagged};

/// A helper trait used for indexing operations of a jagged array.
pub trait JaggedIndex<T> {
    type Output: Sized;

    fn get(self, array: &Jagged<T>) -> Option<&Self::Output>;
    fn get_mut(self, array: &mut Jagged<T>) -> Option<&mut Self::Output>;
    fn remove(self, array: &mut Jagged<T>) -> Self::Output;
}

impl<T> JaggedIndex<T> for Index2 {
    type Output = T;

    fn get(self, array: &Jagged<T>) -> Option<&Self::Output> {
        array.data.get(self.row).and_then(|line| line.get(self.col))
    }

    fn get_mut(self, array: &mut Jagged<T>) -> Option<&mut Self::Output> {
        array
            .data
            .get_mut(self.row)
            .and_then(|line| line.get_mut(self.col))
    }

    fn remove(self, array: &mut Jagged<T>) -> Self::Output {
        array.data[self.row].remove(self.col)
    }
}

impl<T> JaggedIndex<T> for RowIndex {
    type Output = Vec<T>;

    fn get(self, array: &Jagged<T>) -> Option<&Self::Output> {
        array.data.get(self.0)
    }

    fn get_mut(self, array: &mut Jagged<T>) -> Option<&mut Self::Output> {
        array.data.get_mut(self.0)
    }

    fn remove(self, array: &mut Jagged<T>) -> Self::Output {
        array.data.remove(self.0)
    }

    // fn row(self) -> usize {
    //     self.0
    // }
    //
    // fn col(self) -> usize {
    //     panic!("tried to call col on a RowIndex")
    // }
}

/// A helper trait used for data operations of a jagged array.
pub trait JaggedSlice<T> {
    type Index: JaggedIndex<T>;
    fn push_into(self, array: &mut Jagged<T>);
    fn insert_into(self, index: Self::Index, array: &mut Jagged<T>);
}

/// An index representing a specific row in a jagged array.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct RowSlice<T> {
    data: Vec<T>,
}

impl<T> From<Vec<T>> for RowSlice<T> {
    fn from(val: Vec<T>) -> Self {
        Self::new(val)
    }
}

impl<T> RowSlice<T> {
    /// Instantiates a new `RowSlice` from a vector.
    #[must_use]
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }
}

impl<T> JaggedSlice<T> for T {
    type Index = Index2;

    fn push_into(self, array: &mut Jagged<T>) {
        if let Some(row) = array.get_mut(RowIndex::new(array.len().saturating_sub(1))) {
            row.push(self);
        }
    }

    fn insert_into(self, index: Self::Index, array: &mut Jagged<T>) {
        if let Some(line) = array.get_mut(RowIndex::new(index.row)) {
            line.insert(index.col, self);
        }
    }
}

impl<T> JaggedSlice<T> for RowSlice<T>
where
    T: std::fmt::Debug + Clone,
{
    type Index = RowIndex;

    fn push_into(self, array: &mut Jagged<T>) {
        array.data.push(self.data);
    }

    fn insert_into(self, index: Self::Index, array: &mut Jagged<T>) {
        array.data.insert(index.0, self.data);
    }
}

impl<T> JaggedSlice<T> for Vec<T> {
    type Index = RowIndex;

    fn push_into(self, array: &mut Jagged<T>) {
        array.data.push(self);
    }

    fn insert_into(self, index: Self::Index, array: &mut Jagged<T>) {
        array.data.insert(index.0, self);
    }
}
