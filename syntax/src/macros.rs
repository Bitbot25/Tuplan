// TODO: Add simple_tok_spanned.
#[macro_export]
macro_rules! simple_tok {
    ($ident:ident, $($chars:literal),+) => {
        #[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
        struct $ident;

        impl $crate::parse::Parse for $ident {
            fn parse(stream: &mut $crate::parse::ParseStream) -> $crate::Result<$ident> {
                match stream.cur().peek_range($crate::count_tt!($($chars)+)) {
                    Some(array) => if array == &[$($chars),+] {
                        stream.cur().advance_n($crate::count_tt!($($chars)+));
                        Ok($ident)
                    } else {
                        Err($crate::concat_all!("Expected `", $($chars),+, "`."))
                    },
                    None => Err($crate::concat_all!("Found EOF but expected `", $($chars),+, "`.")),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! spanned_field {
    ($field:ident,$name:ident) => {
        impl $crate::Spanned for $name {
            fn span(&self) -> $crate::Span {
                self.$field
            }

            fn span_ref_mut(&mut self) -> &mut $crate::Span {
                &mut self.$field
            }
        }
    };
}

#[macro_export]
macro_rules! simple_tok_spanned {
    ($ident:ident, $($chars:literal),+) => {
        cfg_if::cfg_if! {
            if #[cfg(feature = "span")] {
                #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
                struct $ident {
                    span: $crate::Span
                }
                $crate::spanned_field!(span,$ident);

                impl $crate::parse::Parse for $ident {
                    fn parse(stream: &mut $crate::parse::ParseStream) -> $crate::Result<$ident> {
                        let index = stream.cur().index();
                        match stream.cur().peek_range($crate::count_tt!($($chars)+)) {
                            Some(array) => if array == &[$($chars),+] {
                                stream.cur().advance_n($crate::count_tt!($($chars)+));
                                Ok($ident { span: $crate::Span { begin: index, end: stream.cur().index() } })
                            } else {
                                Err($crate::concat_all!("Expected `", $($chars),+, "`."))
                            },
                            None => Err($crate::concat_all!("Found EOF but expected `", $($chars),+, "`.")),
                        }
                    }
                }
            } else {
                $crate::simple_tok!($ident,$($chars)+);
            }
        }
    };
}

#[macro_export]
macro_rules! concat_all {
    ($val:literal,$($rest:literal),+) => {
        concat!($val, $crate::concat_all!($($rest),+))
    };
    ($single:literal) => {
        $single
    };
}

#[macro_export]
macro_rules! count_tt {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + $crate::count_tt!($($xs)*));
}
