//!
//! Functions related to iteration.
//!

///
/// Iterate over the prefixes of a slice.
/// For example, for an input slice `[a,b,c]` this function iterates over `[[], [a], [a,b], [a,b,c]]`.
///
pub fn prefixes<T>(slice: &[T]) -> impl DoubleEndedIterator<Item = &[T]> {
    (0..=slice.len()).map(move |len| &slice[..len])
}

#[cfg(test)]
mod test {

    fn prefixes_check<T>(input: &[T], expected: &[&[T]])
    where
        T: Clone + std::fmt::Debug + Eq + PartialEq,
    {
        assert_eq!(
            super::prefixes(input).collect::<Vec<_>>(),
            expected.to_vec(),
        );
    }

    #[test]
    fn prefixes_empty() {
        prefixes_check::<u32>(&[], &[&[]]);
    }

    #[test]
    fn prefixes_nonempty() {
        prefixes_check(&[1, 2, 3], &[&[], &[1], &[1, 2], &[1, 2, 3]]);
    }
}
