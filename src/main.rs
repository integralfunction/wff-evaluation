use itertools::{Itertools, repeat_n};
use std::iter::Peekable;
use std::slice::Iter;
// use unicode_segmentation::UnicodeSegmentation;

use tabled::settings::{Alignment, object::Columns};
use tabled::{builder::Builder, settings::Style};

use crate::formula::Formula;
use crate::token::Token;

use std::io;

mod formula;
mod node;
mod operators;
mod parser;
mod term;
mod token;

fn main() {
    print!("> ");
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer).unwrap();

    let mut s = Formula::new(buffer);

    let tokens = s.tokenize().unwrap();
    // println!("{:#?}", tokens);
    let token_iter: Peekable<Iter<'_, Token>> = tokens.iter().peekable();
    let mut parser = parser::Parser::new(token_iter);
    let ast = Box::new(parser.parse());
    // println!("{:#?}", ast);
    let number_of_terms = s.get_all_terms().len();
    // const BASE: usize = 2;
    // let mut b = Builder::with_capacity(BASE.pow(number_of_terms.try_into().unwrap()), 1);

    let mut b = Builder::new();

    /* First row */
    let mut first_row: Vec<String> = vec![];
    for term in s.get_all_terms() {
        first_row.push(term.symbol.clone().to_string());
    }
    first_row.push(ast.to_string());
    b.push_record(first_row);

    /* All other rows */
    for truth_value_tuple in
        repeat_n(vec![false, true].into_iter(), number_of_terms).multi_cartesian_product()
    {
        // println!("{:#?}", truth_value_tuple);
        let mut rec: Vec<String> = vec![];
        for v in truth_value_tuple.clone() {
            rec.push(v.to_string());
        }
        s.update_value_from_vecs(truth_value_tuple);
        rec.push(ast.eval(&s).to_string());
        // println!("{:#?}", eval(&ast, &s));
        b.push_record(rec);
    }

    println!(); // Because reading stdin from earlier didn't add a \n
    let mut table = b.build();
    table.with(Style::modern());
    table.modify(Columns::last(), Alignment::center());

    println!("{}", table);
}

// pub fn unicode_split(x: &str) -> Vec<&str> {
//     return x.graphemes(true).collect::<Vec<&str>>();
// }
