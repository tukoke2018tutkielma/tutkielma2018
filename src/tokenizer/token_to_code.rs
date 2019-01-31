use super::Token::{self, *};
use super::Op;
use std::io::{Error, ErrorKind};

fn convert_single(token: Token) -> Result<String, Error> {

    let result = match token {
        SemiColon => ";",
        OpenBrace => "{",
        CloseBrace => "}",
        OpenParen => "(",
        CloseParen => ")",
        OpenSquareBracket => "[",
        CloseSquareBracket => "]",
        Dot => ".",
        Colon => ",",
        _ => return Err(Error::new(ErrorKind::Other, "Failed to parse token")),
    };
    Ok(result.to_string())
}

pub fn try_to_rust(mut token_list: Vec<Token>) -> Result<String, Error> {
    let mut result: String = String::new();
    if token_list.len() == 1 {
        println!("{:#?}", token_list);
        let token = if let Some(token) = token_list.pop() { token } else {
            return Err(Error::new(ErrorKind::Other, "Failed!"))
        };
        result = convert_single(token)?.to_string();
    } else {
        match token_list.as_slice() {
             _ => return Err(Error::new(ErrorKind::Other, "Failed to parse token"))
        }
        for token in token_list {
            println!("{:?}", token);
        }
        println!("{:?}", convert_single(Token::SemiColon)?);

        println!("{:?}", convert_single(Token::Comment("/* asd */".to_string()))?);
    }
    Ok(result)
}

fn try_to_java() {
    unimplemented!()
}
