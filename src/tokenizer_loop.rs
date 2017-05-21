use bitflags::*;

#[derive(Debug, PartialEq)]
pub struct Token {
      index: usize,
      flags: Flags,
      data: Vec<u8>
}

bitflags! {
      #[derive(Default)]
      pub struct Flags: u32 {
            const CONTROL     = 0b00000001;
            const SPECIAL     = 0b00000010;
            const NUMBER      = 0b00000100;
            const ALPHA       = 0b00001000;
            const LATIN       = 0b00010000;
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
            
            if buffer.len() == MAX_BYTES { 
                  buffer.reserve(MAX_BYTES); 
            };
                        
            match byte {
                  32u8 | 10u8 | 13u8 => {
                        if buffer.len() == 0 { continue; };
                        i += 1;
                        
                        let token = Token {
                              index: i,
                              flags: parse_flags,
                              data: buffer.clone()
                        };
                        println!("{:?}", token );
                        
                        tokens.push( token );
                        buffer.clear();
                        parse_flags.clear();
                        
                  },
                  1 ... 31    => { parse_flags = parse_flags | CONTROL;   buffer.push(byte);},
                  32 ... 47   => { parse_flags = parse_flags | SPECIAL;   buffer.push(byte);},
                  48 ... 57   => { parse_flags = parse_flags | NUMBER;    buffer.push(byte);},
                  58 ... 64   => { parse_flags = parse_flags | SPECIAL;   buffer.push(byte);},
                  65 ... 90   => { parse_flags = parse_flags | ALPHA;     buffer.push(byte);},
                  91 ... 96   => { parse_flags = parse_flags | SPECIAL;   buffer.push(byte);},
                  97 ... 122  => { parse_flags = parse_flags | ALPHA;     buffer.push(byte);},
                  123 ... 126 => { parse_flags = parse_flags | SPECIAL;   buffer.push(byte);},
                  127 ... 159 => { parse_flags = parse_flags | CONTROL;   buffer.push(byte);},
                  160 ... 191 => { parse_flags = parse_flags | SPECIAL;   buffer.push(byte);},
                  192 ... 255 => { parse_flags = parse_flags | LATIN;     buffer.push(byte);},
                  _           => { 
                        buffer.push(byte);
                  }
            }
  
            /* Handle UTF-8 block 128-255 Basic Latin
            else {
                  let le = if byte < 192 {194} else {195};
                  let be = if byte < 192 {byte} else {byte - 64};
                  
                  buffer.push(le);
                  buffer.push(be);       
            }*/
      }
      
      /*if buffer.len() > 0 { 
            let token = Token {
                  data: buffer.clone(),
                  length: buffer.len(),
                  index: i
            };
            tokens.push( token );
            //println!("{:?}", String::from_utf8(buffer).unwrap());
      }*/
      
      Ok( tokens )
}

