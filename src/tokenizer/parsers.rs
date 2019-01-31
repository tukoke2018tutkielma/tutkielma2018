extern crate core;

use std::str;
use std::i32;
use self::core::iter;

use super::Token;
use super::Op;

pub fn parse_whitespace(iter: &mut iter::Peekable<str::Chars>) -> Token {
    loop {
        if let Some(c) = iter.peek() {
            if !c.is_whitespace(){
                break
            }
        } else {
            break;
        }
        iter.next();
    }
    Token::Whitespace
}

pub fn parse_number(chr: char, iter: &mut iter::Peekable<str::Chars>) -> Token {
    let mut result_string: String = String::new();

    result_string.push(chr);

    loop {
        if let Some(c) = iter.peek() {
            if c.is_digit(10) {
                result_string.push(*c);
            } else {
                break;
            }
        } else {
            break;
        }
        iter.next();
    }

    let number = match i32::from_str_radix(result_string.as_str(), 10) {
        Ok(int) => int,
        Err(_) => abort("Error parsing int."),
    };

    Token::Number(number)
}

fn is_identifier_end(c: &char) -> bool {
    vec!['(', ')', ';', '{','}', '=', '<','>', '.', '+', '-', '/', '*'].iter().any(|x| x.eq(c))
}

fn parse_comment(result_string: &mut String, iter: &mut iter::Peekable<str::Chars>) -> Token{
    if result_string == "//"{
            loop {
                if let Some(c) = iter.peek() {
                    if c==&'\n' {
                        break
                    }
                    result_string.push(*c);
                } else {
                    break;
                }
                iter.next();
            }
    } else {
        let mut check_again=false;
        loop {
            if check_again {
                if let Some(c) = iter.peek() {
                    result_string.push(*c);
                    if c==&'/'{
                        break;
                    } else {
                        check_again=false;
                    }
                }else{
                    break;
                }
            } else if let Some(c) = iter.peek() {
                if c==&'*'{
                    check_again=true;
                }
                result_string.push(*c);
            } else {
                break;
            }
            iter.next();
        }
        iter.next();
    }

    let res = result_string.as_str();

    Token::Comment(res.to_string())
}


pub fn parse_char(c: char, mut iter: &mut iter::Peekable<str::Chars>) -> Token {
    let keywords = vec!["if", "else", "while", "for", "public", "int", "long", "float", "String",
                        "Integer", "private", "static", "void", "false", "true", "null"];
    let mut result_string: String = String::new();
    let mut comment = false;

    result_string.push(c);

    loop {
        if let Some(c) = iter.peek() {
            if c.is_whitespace() || is_identifier_end(&c) {
                break;
            }

            if result_string=="/" && c!=&'*' && c!=&'/'{
                return Token::Operator(Op::Divide)
            }
            result_string.push(*c);
            if result_string.len()==2 {
                if result_string=="//" || result_string=="/*"{
                    comment=true;
                }
            }

        } else {
            break;
        }
        iter.next();
        if comment {
            return parse_comment(&mut result_string, &mut iter)
        }
    }

    let res = result_string.as_str();

    if keywords.iter().any(|x| x.eq(&res)) {
        Token::Keyword(res.to_string())
    } else {
        Token::Identifier(res.to_string())
    }
}

pub fn parse_equals(iter: &mut iter::Peekable<str::Chars>) -> Token {
    let mut is_assign = true;

    if let Some(c) = iter.peek() {
        is_assign = *c != '=';
    }

    if !is_assign { iter.next(); }

    if is_assign { Token::Operator(Op::Assign) }
    else { Token::Operator(Op::Equals) }
}

pub fn parse_character(c: char, iter: &mut iter::Peekable<str::Chars>) -> Token {
    let mut result_string: String = String::new();
    result_string.push(c);
    loop {
        if let Some(c) = iter.peek() {
            result_string.push(*c);
            if c==&'\''{
                break;
            }
        }else{
            break;
        }
        iter.next();
    }
    iter.next();
    let res = result_string.as_str();
    Token::Character(res.to_string())
}

pub fn parse_string(c: char, iter: &mut iter::Peekable<str::Chars>) -> Token {
    let mut result_string: String = String::new();
    result_string.push(c);
    loop {
        if let Some(c) = iter.peek() {
            result_string.push(*c);
            if c==&'\"'{
                break;
            }
        }else{
            break;
        }
        iter.next();
    }
    iter.next();
    let res = result_string.as_str();
    Token::StringToken(res.to_string())
}

pub fn parse_greater(iter: &mut iter::Peekable<str::Chars>) -> Token {
    let mut is_greater = true;

    if let Some(c) = iter.peek() {
        is_greater = *c != '=';
    }

    if !is_greater { iter.next(); }

    if is_greater { Token::Operator(Op::GreaterThan) }
    else { Token::Operator(Op::GreaterThanOrEqual) }
}

pub fn parse_less(iter: &mut iter::Peekable<str::Chars>) -> Token {
    let mut is_less = true;

    if let Some(c) = iter.peek() {
        is_less = *c != '=';
    }

    if !is_less { iter.next(); }

    if is_less { Token::Operator(Op::LessThan) }
    else { Token::Operator(Op::LessThanOrEqual) }
}

fn abort<TResult>(description: &'static str) -> TResult {
    panic!(description);
}
