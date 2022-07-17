use syntax_rs::parse::{Parse, ParseStream};
use syntax_rs::Result;

#[derive(Debug)]
struct UInt(u32);

impl Parse for UInt {
    fn parse(stream: &mut ParseStream) -> Result<Self> {
        uint(stream)
    }
}

#[derive(Debug)]
struct Add(Expr, Expr);
#[derive(Debug)]
struct Sub(Expr, Expr);
#[derive(Debug)]
struct Mul(Expr, Expr);
#[derive(Debug)]
struct Div(Expr, Expr);

#[derive(Debug)]
enum Expr {
    Add(Box<Add>),
    Sub(Box<Sub>),
    Mul(Box<Mul>),
    Div(Box<Div>),
    UInt(UInt),
}

impl Parse for Expr {
    fn parse(stream: &mut ParseStream) -> Result<Self> {
        binop(stream)
    }
}

fn binop(stream: &mut ParseStream) -> Result<Expr> {
    let mut lhs = term(stream)?;
    while let Some(op) = stream.eat_of(&['+', '-']) {
        let rhs = term(stream)?;
        lhs = match op {
            '+' => Expr::Add(Box::new(Add(lhs, rhs))),
            '-' => Expr::Sub(Box::new(Sub(lhs, rhs))),
            _ => unreachable!(),
        }
    }
    Ok(lhs)
}

fn term(stream: &mut ParseStream) -> Result<Expr> {
    let mut lhs = Expr::UInt(uint(stream)?);
    while let Some(op) = stream.eat_of(&['*', '/']) {
        let rhs = Expr::UInt(uint(stream)?);
        lhs = match op {
            '*' => Expr::Mul(Box::new(Mul(lhs, rhs))),
            '/' => Expr::Div(Box::new(Div(lhs, rhs))),
            _ => unreachable!(),
        }
    }
    Ok(lhs)
}

fn uint(stream: &mut ParseStream) -> Result<UInt> {
    let mut accum = 0;
    let mut changed = false;
    while let Some(c) = stream.advance_if(|c| c.is_digit(10)) {
        let digit = c as u32 - '0' as u32;
        accum *= 10;
        accum += digit;
        changed = true;
    }
    if changed {
        Ok(UInt(accum))
    } else {
        Err("Expected uint.")
    }
}

fn main() {
    println!("{:?}", syntax_rs::parse::<Expr>("1+2"))
}
