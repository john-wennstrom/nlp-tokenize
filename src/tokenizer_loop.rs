#[derive(Debug, PartialEq)]
pub struct Token {
      data: Vec<u8>,
      length: usize,
      index: usize,
}

pub fn words(bytes: Vec<u8>) -> Result<Vec<Token>, &'static Vec<u8>> {

      //let mut hyphen: bool = false;
      let mut i: usize = 0;
      const MAX_BYTES: usize = 128;
      let mut buffer: Vec<u8> = Vec::with_capacity(MAX_BYTES);
      let mut tokens: Vec<Token> = vec![];
      
      for byte in bytes {
            i = i + 1;
            // Handle UTF-8 block 0-127
            if byte <= 127 {
                  match byte {
                        // If byte is a space or newline
                        32u8 | 10u8 | 13u8 => {
                              // If buffer has no length, continue, else print 
                              // it out and clear the buffer for next token
                              let length = buffer.len();
                              if length == 0 { continue; };
                              
                              /* create string
                              let string: String = match String::from_utf8(buffer.clone()) {
                                    Ok(s) => s,
                                    Err(err) => {
                                          //println!("Err: {:?}\n\n\n", err);
                                          buffer.clear();
                                          continue;
                                    },
                              };*/
                              
                              // create token
                              let token = Token {
                                    data: buffer.clone(),
                                    length: length,
                                    index: i
                              };
                              tokens.push( token );
                              //println!("{:?}", token );
                              buffer.clear();
                        }
                        
                        // If byte is a hyphen-minus
                        45u8 => {
                              
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
      
      if buffer.len() > 0 { 
            let token = Token {
                  data: buffer.clone(),
                  length: buffer.len(),
                  index: i
            };
            tokens.push( token );
            //println!("{:?}", String::from_utf8(buffer).unwrap());
      }
      
      Ok( tokens )
}

