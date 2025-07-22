// use unicode_segmentation::UnicodeSegmentation;

use crate::Token;
use crate::term::Term;
// use crate::{Token, unicode_split};

#[derive(Debug)]
pub struct Formula {
    pub rawstring: String,
    pub terms: Vec<Term>,
}

impl From<&str> for Formula {
    fn from(rawstr: &str) -> Formula {
        return Formula {
            rawstring: rawstr.trim().split(' ').collect::<Vec<&str>>().join(""),
            terms: Formula::get_all_terms_initially(
                rawstr.trim().split(' ').collect::<Vec<&str>>().join(""),
            ),
        };
    }
}

impl From<String> for Formula {
    fn from(rawstr: String) -> Formula {
        return Formula {
            rawstring: rawstr.trim().split(' ').collect::<Vec<&str>>().join(""),
            terms: Formula::get_all_terms_initially(
                rawstr.trim().split(' ').collect::<Vec<&str>>().join(""),
            ),
        };
    }
}

impl Formula {
    fn get_all_terms_initially(rawstring: String) -> Vec<Term> {
        let mut all_terms: Vec<Term> = vec![];
        for char in rawstring.chars() {
            match char {
                ch if char.is_alphabetic() => {
                    let x = all_terms.iter().find(|&term| term.symbol == ch);
                    match x {
                        None => {
                            all_terms.push(Term {
                                symbol: ch,
                                value: true,
                            });
                        }
                        Some(_) => {}
                    }
                }
                _ => {}
            }
        }
        return all_terms;
    }

    pub fn get_all_terms(&self) -> &Vec<Term> {
        return &self.terms;
    }

    pub fn term_from_symbol(&self, symbol: char) -> Option<&Term> {
        let p = self.terms.iter().find(|&term| term.symbol == symbol);
        return p;
    }
    //

    pub fn update_value_from_vecs(&mut self, truth_values: Vec<bool>) {
        for (index, term) in self.terms.iter_mut().enumerate() {
            (term).set_value(truth_values[index]);
        }
    }
    // pub fn add_term(&mut self, t: Term) -> &Term {
    //     self.terms.push(t);
    //     return &self.terms.last().unwrap();
    // }
    // fn add_term(terms: &mut Vec<Term>, t: Term) -> &Term {
    //     terms.push(t);
    //     return terms.last().unwrap();
    // }
    // pub fn get_term(&self, c: char) -> Result<&Term, String> {
    //     // for t in self.terms.as_slice() {
    //     for t in self.terms.borrow_mut().as_slice() {
    //         if t.symbol == c {
    //             return Ok(t);
    //         }
    //     }
    //     return Err(format!("Can't find"));
    // }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = vec![];
        for char in self.rawstring.chars() {
            // println!("{}", char);
            match char {
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                '∧' => tokens.push(Token::And),
                '∨' => tokens.push(Token::Or),
                '¬' => tokens.push(Token::Not),
                ch if char.is_alphabetic() => {
                    tokens.push(Token::Term(ch));
                }
                _ => return Err(String::from("Symbol not in notation")),
            }
        }
        tokens.push(Token::End);
        return Ok(tokens);
    }
    // pub fn all_terms(rawstr: &str) -> Vec<Term> {
    //     return vec![Term {
    //         symbol: 'p',
    //         value: true,
    //     }];
    // }
    // pub fn main_connective(&self) -> Result<(Connective, usize), &str> {
    //     let mut stack: Vec<char> = vec![];
    //     // println!("{:#?}", self.rawstring.chars());
    //     let mut conn: Result<(Connective, usize), &str> = Err("");

    //     let mut to_iterate_over = self.rawstring.chars().collect::<Vec<char>>();
    //     let last_index = to_iterate_over.len() - 1;

    //     // if to_iterate_over[0] == '(' && to_iterate_over[last_index] == ')' {
    //     //     println!("before mod {:?}", to_iterate_over);
    //     //     to_iterate_over.pop();
    //     //     to_iterate_over = to_iterate_over.split_first_mut().unwrap().1.to_vec();
    //     //     println!("after mod {:?}", to_iterate_over);
    //     // }

    //     for (pos, char) in to_iterate_over.iter().enumerate() {
    //         // println!("PRE stack rn: {:?}", stack);
    //         // println!("char pos: {}", pos);
    //         // println!("PRE char rn: {:?}", char);
    //         if stack.is_empty() {
    //             let connective = Connective::from_symbol(char);
    //             match connective {
    //                 Ok(conn) => {
    //                     if (conn.is_unary() && pos == 0) || (conn.is_binary() && pos != 0) {
    //                         println!("returning main pos @ {}", pos);
    //                         println!("main pos {:#?}", conn);
    //                         return Ok((conn, pos));
    //                     } else {
    //                         return Err("Not a wff (main_connective)");
    //                     }
    //                 }
    //                 Err(_error) => {}
    //             }
    //         };
    //         match char {
    //             '(' => stack.push(*char),
    //             ')' => {
    //                 stack.pop();
    //             }
    //             _ => {}
    //         }
    //         // println!("POST stack rn: {:?}", stack);
    //         // println!("POST char rn: {:?}", char);
    //     }
    //     return conn;
    // }

    // pub fn remove_soft_parenthesis(rawstr: &str) -> &str {
    //     let arr = unicode_split(rawstr);
    //     let output_string = String::from("");
    //     if (arr[0] != "(") {
    //         return rawstr;
    //     }
    //     let mut stack: Vec<&str> = vec!["("];
    //     for (pos, char) in arr.iter().enumerate() {
    //         if pos == 0 {
    //             continue;
    //         }
    //         match *char {
    //             "(" => stack.push(*char),
    //             ")" => {
    //                 stack.pop();
    //             }
    //             _ => {}
    //         }
    //     }
    //     return "";
    // }
    // pub fn remove_first_last(&mut self) -> &mut Self {
    //     let mut arr = unicode_split(&self.rawstring);
    //     arr.pop();
    //     arr = arr.split_first_mut().unwrap().1.to_vec();
    //     // println!("{:#?}", &arr);
    //     self.rawstring = arr.join("");
    //     return self;
    // }
    // pub fn left(&self) -> Formula {
    //     let (_, pos) = self.main_connective().unwrap();

    //     // let sa = &self.rawstring.chars().collect::<Vec<char>>();
    //     // let splt = (&sa[0..pos]).iter().collect::<String>();

    //     let arr = unicode_split(&self.rawstring);
    //     let splt = (&arr[0..pos]).join("");
    //     return Formula::from(splt.as_str());
    // }
    // pub fn right(&self) -> Formula {
    //     let (_, pos) = self.main_connective().unwrap();

    //     // let arr = &self.rawstring.graphemes(true).collect::<Vec<&str>>();
    //     // let splt = (&arr[(pos + 1)..arr.len()]).join("");

    //     let arr = unicode_split(&self.rawstring);
    //     let substring = (&arr[(pos + 1)..arr.len()]).join("");
    //     return Formula::from(substring.as_str());
    // }
}
