use crate::*;
use std::iter::Peekable;
use std::str::Chars;

impl Expr {
    pub fn parse(input: &str) -> Result<Expr, String> {
        let mut chars = input.chars().peekable();
        parse_expr(&mut chars)
    }
}

fn parse_expr(chars: &mut Peekable<Chars>) -> Result<Expr, String> {
    let mut expr = parse_atom(chars)?;
    skip_whitespace(chars);
    while let Some(&ch) = chars.peek() {
        if ch == ')' || ch == '.' {
            break;
        }
        let rhs = parse_atom(chars)?;
        expr = Expr::Apply(Box::new(expr), Box::new(rhs));
        skip_whitespace(chars);
    }
    Ok(expr)
}

fn parse_atom(chars: &mut Peekable<Chars>) -> Result<Expr, String> {
    skip_whitespace(chars);
    match chars.peek() {
        Some(&ch) if is_lambda(ch) => parse_lambda(chars),
        Some(&'(') => {
            chars.next(); // consume '('
            let expr = parse_expr(chars)?;
            skip_whitespace(chars);
            match chars.next() {
                Some(')') => Ok(expr),
                _ => Err("Expected ')'".to_string()),
            }
        }
        Some(&ch) if is_ident_start(ch) => parse_variable(chars),
        Some(&ch) => Err(format!("Unexpected character: '{}'", ch)),
        None => Err("Unexpected end of input".to_string()),
    }
}

fn parse_lambda(chars: &mut Peekable<Chars>) -> Result<Expr, String> {
    chars.next(); // consume λ or \
    skip_whitespace(chars);
    let var = parse_ident(chars)?;
    skip_whitespace(chars);
    match chars.next() {
        Some('.') => (),
        _ => return Err("Expected '.' after lambda parameter".to_string()),
    }
    let body = parse_expr(chars)?;
    Ok(Expr::Lambda(var, Box::new(body)))
}

fn parse_variable(chars: &mut Peekable<Chars>) -> Result<Expr, String> {
    let name = parse_ident(chars)?;
    Ok(Expr::Variable(name))
}

fn parse_ident(chars: &mut Peekable<Chars>) -> Result<String, String> {
    let mut name = String::new();
    while let Some(&ch) = chars.peek() {
        if ch.is_alphanumeric() || ch == '_' {
            name.push(ch);
            chars.next();
        } else {
            break;
        }
    }
    if name.is_empty() {
        Err("Expected identifier".to_string())
    } else {
        Ok(name)
    }
}

fn skip_whitespace(chars: &mut Peekable<Chars>) {
    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}

fn is_lambda(ch: char) -> bool {
    ch == 'λ' || ch == '\\'
}

fn is_ident_start(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}
