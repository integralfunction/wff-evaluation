use itertools::{Itertools, repeat_n};
use std::iter::Peekable;
use std::slice::Iter;
use tabled::settings::formatting::AlignmentStrategy;
use tabled::settings::object::Columns;
// use unicode_segmentation::UnicodeSegmentation;

use tabled::settings::Alignment;
use tabled::{builder::Builder, settings::Style};

use crate::formula::Formula;
use crate::operators::{BinaryOperator, UnaryOperator};
use crate::term::Term;

use std::io;

mod formula;
mod operators;
mod parser;
mod term;

#[derive(Debug)]
enum Token {
    RightParen,
    LeftParen,
    Term(char),
    Not,
    And,
    Or,
    // If,
    // Iff,
    End,
}

#[derive(Debug)]
pub enum Node {
    Leaf(char),
    UnaryExpr {
        op: UnaryOperator,
        child: Box<Node>,
    },
    BinaryExpr {
        op: BinaryOperator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

fn eval(tree: &Box<Node>, f: &Formula) -> bool {
    match **tree {
        Node::Leaf(symbol) => f.term_from_symbol(symbol).unwrap().value,
        Node::UnaryExpr { ref op, ref child } => {
            return op.eval(eval(child, f));
        }
        Node::BinaryExpr {
            ref op,
            ref lhs,
            ref rhs,
        } => return op.eval(eval(&lhs, f), eval(&rhs, f)),
    }
}

fn main() {
    print!("> ");
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer).unwrap();

    let mut s = Formula::from(buffer);
    let tokens = s.tokenize().unwrap();
    // println!("{:#?}", tokens);
    let token_iter: Peekable<Iter<'_, Token>> = tokens.iter().peekable();
    let mut parser = parser::Parser::new(token_iter);
    let ast = Box::new(parser.parse().unwrap());
    // println!("{:#?}", ast);
    let number_of_terms = s.get_all_terms().len();
    const BASE: usize = 2;
    // let mut b = Builder::with_capacity(BASE.pow(number_of_terms.try_into().unwrap()), 1);

    let mut b = Builder::with_capacity(0, 0);

    let mut first_row: Vec<String> = vec![];
    for term in s.get_all_terms() {
        first_row.push(term.symbol.clone().to_string());
    }
    first_row.push(s.rawstring.clone());
    b.push_record(first_row);
    // b.push_record(["T", "F", "F"]);
    for truth_value_tuple in
        repeat_n(vec![false, true].into_iter(), number_of_terms).multi_cartesian_product()
    {
        // println!("{:#?}", truth_value_tuple);
        let mut rec: Vec<String> = vec![];
        for v in truth_value_tuple.clone() {
            rec.push(v.to_string());
        }
        s.update_value_from_vecs(truth_value_tuple);
        rec.push(eval(&ast, &s).to_string());
        // println!("{:#?}", eval(&ast, &s));

        b.push_record(rec);
        // for (index, term) in s.terms.iter().enumerate() {
        //     println!("{}, {:#?}", index, term)
        // }
    }

    println!();
    let mut table = b.build();
    table.with(Style::modern());
    table.modify(Columns::last(), Alignment::center());

    println!("{}", table);
}

// pub fn unicode_split(x: &str) -> Vec<&str> {
//     return x.graphemes(true).collect::<Vec<&str>>();
// }
