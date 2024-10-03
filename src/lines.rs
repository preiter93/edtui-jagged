use crate::{Index2, Jagged};

type Lines = Jagged<char>;

impl Lines {
    /// Finds the index of the matching (closing or opening) bracket from a given starting point.
    #[must_use]
    pub fn find_matching_bracket(&self, index: Index2) -> Option<Index2> {
        let &opening_bracket = self.get(index)?;

        let (closing_bracket, reverse) = match opening_bracket {
            '{' => ('}', false),
            '}' => ('{', true),
            '(' => (')', false),
            ')' => ('(', true),
            '[' => (']', false),
            ']' => ('[', true),
            _ => return None,
        };

        let mut counter = 0;

        let iter: Box<dyn Iterator<Item = (Option<&char>, Index2)>> = if reverse {
            Box::new(self.iter().from(index).rev().skip(1))
        } else {
            Box::new(self.iter().from(index).skip(1))
        };

        for (value, index) in iter {
            let Some(&value) = value else { continue };

            if value == opening_bracket {
                counter += 1;
            }

            if value == closing_bracket {
                if counter == 0 {
                    return Some(index);
                }
                counter -= 1;
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_closing_bracket() {
        let cursor = Index2::new(0, 0);
        let lines = Jagged::from("{ab\n{{}}c}d");

        let closing_bracket = lines.find_matching_bracket(cursor);
        assert_eq!(closing_bracket, Some(Index2::new(1, 5)));

        let cursor = Index2::new(1, 5);
        let closing_bracket = lines.find_matching_bracket(cursor);
        assert_eq!(closing_bracket, Some(Index2::new(0, 0)));
    }
}
