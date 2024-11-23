/// Re-implementation of the old `char_at`
///
/// See [this GitHub file](https://github.com/rust-lang/regex/blob/1a069b9232c607b34c4937122361aa075ef573fa/regex-syntax/src/ast/parse.rs#L483) for more info
pub trait CharIndex {
    /// Indexes a string at a byte offset, extracting a complete char.
    /// Returns None if the index is out-of-bounds or not at a char boundary.
    /// This is **not** the i-th char, this is the char starting at byte i
    fn try_char_at(&self, index: usize) -> Option<char>;

    /// Indexes a string at a byte offset, extracting a complete char.
    /// Panics if the index is out-of-bounds or not at a char boundary.
    /// This is **not** the i-th char, this is the char starting at byte i
    fn char_at(&self, index: usize) -> char {
        self.try_char_at(index)
            .unwrap_or_else(|| panic!("expect a char at offset {}", index))
    }
}

impl CharIndex for &str {
    fn try_char_at(&self, i: usize) -> Option<char> {
        if self.is_char_boundary(i) {
            self[i..].chars().next()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn oob_panics() { "foo".char_at(10); }
}
