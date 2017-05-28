use bitflags::*;
use std::borrow::Cow;
use encoding::{Encoding, DecoderTrap};
use encoding::all::UTF_8;
use tokenizer_loop::*;

pub fn to_number(token: &Token) -> i32 {

      let values: [i32; 26]= [1,2,3,4,5,6,7,6,5,4,3,2,1,1,2,3,4,5,6,7,6,5,4,3,2,1];
      let mut sum: i32 = 0;
      let mut pos: usize = 0;
      
      if token.flags == ALPHA {
      
            for byte in token.bytes.clone() { 
                  pos = if byte > 96 {(byte - 97) as usize} else {(byte - 65) as usize};
                  sum += values[pos];
            }
      }
      sum
}
