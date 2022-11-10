use std::{fs::File,path::Path, io::{prelude::*, BufReader, Error}, collections::hash_map};


pub enum TokenType {
    Strn,
    Nmbr,
    Word
}

#[derive(Debug)]
pub enum LexerError {
    FileError(std::io::Error),
}

pub struct Token{
    token_type: TokenType,
    value: String,
    reference_count: u32, //unsigned because we don't need negative counts
}

pub struct Lexer {
    file_path: String,      // I am using a string because it is easier to implement in comparison
                            // to the path type. I could put in a box, but I've never done that.
    //file_data: Vec<String>,
    file_hnd: File,
    //token_list: Vec<Token>,
    token_list: std::collections::HashMap<String, Token>,
}

enum LexerState {
    StartState,
    SlashPending,
    AcquiringSlash,
    AcquiringParen,
    AcquiringToken,
    AcquiringString,
}

impl Lexer {
    pub fn new(p: String) -> Result<Lexer, LexerError>{
        // Open File. Clone prevents use after move
        let fi = File::open(p.clone()).expect("File not found!");
        // Create BufReader for reading the contents of the file
        //let buf = BufReader::new(fi);

        

        // Return the new struct
        Ok(Lexer{
            file_path: p,
            file_hnd: fi,
            //token_list: Vec::new()
            token_list: std::collections::HashMap::new()
        })
    }

    pub fn do_lex(&mut self) {
        let mut token_state: LexerState = LexerState::StartState;
        
        let mut c1: [u8; 1] = [0];
        let mut c2: u8 = 0;

        while self.file_hnd.read(&mut c1[..]).unwrap() > 0{
            println!("{}:{};", c1[0], c1[0] as char);
            
            match token_state{
                LexerState::StartState => eprintln!("Start State!"),
                LexerState::SlashPending => eprintln!("SlashPending"),
                LexerState::AcquiringSlash => eprintln!("Acquiring"),
                LexerState::AcquiringParen => eprintln!("AcquiringParen"),
                LexerState::AcquiringToken => eprintln!("AcquiringToken"),
                LexerState::AcquiringString => eprint!("Acquiring String"),
                
            }

            c2 = c1[0];
        }

    }

    pub fn doStart(&mut self) {

    }

    pub fn doToken(&mut self) {

    }

    // we are not editing anything here, so we can just leave it as a normal self reference
    pub fn print(&self) {

    }
}

