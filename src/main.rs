#[macro_use]
extern crate clap;
extern crate time;
extern crate filebuffer;

use clap::{Arg, App};
use time::PreciseTime;

fn main() {
      let matches = App::new("Nlp")
            .version("0.1")
            .arg(Arg::with_name("INPUT")
                  .help("Input file to use")
                  .required(true)
                  .index(1))
            .get_matches();
            
      let file_name: String = value_t!(matches.value_of("INPUT"), String).unwrap();
      let data = filebuffer::FileBuffer::open(file_name.as_str()).unwrap();
      
      let bytes = data.to_vec(); // [74, 111, 104, 110, 10, 65]
      
      const MAX_BYTES: usize = 128;
      let mut buffer: Vec<u8> = Vec::with_capacity(MAX_BYTES);
      
      // Start timer
      let start = PreciseTime::now();
      
      for byte in bytes {
            
            // Handle UTF-8 block 0-127
            if byte <= 127 {
                  match byte {
                        // If byte is a whitespace
                        32u8 | 10u8 | 13u8 => {
                              // If buffer has no length, continue, else print 
                              // it out and clear the buffer for next token
                              if buffer.len() == 0 { continue; };
                              
                              let token: String;
                              token = match String::from_utf8(buffer.clone()) {
                                    Ok(string) => string,
                                    Err(err) => {
                                          
                                          println!("Err: {:?}\n\n\n", err);
                                          buffer.clear();
                                          continue;
                                    },
                              };
                              // 62% time saving for not printing. 
                              // Tokenizes text file 5Mb/s
                              //println!("{}", token);
                              buffer.clear();
                        }
                        _ => {
                              // If we ran out of memory, allocate more, then 
                              // push the byte to buffer
                              if buffer.len() == MAX_BYTES { 
                                    buffer.reserve(MAX_BYTES); 
                              };
                              buffer.push(byte);
                              
                        }
                  }
            } 
            // Handle UTF-8 block 128-255 Basic Latin
            else {
                  let le = if byte < 192 {194} else {195};
                  let be = if byte < 192 {byte} else {byte - 64};
                  
                  buffer.push(le);
                  buffer.push(be);       
            }
      } 
      
      // If there is no whitespace in the end of the source, and if buffer has 
      // some data for us, it will be our last token.
      if buffer.len() > 0 { 
            println!("{} | ", String::from_utf8(buffer).unwrap());
      }
      
      let end = PreciseTime::now();
      
      println!("{} seconds.", start.to(end));

} 

