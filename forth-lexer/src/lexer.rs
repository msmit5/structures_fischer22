use std::{fs::File, io::prelude::Read};//io::prelude::*};

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Strn,
    Nmbr,
    Word
}

#[derive(Debug)]
pub enum LexerError {
    FileError(std::io::Error),
}

#[derive(Debug)]
pub struct Token{
    token_type: TokenType,
    value: String,
    reference_count: u32, //unsigned because we don't need negative counts
}

pub struct Lexer {
    file_path: String,      // I am using a string because it is easier to implement in comparison
                            // to the path type. I could put in a box, but I've never done that.
    file_hnd: File,
    token_list: std::collections::HashMap<String, Token>,
    state: LexerState,
    cur_char: [u8; 1]
}

#[derive(Debug, PartialEq)]
enum LexerState {
    StartState,
    SlashPending,
    ParenPending,
    AcquiringSlash,
    AcquiringParen,
    AcquiringToken,
    AcquiringString,
    Done,
}


//-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=//
impl Lexer {
    pub fn new(p: String) -> Result<Lexer, std::io::Error>{
        // Open File. Clone prevents use after move
        let fi = File::open(p.clone())?;

        // Return the new struct
        Ok(Lexer{
            file_path: p,
            file_hnd: fi,
            token_list: std::collections::HashMap::new(),
            state: LexerState::StartState,
            cur_char: [255_u8]
        })
    }


    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=//
    pub fn do_lex(&mut self) {
        self.state = LexerState::StartState; 

        let mut prev = 0x00_u8;
        self.cur_char[0] = 0x00_u8;
        //loop{ 
        while self.file_hnd.read(&mut self.cur_char[..]).unwrap() > 0{
            //println!("{}:{};", self.cur_char[0], self.cur_char[0] as char);
            
            match self.state {
                LexerState::StartState      => self.do_start(),
                LexerState::SlashPending    => self.slash_pending(),
                LexerState::ParenPending    => self.paren_pending(),
                LexerState::AcquiringSlash  => self.acquiring_slash(),
                LexerState::AcquiringParen  => self.acquiring_paren(),
                LexerState::AcquiringToken  => self.acquiring_token(prev),
                LexerState::AcquiringString => self.acquiring_string(prev),
                LexerState::Done            => return,
            }
            prev= self.cur_char[0];
        }
    }


    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=//
    fn slash_pending(&mut self) {
        // read until <space>
        if self.cur_char[0] == 0x20_u8 {
            self.state = LexerState::AcquiringSlash;
        } else {
            self.state = LexerState::AcquiringToken;
        }
    }

    
    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=//
    fn paren_pending(&mut self) {
        // read until <space>
        if self.cur_char[0] == 0x20_u8 {
            self.state = LexerState::AcquiringParen;
        } else {
            self.state = LexerState::AcquiringToken;
        }
    }


    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=//
    fn acquiring_slash(&mut self) {
        // read until <CR>
        while self.cur_char[0] != 0x0A_u8 {
            match self.file_hnd.read(&mut self.cur_char[..]) {
                Ok(a)  => {
                    if a < 1 {
                        self.state = LexerState::Done;
                        return;
                    }
                },
                Err(a) => {
                    eprintln!("{:?}", a);
                    panic!("Fatal error!");
                }
            }
        }
    }


    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=//
    fn acquiring_paren(&mut self) {
        // read until )
        while self.cur_char[0] != 0x29_u8 {
            match self.file_hnd.read(&mut self.cur_char[..]) {
                Ok(a)  => {
                    if a < 1 {
                        self.state = LexerState::Done;
                        return;
                    }
                },
                Err(a) => {
                    eprintln!("{:?}", a);
                    panic!("AAAAAAAAHHHHHHHHHH!"); // After all, we are panicking ;)
                }
            }
        }
        self.state = LexerState::StartState;
    }


    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=//
    fn acquiring_token(&mut self, already_read: u8) {
        let mut _tkn = String::new();
        if already_read != 0x00_u8{
            // Prepare to append the characters that were already read.
            let _tmp = [already_read, self.cur_char[0]];
            _tkn += std::str::from_utf8(&_tmp[..]).expect("invalid character");
        }
        
        // Read until <space> and <CR>
        while self.cur_char[0] != 0x20_u8 && self.cur_char[0] != 0x0a_u8  {
            match self.file_hnd.read(&mut self.cur_char[..]) {
                Ok(0) => { // 0 bytes read if eof;
                    self.do_token(_tkn, TokenType::Word);
                    self.state = LexerState::Done;
                    return;
                }
                Ok(1) => {
                    _tkn += std::str::from_utf8(&self.cur_char[..]).expect("Invalid utf8 char");
                }
                Err(e) => {
                    eprintln!("ERROR!\n{:?}",e);
                    panic!("Encountered an error!");
                }
                _ => {
                    panic!("Honestly, how did this read more than a byte when the buffer is one byte?");
                }
            } // match
        } // while
       
        _tkn = String::from(_tkn.trim());
        if _tkn.len() == 0 {
            return;
        } else if _tkn.eq(&String::from(".\"")) {
            self.do_token(_tkn, TokenType::Word);
            self.state = LexerState::AcquiringString;
            return;
        } else if _tkn.is_numeric() {
            self.do_token(_tkn, TokenType::Nmbr);
            self.state = LexerState::StartState;
            return;
        } else {
            self.do_token(_tkn, TokenType::Word);
            self.state = LexerState::StartState;
            return;
        }
    } // end acquiring_token


    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=//
    fn acquiring_string(&mut self, already_read: u8){
        let mut _tkn = String::new();
        // don't add a null character to the beginning of a string
        if already_read != 0x00_u8{
            // Prepare to append the characters that were already read.
            let _tmp = [already_read, self.cur_char[0]];
            _tkn += std::str::from_utf8(&_tmp[..]).expect("invalid character");
        }
        
        while self.cur_char[0] != 0x22_u8 {
            match self.file_hnd.read(&mut self.cur_char[..]) {
                Ok(0) => {
                    self.do_token(_tkn, TokenType::Word);
                    self.state = LexerState::Done;
                    return;
                }
                Ok(1) => {
                    // Ignore closing quote
                    if self.cur_char[0] != 0x022 {
                        _tkn += std::str::from_utf8(&self.cur_char[..]).expect("Invalid utf8 char");
                    }
                }
                Err(e) =>{
                    eprintln!("ERROR\n{:?}", e);
                    panic!("Encountered an error!");
                }
                _ => {
                    panic!("How did this read more than one byte when the buffer is one byte!!!");
                }
            }
        }

        _tkn = String::from(_tkn.trim());
        if _tkn.len() == 0 {
            return;
        } else {
            self.do_token(_tkn, TokenType::Strn)
        }
        self.state = LexerState::StartState;
    } // end acquiring_string


    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=//
    fn do_start(&mut self) {
        match self.cur_char[0] {
            // I am forced to use hex because of rust using utf8 instead of 
            0x20_u8 => { /* <space> */ }, // do nothing
            0x0A_u8 => { /* <CR> */ }, // Do nothing
            0x55_u8 => { // "\"
                self.state = LexerState::SlashPending;
            },
            0x28_u8 => { // "("
                self.state = LexerState::ParenPending;
            },
            0x22_u8 => { // '"'
                self.state = LexerState::AcquiringString;
            }
            0x00_u8 => {
                // only accessed on first entry.
                //let _ = self.file_hnd.read(&mut self.cur_char[..]);
                self.do_start();
            }
            _ => {
                self.state = LexerState::AcquiringToken;
            }
        }
    } // end do_start


    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=//
    fn do_token(&mut self, s: String, t: TokenType) {
        if self.token_list.contains_key(&s) {
            let _old = self.token_list.get_mut(&s).unwrap();
            _old.reference_count+=1;
        } else {
            self.token_list.insert(s.clone(), Token{token_type: t, value: s, reference_count: 1});
        }
    }


    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=//
    pub fn print(&self) {
        println!("{:#?}", self.token_list)
    }
} // impl lexer


    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=//
// Here, I am extending the String "object" and allowing me to check if the entire string is
// fully numeric.
trait IsNumeric {
    fn is_numeric(&self) -> bool;
}

impl IsNumeric for String {
    fn is_numeric(& self) -> bool {
        for c in self.chars() {
            if !c.is_numeric() {
                return false;
            }
        }
        return true;
    }
}
