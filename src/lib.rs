#![feature(core_intrinsics)]

pub mod token;
pub mod parse;

pub type Result<T> = std::result::Result<T, &'static str>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
