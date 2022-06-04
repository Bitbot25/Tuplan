#![feature(allocator_api, str_internals)]

pub mod compiler;
pub mod cursor;
pub mod parse;
pub mod spec;
pub mod macros;

pub type Result<T> = std::result::Result<T, &'static str>;

/*#[cfg(test)]
mod tests {
    use crate::token::Cursor;

    #[test]
    fn getc_works() {
        let mut cursor = Cursor::new("abc123");
        assert_eq!(cursor.getv(), Some('a'));
        assert_eq!(cursor.getv(), Some('b'));
        assert_eq!(cursor.getv(), Some('c'));
        assert_eq!(cursor.getv(), Some('1'));
        assert_eq!(cursor.getv(), Some('2'));
        assert!(!cursor.is_eof(), "Cursor should not be exhausted.");
        assert_eq!(cursor.getv(), Some('3'));
    }
}*/
