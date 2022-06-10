use syntax_rs::{
    parse::{Parse, ParseStream},
    simple_tok_spanned, spec, Span, Spanned,
};
use unicode_xid::UnicodeXID;

#[derive(Debug)]
struct LitInt {
    value: i64,
}

impl Parse for LitInt {
    fn parse(stream: &mut ParseStream) -> syntax_rs::Result<Self> {
        fn to_u32(chars: &str) -> syntax_rs::Result<u32> {
            if chars.len() == 0 {
                return Err("Expected integer.");
            }
            let mut number = 0;

            // We don't need to do .chars() here because we are only dealing with numbers.
            for c in chars.as_bytes() {
                let digit: u32 = *c as u32 - 0x30;
                number *= 10;
                number += digit;
            }
            Ok(number)
        }

        stream.try_parse(|stream| {
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
    fn parse(stream: &mut ParseStream) -> syntax_rs::Result<Self> {
        stream.parse::<Quote>()?;

        let inside = stream.try_parse(|stream| {
            Ok(LitStr {
                val: String::from(stream.cur().advance_while(|c| c != '\"')),
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
    fn parse(stream: &mut ParseStream) -> syntax_rs::Result<Self> {
        if let Ok(lit_int) = stream.parse::<LitInt>() {
            Ok(Literal::Int(lit_int))
        } else if let Ok(lit_str) = stream.parse::<LitStr>() {
            Ok(Literal::String(lit_str))
        } else {
            Err("Expected integer or str.")
        }
    }
}

simple_tok_spanned!(Quote, "\"");

#[derive(Debug)]
struct Ident {
    string: String,
    span: Span,
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
    fn parse(stream: &mut ParseStream) -> syntax_rs::Result<Self> {
        stream.try_parse(|stream| {
            let snap = stream.snapshot();
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
                Err("Expected identifier.")
            } else {
                Ok(Ident {
                    string: String::from(slice),
                    span: stream.since(snap),
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
    fn parse(stream: &mut ParseStream) -> syntax_rs::Result<Self> {
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
    fn parse(stream: &mut ParseStream) -> syntax_rs::Result<Self> {
        stream.try_parse(|stream| {
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

// NOTE: A #[derive(Parse)] and #[derive(Spanned)] will probably be added in the future.
#[derive(Debug)]
enum Token {
    Punctuation(Punctuation),
    Literal(Literal),
    Symbol(Symbol),
}

impl Parse for Token {
    fn parse(stream: &mut ParseStream) -> syntax_rs::Result<Self> {
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

const CODE: &str = "1+2+3+4+5+6+7+8+9+10";

fn main() {
    println!("{:?}", syntax_rs::exhaustive_parse::<Token>(CODE));
}
