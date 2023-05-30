use core::fmt;
use std::fmt::{Display, Formatter};

struct FileInfo {
    path: String
}

pub struct Lexer{
    content: Vec<char>,
    cur:     usize,
    bol:     usize,
    row:     usize,
    file_info: FileInfo,
}

impl Lexer{
    pub fn new(file_path: &str) -> Self{
        let file_content = std::fs::read(file_path).unwrap();
        //TODO: parse strings first, so it doesn't replace \r\n in strings
        let strng = String::from_utf8(file_content).unwrap();
        let content = strng.chars().collect();

        Self{content, cur:0, bol:0, row:0, file_info: FileInfo{path:file_path.to_string()}}

    }

    pub fn peek_newline(&mut self) -> Result<bool, String>{
        if self.not_empty(){
            let mut local_cur = self.cur;
            while self.not_empty() && self.content[local_cur].is_ascii_whitespace() &&  self.content[local_cur] != '\n' {
                local_cur += 1;
            }

            let ch = self.content[local_cur];
            return Ok(ch as char == '\n')
        }
        Err("lexer content is empty".to_string())
    }

    pub fn next_token(&mut self) -> Result<Token, String>{
        self.trim_left();
        self.handle_comment();
       
        if self.is_empty() { return Err("lexer content buffer is empty".to_string()) }

        let curr_loc:Loc = self.loc();
        let first_char = self.content[self.cur];
        
        if first_char.is_alphabetic(){
            let index = self.cur;

            while self.not_empty() && self.content[self.cur].is_alphanumeric(){
                self.chop_char();
            }
            if self.content[self.cur] == '!' {
                self.chop_char(); 
                let text = String::from_iter(&self.content)[index..self.cur].to_string();
                return Ok(Token::new(TokenKind::Function( text ), curr_loc))
            }

            let text = String::from_iter(&self.content)[index..self.cur].to_string();
            return Ok(Token::new(TokenKind::Name( text ), curr_loc))
        }

        if first_char == '"'{
            self.chop_char();
            let start = self.cur;
            while self.not_empty() && self.content[self.cur] != '"'{
                self.chop_char();
            }

            if self.not_empty() {
                let text = String::from_iter(&self.content)[start..self.cur].to_string();
                self.chop_char();
                return Ok(Token::new(TokenKind::String( text ), curr_loc));
            }

            let e = format!("{:?}: ERROR: unclosed string literal", curr_loc);
            return Err(e);
        }


        let kind:Option<TokenKind> = match first_char {
            '<' => Some(TokenKind::OSpiky),
            '>' => Some(TokenKind::CSpiky),
            '(' => Some(TokenKind::OParen),
            ')' => Some(TokenKind::CParen),
            ':' => Some(TokenKind::Colon),
            ',' => Some(TokenKind::Coma),
            '=' => {
                self.chop_char();
                match self.content[self.cur] {
                    '>' => {
                        Some(TokenKind::Return)
                    },
                    _   => {eprintln!("not implemented yet"); None}
                }
            }
             _  => { None },
        };
        if kind.is_some(){
            self.chop_char();
            return Ok(Token::new(kind.unwrap(), curr_loc));
        }

        if first_char.is_numeric(){
            let start = self.cur;
            while self.not_empty() && self.content[self.cur].is_numeric() {
                self.chop_char()
            }
            
            match String::from_iter(&self.content)[start..self.cur].to_string().parse::<i32>() {
                Ok(value) => return Ok(Token::new(TokenKind::Number(value), curr_loc)),
                Err(err)  => return Err(err.to_string())
            };
        }
        unreachable!("UNKNOWN char: {}", first_char)
    }

    fn trim_left(&mut self){
        while self.not_empty() && self.content[self.cur].is_ascii_whitespace() {
            self.chop_char();
        }
    }

    fn chop_char(&mut self){
        if self.not_empty(){
            let ch = self.content[self.cur];

            self.cur +=1;
            if ch as char == '\n'{
                self.bol = self.cur;
                self.row += 1;
            }
        }
    }

    fn drop_line(&mut self){
        while self.not_empty() && self.content[self.cur] != '\n' {
            self.chop_char();
        }
        if self.not_empty(){
            self.chop_char()
        }
    }

    fn not_empty(&self) -> bool{
        self.cur < self.content.len()
    }

    fn is_empty(&self) -> bool {
        !self.not_empty()
    }

    fn handle_comment(&mut self){
        while self.not_empty() && self.content[self.cur] == ';' && self.cur+1 < self.content.len() && self.content[self.cur+1] == ';'{
            self.drop_line();
            self.trim_left();
        }
    }
    pub fn loc(&self) -> Loc{
        Loc{path: self.file_info.path.to_string(), row: self.row, column: self.cur-self.bol}
    }
}

#[derive(Debug, Clone)]
pub enum TokenKind{
    Name(String),
    Function(String),
    Number(i32),
    String(String),
    OParen,
    CParen,
    OSpiky,
    CSpiky,
    Colon,
    Coma,
    Return,
}

#[derive(Debug)]
pub struct Loc{
    pub path: String,
    pub row: usize, 
    pub column: usize
}
impl fmt::Display for Loc{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"path: {}, row: {}, col:{}", self.path, self.row+1, self.column+1) 
    }
}


#[derive(Debug)]
pub struct Token{
    pub kind: TokenKind,
    pub loc: Loc,
}

impl Token{
    pub fn new(kind: TokenKind, loc: Loc) -> Self{
        Self{kind, loc}
    }
}


/*
 /---------------------------------------------------\
||                Weevil-ang Syntax                  || 
 \---------------------------------------------------/

;; Comment

need<"request">

entry!(
    puts! "Hello Wolrd"
    ret 0
)


tf!<num:i32> i32 (
    ret num + 9
)
*/
