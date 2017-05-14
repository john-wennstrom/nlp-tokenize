use std::iter::{Iterator, Peekable};
use std::slice::Iter;

#[derive(Debug, PartialEq)]
pub struct Token {
      object: Kind,
      content: Vec<u8>,
      //length: usize,
      //index: usize,
}

#[derive(Debug, PartialEq)]
pub enum Kind {
      Stop,             // Alphanumerical strings with additional . , : ; ? !
      Alpha,            // Pure alpha string
      Other,            // Unidentified elements
      Hyphen,           // Alpha string ending with hyphen-minus -
      Bracket,          // Alphanumeric strings containing one or more < [ ( " ' `
      Numeric,          // Pure numeric strings 123
      NonAlphaNum       // Strings with only | ¦ § _ ~ ^
      
}

pub trait Tokenizer {
      fn tokenize(&self) -> Result<Vec<Token>, &'static Vec<u8>>;
}

impl Tokenizer for Vec<u8> {

      fn tokenize(&self) -> Result<Vec<Token>, &'static Vec<u8>> {
      
            // Turn source into peekable iterator over vector of bytes
            let mut it = self.iter().peekable();
            let mut tokens: Vec<Token> = vec![];
            
            loop {
                  match it.peek() {
                        Some(_) => {
                                    //println!("{:?}", &byte); 
                                    let object: Vec<u8> = consume_while(&mut it, |a| not_eow(a))
                                          .into_iter()
                                          .collect();    
                                    
                                    // Parse kind
                                    let kind = match object_type(object.clone()) {
                                          Some(kind) => kind,
                                          _ => Kind::Other
                                    };
                                    
                                    // Create token
                                    let token = Token {object: kind, content: object};
                                    
                                    println!("{:?}", token);
                                    tokens.push( token );
                                    it.next().unwrap();
                              
                        },
                        None => break
                  }
            }
            Ok(tokens)
      }
}


fn not_eow(byte: u8) -> bool {
      match byte {
            32u8 | 10u8 | 13u8 => {false}
            _ => {true}
      }
}

fn object_type(mut object: Vec<u8>) -> Option<Kind> {
      let mut result: Kind;
      
      match object.pop() {
            Some(byte) => {
                  match byte {
                        44u8 | 46u8 | 33u8 | 59u8 | 58u8 | 63u8 => result = Kind::Stop,
                        45u8 => result = Kind::Hyphen,
                        _ => result = Kind::Other
                  }
            },
            None => result = Kind::Other
      }
      
      Some(result)
}


fn consume_while<F>(it: &mut Peekable<Iter<u8>>, condition: F) -> Vec<u8>
      where F : Fn(u8) -> bool {

      let mut v: Vec<u8> = vec![];

    
      loop {
            match it.peek() {
                  Some(&&byte) => {
                        if condition(byte) {
                              it.next().unwrap();
                              v.push(byte);
                        } else {
                              break;
                        }
                  }
                  None => break
            }
      }
      
      v
}
