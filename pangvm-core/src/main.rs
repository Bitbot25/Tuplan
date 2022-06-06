use pangvm_core::{
    parse::{Parse, ParseStream},
    spec, Span, Spanned,
};
use unicode_xid::UnicodeXID;

// NOTE: This is all just experimenting. This should be implemented using macros later i think. Only the core ones like literal should be implemented using raw code.
// TODO: Make a #[derive(Parse)] when all fields are T: Parse.

// TODO: Make the error messages more epic.

#[derive(Debug)]
struct LitInt {
    value: i64,
}

impl Parse for LitInt {
    fn parse(stream: &mut ParseStream) -> pangvm_core::Result<Self> {
        fn to_u32(chars: &[char]) -> pangvm_core::Result<u32> {
            if chars.len() == 0 {
                return Err("Expected integer.");
            }
            let mut number = 0;
            for c in chars {
                let digit = *c as u32 - 0x30;
                number *= 10;
                number += digit;
            }
            Ok(number)
        }

        stream.virtual_parse(|stream| {
            let negative = stream.cur().consume('-');
            let mut value = to_u32(stream.cur().advance_while(|c| c.is_digit(10)))? as i64;
            if negative {
                value *= -1;
            }

            Ok(LitInt { value })
        })
    }
}

#[derive(Debug)]
struct LitStr {
    val: String,
}

impl Parse for LitStr {
    fn parse(stream: &mut ParseStream) -> pangvm_core::Result<Self> {
        stream.parse::<Quote>()?;

        let inside = stream.virtual_parse(|stream| {
            Ok(LitStr {
                val: stream
                    .cur()
                    .advance_while(|c| c != '\"')
                    .into_iter()
                    .collect(),
            })
        });

        stream.parse::<Quote>()?;
        inside
    }
}

#[derive(Debug)]
enum Literal {
    Int(LitInt),
    String(LitStr),
}

impl Parse for Literal {
    fn parse(stream: &mut ParseStream) -> pangvm_core::Result<Self> {
        if let Ok(lit_int) = stream.parse::<LitInt>() {
            Ok(Literal::Int(lit_int))
        } else if let Ok(lit_str) = stream.parse::<LitStr>() {
            Ok(Literal::String(lit_str))
        } else {
            Err("Expected integer or str.")
        }
    }
}

#[derive(Debug)]
struct Quote;

impl Parse for Quote {
    fn parse(stream: &mut ParseStream) -> pangvm_core::Result<Self> {
        stream.virtual_parse(|stream| match stream.cur().advance() {
            Some('\"') => Ok(Quote),
            _ => Err("Expected '\"'"),
        })
    }
}

#[derive(Debug)]
struct Ident {
    string: String,
    span: Span
}

impl Spanned for Ident {
    fn span(&self) -> Span {
        self.span
    }

    fn span_ref_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl Parse for Ident {
    fn parse(stream: &mut ParseStream) -> pangvm_core::Result<Self> {
        stream.virtual_parse(|stream| {
            let span = stream.push_span();
            let mut first_c = false;
            let slice = stream.cur().advance_while(|c| {
                if first_c {
                    first_c = false;
                    UnicodeXID::is_xid_start(c)
                } else {
                    UnicodeXID::is_xid_continue(c)
                }
            });
            if slice.is_empty() {
                println!("no identifier");
                Err("Expected identifier.")
            } else {
                Ok(Ident {
                    string: slice.into_iter().collect(),
                    span: span.into_inner(),
                })
            }
        })
    }
}

#[derive(Debug)]
enum Symbol {
    KwFunction,
    KwLet,
    KwIf,
    Ident(Ident),
}

impl Parse for Symbol {
    fn parse(stream: &mut ParseStream) -> pangvm_core::Result<Self> {
        // FIXME: This is a dumb way to do shit.
        let ident: Ident = stream
            .parse()
            .map_err(|_e| "Expected identifier, `function`, `let` or `if`.")?;

        Ok(match ident.string.as_str() {
            "fun" => Symbol::KwFunction,
            "let" => Symbol::KwLet,
            "if" => Symbol::KwIf,
            _ => Symbol::Ident(ident),
        })
    }
}

#[derive(Debug)]
enum Punctuation {
    Plus,
    Minus,
    Star,
    Slash,
}

impl Parse for Punctuation {
    fn parse(stream: &mut ParseStream) -> pangvm_core::Result<Self> {
        stream.virtual_parse(|stream| {
            Ok(
                match stream
                    .cur()
                    .advance()
                    .ok_or("Expected `+`, `-`, `*` or `/`.")?
                {
                    '+' => Punctuation::Plus,
                    '-' => Punctuation::Minus,
                    '*' => Punctuation::Star,
                    '/' => Punctuation::Slash,
                    _ => return Err("Expected `+`, `-`, `*` or `/`."),
                },
            )
        })
    }
}

#[derive(Debug)]
enum Token {
    Punctuation(Punctuation),
    Literal(Literal),
    Symbol(Symbol),
}

impl Parse for Token {
    fn parse(stream: &mut ParseStream) -> pangvm_core::Result<Self> {
        stream.skip_all(spec::is_whitespace);
        if let Ok(lit) = stream.parse::<Literal>() {
            Ok(Token::Literal(lit))
        } else if let Ok(punctuation) = stream.parse::<Punctuation>() {
            Ok(Token::Punctuation(punctuation))
        } else if let Ok(symbol) = stream.parse::<Symbol>() {
            Ok(Token::Symbol(symbol))
        } else {
            Err("Expected punctuation, symbol or literal.")
        }
    }
}

// TODO: Maybe i should implement a Staging struct?

const CODE: &str = "€";

fn main() {
    let chars: Vec<char> = CODE.chars().collect();
    let mut stream = ParseStream::new(chars.as_slice());
    println!("{:?}", stream.exhaustive_parse::<Token>());
}