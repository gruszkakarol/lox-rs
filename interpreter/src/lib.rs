#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lazy_static;

mod ast;
mod error;
mod expr;
mod interpreter;
mod lexer;
mod parser;
mod runtime_value;
mod statement;
mod token;
mod utils;
use crate::ast::print_ast;
use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{self, BufRead};
use std::{env, fs::File};

pub fn run_prompt() {
    loop {
        println!(">");
        let mut code = String::new();
        let stdin = io::stdin();
        stdin.lock().read_line(&mut code).unwrap();
        run_code(&code);
    }
}

pub fn run_code(source_code: &str) -> Result<(), Error> {
    let mut lexer = Lexer::new(source_code);
    match lexer.scan_tokens() {
        Ok(tokens) => {
            println!("{:#?}", tokens);
            let mut parser = Parser::new(&tokens);
            let stmts = parser.parse_tokens()?;
            let mut interpreter = Interpreter::new();
            interpreter.interpret(&stmts)?;
            Ok(())
        }
        Err(e) => {
            println!("{:#?}", e);
            Err(e[0].clone())
        }
    }
}

pub fn run_file(path: &str) {
    match File::open(path) {
        Ok(f) => {
            let mut source_code = String::new();
            let mut buf_reader = BufReader::new(f);
            buf_reader
                .read_to_string(&mut source_code)
                .expect("This file is empty!");
            run_code(&source_code);
        }
        _ => println!("This file doesn't exist!"),
    }
}
