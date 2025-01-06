use anyhow::Result;
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::prelude::*;
use std::io::{stdin, Read};
use text::{newline, whitespace};

#[derive(Clone, Debug)]
enum Block {
    Empty,
    Graf(Vec<String>),
    Heading(String),
}

#[derive(Debug)]
struct Doc(Vec<Block>);

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input)?;

    match parser().parse(input.clone()) {
        Ok(ast) => println!("{ast:?}"),
        Err(errs) => errs.into_iter().for_each(|e| {
            Report::build(ReportKind::Error, e.span())
                .with_message(e.to_string())
                .with_label(Label::new(e.span()).with_message(e).with_color(Color::Red))
                .finish()
                .print(Source::from(&input))
                .unwrap()
        }),
    };

    Ok(())
}

fn parser() -> impl Parser<char, Doc, Error = Simple<char>> {
    let content = newline()
        .not()
        .repeated()
        .at_least(1)
        .map(|chars| chars.into_iter().collect());

    let empty_block = newline().then(whitespace()).to(Block::Empty);

    //let marker = just("#")

    let graf_block = content
        .separated_by(newline())
        .at_least(1)
        .map(|lines| Block::Graf(lines));

    let heading_block = just('#')
        .then(whitespace())
        .ignore_then(content)
        .map(|line| Block::Heading(line));

    choice((heading_block, graf_block, empty_block))
        .repeated()
        .map(|blocks| Doc(blocks))
        .then_ignore(end())
}
