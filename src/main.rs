#[macro_use]
extern crate clap;
extern crate time;
extern crate filebuffer;

mod tokenizer_loop;
mod tokenizer_peek;

use clap::{Arg, App};
use time::{PreciseTime, Duration};
use tokenizer_loop::*;
use tokenizer_peek::*;

fn main() {
      let matches = App::new("Nlp")
            .version("0.1")
            .arg(Arg::with_name("INPUT")
                  .help("Source file to use")
                  .required(true)
                  .index(1))
            .get_matches();
            
      let file_name: String = value_t!(matches.value_of("INPUT"), String).unwrap();
      let data = filebuffer::FileBuffer::open(file_name.as_str()).unwrap();
      
      let bytes = data.to_vec();

      // Use Tokenizer_loop
      let start = PreciseTime::now();
      let tokens = bytes.tokenize();
      let end = PreciseTime::now();
      let ms = start.to(end).num_milliseconds();
      
      match tokens {
            Ok(s) => println!("Tokenized {} objects in {} ms, {} objects/s", s.len(), ms, s.len() as i64 / ms * 1000 ),
            Err(e) => println!("{:?}", e),
      }
      
      // Use Tokenizer_peek
      let start = PreciseTime::now();
      let tokens = words(bytes);
      let end = PreciseTime::now();
      let ms = start.to(end).num_milliseconds();
      
      match tokens {
            Ok(s) => println!("Tokenized {} objects in {} ms, {} objects/S", s.len(), ms, s.len() as i64 / ms * 1000),
            Err(e) => println!("{:?}", e),
      }
} 

