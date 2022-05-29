use pangvm::{parse::{ParseStream, Parse}, token::Cursor};

// NOTE: This is all just experimenting. This should be implemented using macros later i think. Only the core ones like literal should be implemented using raw code.

#[derive(Debug)]
struct LitInt {
    value: i64,
}

impl Parse for LitInt {
    fn parse(stream: &mut ParseStream<'_>) -> pangvm::Result<Self> {
        stream.try_step(|mut cursor| {
            let mut is_first = true;
            cursor.getc_while(|c| {
                let b = c.is_digit(10) || (is_first && (c == '+' || c == '-'));
                is_first = false;
                b
            }, |val| val.parse::<i64>().map_err(|_e| "Expected integer.")).map(|parsed| (LitInt { value: parsed }, cursor))
        })
    }
}

#[derive(Debug)]
struct LitStr {
    val: String,
}

impl Parse for LitStr {
    fn parse(stream: &mut ParseStream<'_>) -> pangvm::Result<Self> {
        stream.parse::<PunctQuote>()?;

        let inside = stream.try_step(|mut cur| {
            Ok((LitStr { val: cur.getc_while(|c| c != '\"', |val| String::from(val)) }, cur))
        });

        stream.parse::<PunctQuote>()?;
        inside
    }
}

#[derive(Debug)]
enum Literal {
    Int(LitInt),
    String(LitStr),
}

impl Parse for Literal {
    fn parse(stream: &mut ParseStream<'_>) -> pangvm::Result<Self> {
        if let Ok(lit_int) = stream.parse::<LitInt>() {
            Ok(Literal::Int(lit_int))
        } else if let Ok(lit_str) = stream.parse::<LitStr>() {
            Ok(Literal::String(lit_str))
        } else {
            Err("Expected integer or str.")
        }
    }
}

struct PunctQuote;

impl Parse for PunctQuote {
    fn parse(stream: &mut ParseStream<'_>) -> pangvm::Result<Self> {
        stream.try_step(|mut cur| {
            match cur.getc() {
                Some('\"') => Ok((PunctQuote, cur)),
                _ => Err("Expected '\"'"),
            }
        })
    }
}

enum Punctuation {
    Quote(PunctQuote),
}

enum TokenTree {
    Punctuation(),
    Delimited(),
    Literal(Literal),
}

fn main() {
    let mut stream = ParseStream::new("cool");
    println!("{:?}", stream.parse::<Literal>());
}