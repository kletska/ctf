use std::collections::HashSet;

use crate::internal_representation::{RawAST, RawLambda, AST};

#[derive(Eq, PartialEq)]
pub enum Token {
    Sym(String),
    BeginLamArgs,
    BeginLamBody,
    EndLam,
    BeginApp,
    EndApp,
}

impl Token {
    fn from_str(token: &str) -> Token {
        use self::Token::*;
        match token {
            "{" => BeginLamArgs,
            "~" => BeginLamBody,
            "}" => EndLam,
            "[" => BeginApp,
            "]" => EndApp,
            _ => Sym(token.to_string()),
        }
    }
}

fn tokenize(program: String) -> Vec<Token> {
    program
        .replace("{", " { ")
        .replace("~", " ~ ")
        .replace("}", " } ")
        .replace("[", " [ ")
        .replace("]", " ] ")
        .split_whitespace()
        .map(Token::from_str)
        .collect()
}

pub enum ParseErr {
    IncorectBracketSeq,
    ProgramCantStartWithSymbol,
    WrongLambdaBody,
    NoLambdaArgs,
    NoLambdaBody,
    UnexpectedSymbol,
    NoArgsInApplication,
    OnlySymbolAllowedInLambdaArgs,
}

fn get_top_elem(stack: &mut Vec<RawAST>, endToken: &Token) -> Result<RawAST, ParseErr> {
    match stack.pop() {
        Some(top) => {
            match top {
                RawAST::Lam(RawLambda::Args(_)) if *endToken == Token::BeginLamBody => {
                    Ok(top)
                }
                
                RawAST::Lam(RawLambda::Full(_, _)) if *endToken == Token::EndLam => {
                    Ok(top)
                }

                RawAST::App(_) if *endToken == Token::EndApp => {
                    Ok(top)
                }
                _ => Err(ParseErr::IncorectBracketSeq)
            }
        }
        None => Err(ParseErr::IncorectBracketSeq),
    }
}

pub fn raw_parse(program: Vec<Token>) -> Result<RawAST, ParseErr> {
    let mut stack: Vec<RawAST> = Vec::new();

    for i in program {
        match i {
            Token::Sym(s) => {
                match stack.last_mut() {
                    Some(top) => top.push(RawAST::Sym(s)),
                    None => return Err(ParseErr::ProgramCantStartWithSymbol),
                }
            }
            Token::BeginLamArgs => {
                stack.push(RawAST::Lam(RawLambda::Args(Vec::new())));
            }
            Token::BeginLamBody => {
                let args = get_top_elem(&mut stack, &i)?;
                match args {
                    RawAST::Lam(RawLambda::Args(vec)) => stack.push(RawAST::Lam(RawLambda::Full(vec, Vec::new()))),
                    _ => return Err(ParseErr::NoLambdaArgs),
                }
            }
            Token::BeginApp => {
                stack.push(RawAST::App(Vec::new()));
            }
            _ => {
                let top = get_top_elem(&mut stack, &i)?;
                match stack.last_mut() {
                    Some(new_top) => new_top.push(top),
                    None => stack.push(top),
                }
            }
        }
    }
    
    if stack.len() == 1 {
        Ok(stack.remove(0))
    } else {
        Err(ParseErr::IncorectBracketSeq)
    }
}

type ParseRes = Result<AST, ParseErr>;


fn parse_application(args: &[RawAST], store: &mut HashSet<String>) -> ParseRes {
    if let Some((head, tail)) = args.split_first() {
        let mut result = parse_AST(head, store)?;
        for i in tail {
            result = AST::App(Box::new(result), Box::new(parse_AST(i, store)?));
        }
        Ok(result)
    } else {
        Err(ParseErr::NoArgsInApplication)
    } 
}

fn parse_lambda(args: &[RawAST], body: &[RawAST], store: &mut HashSet<String>) -> ParseRes {
    if let Some((head, tail)) = args.split_first() {
        match head {
            RawAST::Sym(s) => {
                if store.contains(s) {
                    Ok(AST::Lam(s.clone(), Box::new(parse_lambda(tail, body, store)?)))
                } else {
                    store.insert(s.clone());
                    let result = Ok(AST::Lam(s.clone(), Box::new(parse_lambda(tail, body, store)?)));
                    store.remove(s);
                    result
                } 
            },
            _ => Err(ParseErr::OnlySymbolAllowedInLambdaArgs),
        }
    } else {
        parse_application(body, store)
    }
}

fn parse_AST(program: &RawAST, store: &mut HashSet<String>) -> ParseRes {
    match program {
        RawAST::Sym(s) => {
            if store.contains(s) {
                Ok(AST::Sym(s.clone()))
            } else {
                Err(ParseErr::UnexpectedSymbol)
            }
        }
        RawAST::Lam(RawLambda::Args(_)) => Err(ParseErr::NoLambdaBody),
        RawAST::Lam(RawLambda::Full(args, body)) => parse_lambda(args, body, store),
        RawAST::App(vec) => parse_application(vec, store),
    }
}

fn parse(program: String) -> ParseRes {
    let mut store: HashSet<String> = HashSet::new();
    parse_AST(&raw_parse(tokenize(program))?, &mut store)
}
