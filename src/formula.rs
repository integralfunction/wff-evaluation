use crate::Token;
use crate::term::Term;

#[derive(Debug)]
pub struct Formula {
    pub rawstring: String,
    pub terms: Vec<Term>,
}

impl Formula {
    pub fn new(rawstr: String) -> Self {
        let x = rawstr.trim().split(' ').collect::<Vec<&str>>().join("");
        return Formula {
            rawstring: x.clone(),
            terms: Formula::get_all_terms_initially(x),
        };
    }

    pub fn split_by_symbols(source: String, pat: Vec<&str>) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let arr = source.chars().collect::<Vec<char>>();
        // println!("len: {:?}", arr.len());
        // println!("arr: {:?}", arr);
        let mut index = 0;
        while index < arr.len() {
            let character = arr[index];
            let mut pad = 0;

            // TODO: Instead of matching [a-z] as terms, we should have an option to make
            // it only a subset, for example [p-t]
            if character.is_alphabetic() {
                result.push(character.to_string());
                index += 1;
                continue;
            }

            loop {
                let c = &arr[index..index + pad + 1];
                let d = c.iter().collect::<String>();
                // println!("c: {:?}", c);
                if !pat.contains(&d.as_str()) {
                    pad += 1;
                } else {
                    result.push(d);
                    index += 1 + pad;
                    break;
                }
            }
        }
        // println!("split_by_symbols {:#?}", result);
        return result;
    }

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
        // let x = Self::split_by_symbols(rawstring, vec!["(", ")", "&&", "!", "||"]);
        // for thing in x {
        //     match thing {
        //         ch if ch.chars().all(char::is_alphabetic) => {
        //             let y = ch.chars().next().unwrap();
        //             let x = all_terms.iter().find(|&term| term.symbol == y);
        //             match x {
        //                 None => {
        //                     all_terms.push(Term {
        //                         symbol: y,
        //                         value: true,
        //                     });
        //                 }
        //                 Some(_) => {}
        //             }
        //         }
        //         _ => {}
        //     }
        // }
        // dbg!(&all_terms);
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

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = vec![];
        let x = Self::split_by_symbols(self.rawstring.clone(), vec!["(", ")", "&&", "!", "||"]);
        for thing in x {
            // println!("{}", char);
            match thing.as_str() {
                "(" => tokens.push(Token::LeftParen),
                ")" => tokens.push(Token::RightParen),
                "&&" => tokens.push(Token::And),
                "||" => tokens.push(Token::Or),
                "!" => tokens.push(Token::Not),
                ch if ch.chars().all(char::is_alphabetic) => {
                    tokens.push(Token::Term(ch.chars().next().unwrap()));
                }
                _ => return Err(String::from("Symbol not in notation")),
            }
        }
        tokens.push(Token::End);
        return Ok(tokens);
    }
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
}
