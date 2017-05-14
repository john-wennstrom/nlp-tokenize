#[macro_use]
extern crate clap;
extern crate time;
extern crate filebuffer;

mod tokenz;

use clap::{Arg, App};
use time::PreciseTime;
use tokenz::*;

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
      let start = PreciseTime::now();
      
      // Main iteration
      //
      
      //let buffer = words(bytes);
      
      bytes.tokenize();
      
      /* Last bytes in the buffer we turn into token
      if buffer.len() > 0 { 
            println!("{:?}", String::from_utf8(buffer).unwrap());
      }*/
      // Stop timer
      let end = PreciseTime::now();
      
      println!("{} seconds.", start.to(end));

} 

