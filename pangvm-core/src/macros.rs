#[macro_export]
macro_rules! simple_tok {
    ($ident:ident, $($chars:literal),+) => {
        struct $ident;

        impl $crate::parse::Parse for $ident {
            fn parse(stream: &mut $crate::parse::ParseStream) -> $crate::Result<$ident> {
                match stream.cur().peek_range(count!($($chars)+)) {
                    Some(array) => if array == &[$($chars),+] { Ok($ident) } else { Err(concat_all!("Expected `", $($chars),+, "`.")) },
                    None => Err(concat_all!("Found EOF but expected `", $($chars),+, "`.")),
                }
            }
        }
    };
}

macro_rules! concat_all {
    ($val:literal,$($rest:literal),+) => {
        concat!($val, concat_all!($($rest),+))
    };
    ($single:literal) => {
        $single
    };
}

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}