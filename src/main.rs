#[macro_use]
extern crate clap;
extern crate time;
extern crate filebuffer;

mod tokenizer_loop;
mod tokenizer_peek;

use clap::{Arg, App};
use time::PreciseTime;
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

      
      // Start timer
      let start1 = PreciseTime::now();
      bytes.tokenize();
      let end1 = PreciseTime::now();
      
      let start2 = PreciseTime::now();
      words(bytes);
      let end2 = PreciseTime::now();
      
      println!("{} for tokenizer_peek", start1.to(end1));
      println!("{} for tokenizer_loop", start2.to(end2));

} 

