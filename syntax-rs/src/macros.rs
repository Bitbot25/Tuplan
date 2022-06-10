#[macro_export]
macro_rules! simple_tok {
    ($ident:ident, $str:literal) => {
        #[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
        struct $ident;

        impl $crate::parse::Parse for $ident {
            fn parse(stream: &mut $crate::parse::ParseStream) -> $crate::Result<$ident> {
                match stream.cur().peek_n($str.len()) {
                    Some(array) => {
                        if array == $str {
                            // TODO: This can be optimized
                            stream.cur().advance_n($str.len());
                            Ok($ident)
                        } else {
                            Err($crate::concat_all!("Expected `", $str, "`."))
                        }
                    }
                    None => Err($crate::concat_all!("Found EOF but expected `", $str, "`.")),
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
    ($ident:ident, $str:literal) => {
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
                        match stream.cur().peek_n($str.len()) {
                            Some(array) => if array == $str {
                                stream.cur().advance_n($str.len());
                                Ok($ident { span: $crate::Span { begin: index, end: stream.cur().index() } })
                            } else {
                                Err($crate::concat_all!("Expected `", $str, "`."))
                            },
                            None => Err($crate::concat_all!("Found EOF but expected `", $str, "`.")),
                        }
                    }
                }
            } else {
                $crate::simple_tok!($ident,$str);
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
