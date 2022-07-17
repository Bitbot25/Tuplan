// TODO: Warn when unicode combining characters are detected as they may be misinterpreted by the compiler.

use crate::parse::{Parse, ParseStream};
use crate::Result;
use unicode_xid::UnicodeXID;

#[repr(u8)]
pub enum LineBreak {
    CRLF,
    CR,
    LF,
    NEL,
}

impl Parse for LineBreak {
    fn parse(stream: &mut ParseStream) -> Result<Self> {
        Ok(match (stream.advance(), stream.snapshot()) {
            (Some('\u{000D}'), _snap) => {
                if stream.eats('\u{000A}') {
                    LineBreak::CRLF
                } else {
                    LineBreak::CR
                }
            }
            (Some('\u{000A}'), _snap) => LineBreak::LF,
            (Some('\u{0085}'), _snap) => LineBreak::NEL,
            (_, snap) => {
                stream.rewind(snap);
                return Err("Unrecognized linebreak. Expected CRLF, CR, LF or NEL.");
            }
        })
    }
}

pub trait UnicodeSpec {
    fn is_xid_start(&self) -> bool;
    fn is_xid_continue(&self) -> bool;
    fn is_whitespace(&self) -> bool;
}

#[cfg(feature = "char_spec")]
impl UnicodeSpec for char {
    fn is_xid_start(&self) -> bool {
        <Self as UnicodeXID>::is_xid_start(*self)
    }

    fn is_xid_continue(&self) -> bool {
        <Self as UnicodeXID>::is_xid_continue(*self)
    }

    fn is_whitespace(&self) -> bool {
        matches!(
            *self,
            '\u{0009}'
                | '\u{000A}'
                | '\u{000B}'
                | '\u{000C}'
                | '\u{000D}'
                | '\u{0020}'
                | '\u{0085}'
                | '\u{00A0}'
                | '\u{1680}'
                | '\u{2000}'
                | '\u{2001}'
                | '\u{2002}'
                | '\u{2003}'
                | '\u{2004}'
                | '\u{2005}'
                | '\u{2006}'
                | '\u{2007}'
                | '\u{2008}'
                | '\u{2009}'
                | '\u{200A}'
                | '\u{2028}'
                | '\u{2029}'
                | '\u{202F}'
                | '\u{205F}'
                | '\u{3000}'
        )
    }
}

// TODO: Make a set_unicode_spec function in compiler.rs
pub fn is_whitespace<T: UnicodeSpec>(val: T) -> bool {
    val.is_whitespace()
}

pub fn is_xid_start<T: UnicodeSpec>(val: T) -> bool {
    val.is_xid_start()
}

pub fn is_xid_continue<T: UnicodeSpec>(val: T) -> bool {
    val.is_xid_continue()
}

pub fn parse_linebreak(stream: &mut ParseStream) -> Result<LineBreak> {
    LineBreak::parse(stream)
}
