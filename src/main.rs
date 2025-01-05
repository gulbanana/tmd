use anyhow::{anyhow, Result};
use chumsky::prelude::*;
use std::io::{stdin, Read};
use text::newline;

#[derive(Clone, Debug)]
struct AST(Vec<String>);

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input)?;

    let output = parser().parse(input).map_err(|err| anyhow!("{err:?}"))?;

    for graf in output.0.iter() {
        println!("<p>{0}</p>", graf.trim_end());
    }

    Ok(())
}

fn parser() -> impl Parser<char, AST, Error = Simple<char>> {
    let graf_break = newline().repeated().at_least(2);

    let graf = graf_break
        .not()
        .repeated()
        .at_least(1)
        .map(|chars| chars.into_iter().collect());

    graf.padded_by(graf_break.or_not())
        .repeated()
        .map(|grafs| AST(grafs))
        .then_ignore(end())
}
