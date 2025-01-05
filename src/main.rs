use std::io::{stdin, Read};

use chumsky::prelude::*;

#[derive(Clone, Debug)]
enum AST {
    Text(String),
}

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let output = parser().parse(input);
    println!("{output:?}");
}

fn parser() -> impl Parser<char, AST, Error = Simple<char>> {
    any()
        .repeated()
        .map(|chars| AST::Text(chars.into_iter().collect()))
        .then_ignore(end())
}
