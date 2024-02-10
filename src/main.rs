use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

use crate::{eval::evaluate, lexer::Lexer, parser::ASTBuilder};

pub mod eval;
pub mod lexer;
pub mod parser;

fn main() {
    let stdin = io::stdin();

    let mut input_iter = stdin.lock().lines();
    loop {
        let input = input_iter.next().unwrap().unwrap();
        let mut lx = Lexer::new(&input);
        let mut token_vec = VecDeque::new();
        while let Some(token) = lx.next_token().unwrap() {
            token_vec.push_back(token);
        }

        let answer = evaluate(ASTBuilder::new(token_vec).build_ast().expect("bad token"));

        println!(" = {}", answer);
    }
}
