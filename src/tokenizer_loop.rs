use bitflags::*;
use std::borrow::Cow;

#[derive(Debug, PartialEq)]
pub struct Token {
      index: usize,
      flags: Flags,
      data: String //Vec<u8>
}

bitflags! {
      #[derive(Default)]
      pub struct Flags: u64 {
            const CONTROL     = 0b0000000000000001;
            const SPECIAL     = 0b0000000000000010;
            const NUMBER      = 0b0000000000000100;
            const ALPHA       = 0b0000000000001000;
            const LATIN       = 0b0000000000010000;
            const STOP        = 0b0000000000100000;
            const FULL_STOP   = 0b0000000001000000;
            const QUOTE       = 0b0000000010000000;
            const BRACKET     = 0b0000000100000000;
            const HYPHEN      = 0b0000001000000000;
      }
}

impl Flags {
    pub fn clear(&mut self) {
        self.bits = 0;
    }
}

pub fn words(bytes: Vec<u8>) -> Result<Vec<Token>, &'static Vec<u8>> {

      const MAX_BYTES: usize        = 8;
      let mut i: usize              = 0;
      let mut buffer: Vec<u8>       = Vec::with_capacity(MAX_BYTES);
      let mut tokens: Vec<Token>    = vec![];
      let mut parse_flags: Flags    = Default::default();
      
      for byte in bytes {
            
            // Increase buffer if needed
            if buffer.len() == MAX_BYTES { 
                  buffer.reserve(MAX_BYTES); 
            };
                        
            match byte {
                  32 | 10 | 13 => {
                        if buffer.len() == 0 { continue; };
                        i += 1;
                        
                        let token = Token {
                              index: i,
                              flags: parse_flags,
                              data: as_string(buffer.clone())
                        };
                        println!("{:?}", token );
                        
                        tokens.push( token );
                        buffer.clear();
                        parse_flags.clear();
                        },
                  65 ... 90 | 97 ... 122 => { 
                        buffer.push(byte); 
                        parse_flags = parse_flags | ALPHA;
                        },
                  33 ... 47 | 58 ... 64 | 91 ... 96 | 123 ... 126 | 160 ... 191 => { 
                        buffer.push(byte); 
                        parse_flags = parse_flags | parse_special(byte);
                        },
                  48 ... 57 => { 
                        buffer.push(byte); 
                        parse_flags = parse_flags | NUMBER;
                        },
                  127 ... 159 | 1 ... 31 => { 
                        buffer.push(byte); 
                        parse_flags = parse_flags | CONTROL;
                        },
                  192 ... 255 => { 
                        buffer.push(byte); 
                        parse_flags = parse_flags | LATIN;
                        },
                  _ => { 
                        buffer.push(byte); 
                        }
            }
      }
      
      // Get the last token from buffer
      if buffer.len() > 0 { 
            let token = Token {
                  index: i + 1,
                  flags: parse_flags,
                  data: as_string(buffer.clone())
            };
            println!("{:?}", token );
      }
      Ok( tokens )
}

/*
 * Parse Special characters and return Flags
 *
 */
pub fn parse_special(byte: u8) -> Flags {
      match byte {
            44 | 58 | 59 => { 
                  STOP 
                  },
            45 => {
                  HYPHEN
                  },
            33 | 46 | 63 => { 
                  FULL_STOP 
                  },
            34 => { 
                  QUOTE 
                  },
            40 | 41 | 60 | 62 | 91 | 93 | 123 | 125 => { 
                  BRACKET 
                  },
            _ => {
                  SPECIAL
                  }
      }
}

/*
 * Transform a code point in the range U+0080-U+00FF into decoded UTF-8
 * U80_FF( 209 ) -> [195, 145]
 *
 * u8 slice must know the size statically, else we have to use lifetimes
 */
pub fn U80_FF(byte: u8) -> [u8; 2] {
      let slice = if byte < 192 {[194u8, byte]} else {[195u8, byte - 64u8]};
      slice
}

/*
 * Transform a vector of bytes into a String
 *
 */
pub fn as_string(input: Vec<u8>) -> String {
      let mut buf: Vec<u8> = Vec::with_capacity(input.len() * 2);
      
      for byte in input {
            if byte > 127 {
                  buf.extend_from_slice( &U80_FF(byte).to_owned() );
            } else {
                  buf.push(byte);
            }
      }
      let result = unsafe {
            String::from_utf8_unchecked(buf)
      };
      
      result
}
