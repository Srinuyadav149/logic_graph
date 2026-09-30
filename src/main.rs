mod logic_graph;
mod flgn;

use std::io;
use flgn::tokenizer::{Token, tokenize_flgn};

fn main() -> io::Result<()> {
    let tokens: Vec<Token> = tokenize_flgn("logic.flgn")?;
    println!("{:?}", tokens);
    Ok(())
}