mod parsers;
mod token_to_code;
use self::parsers::*;
use self::token_to_code::*;
use std::io::{Error, ErrorKind};

#[derive(Debug,PartialEq,Clone)]
pub enum Op {
    Assign,
    Equals,
    Add,
    Substract,
    Divide,
    Multiply,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Debug,PartialEq,Clone)]
pub enum Token {
    Unknown(char),
    Whitespace,
    Keyword(String),
    Identifier(String),
    Number(i32),
    Character(String),
    StringToken(String),
    Comment(String),
    Operator(Op),
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenSquareBracket,
    CloseSquareBracket,
    Dot,
    Colon,
    SemiColon,
}
#[derive(Debug,PartialEq,Clone)]
pub struct CodeToken {
        tokens: Vec<Token>,
        original: String
}

pub fn tokenize(string: &str) -> CodeToken {
    let mut result: Vec<Token> = vec![];

    let mut char_iter = string.chars().peekable();

    loop {
        if let Some(c) = char_iter.next() {
            let token = match c {
                ' ' | '\t' | '\n'   => parse_whitespace(&mut char_iter),
                chr @ '0'...'9'     => parse_number(chr, &mut char_iter),
                '.'                 => Token::Dot,
                ';'                 => Token::SemiColon,
                ':'                 => Token::Colon,
                '('                 => Token::OpenParen,
                ')'                 => Token::CloseParen,
                '{'                 => Token::OpenBrace,
                '}'                 => Token::CloseBrace,
                '['                 => Token::OpenSquareBracket,
                ']'                 => Token::CloseSquareBracket,
                '-'                 => Token::Operator(Op::Substract),
                '+'                 => Token::Operator(Op::Add),
                '*'                 => Token::Operator(Op::Multiply),
                '='                 => parse_equals(&mut char_iter),
                '\''                => parse_character(c, &mut char_iter),
                '\"'                => parse_string(c, &mut char_iter),
                '>'                 => parse_greater(&mut char_iter),
                '<'                 => parse_less(&mut char_iter),
                 _                  => parse_char(c, &mut char_iter)
            };

            if token != Token::Whitespace {
                result.push(token);
            }
        } else {
            break;
        }
    }

    CodeToken{tokens: result,original: string.to_string()}
}

pub fn to_code(tokens: CodeToken) -> Result<String, Error> {
    let token_list: Vec<Token> = tokens.tokens;
    let breaker_list = vec![Token::SemiColon,Token::OpenBrace,Token::CloseBrace];
    let mut result: String = String::new();
    let mut sublists: Vec<CodeToken> = Vec::new();
    let mut stringlist: Vec<String> = Vec::new();
    let mut to_cut = tokens.original.replace("\r\n", "\n").replace("\n", " ");
    while let Some(i) = to_cut.find(|c: char| c == '{' || c == '}' || c == ';') {
        print!("{} ",i);
        let cloned = to_cut.clone();
        let (part, rest) = cloned.split_at(i+1);
        println!("{:?}", part);
        println!("{:?}", rest);
        stringlist.push(part.trim().to_string());
        to_cut = rest.to_string();
    }
    println!("{:#?}", stringlist);

    // remove whitespace
    let token_list = token_list.into_iter().filter(|ref i|i != &&Token::Whitespace).collect::<Vec<_>>();
    let mut token_iter = token_list.into_iter().peekable();
    let mut index = 0;
    loop {
        let mut sub: Vec<Token> = Vec::new();
        loop {
            if let Some(tok) = token_iter.next() {
                sub.push(tok.clone());
                if breaker_list.contains(&tok) { break; };
            } else {
                break;
            }
        }
        let original = if let Some(orig) = stringlist.get(index){
            orig
        } else {
            eprintln!("Error occured and might be seen on row {}", index+1);
            "ERROR! SORRY FOR THE ERROR"
        };
        sublists.push(CodeToken{tokens: sub, original: original.to_string()});
        if let Some(_) = token_iter.peek() {
            index+=1;
            continue;
        }
        break;

    }
    for (i, list) in sublists.into_iter().enumerate() {
        println!("{:?}", i);
        let a = match try_to_rust(list.tokens) {
            Ok(code) => code,
            Err(_) => {println!("Failed on row {:?}, you might want to check it", i+1); list.original},
        };
        result.push_str(&format!("{}\n",a))
    }
    Ok(result)
}
