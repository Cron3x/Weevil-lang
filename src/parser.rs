use core::panic;
use std::{any::Any, collections::HashMap};
use crate::type_system::*;
use crate::lexer::*;

#[derive(Debug)]
pub enum ContainerKind{
    Enum{
        name: String,
        body: Vec<String>
    },
    Struct{},
    Func {
        name: String,
        return_type: TypeSystem,
        body: Vec<StmtKind>,
    }
}

#[derive(Debug)]
pub enum StmtKind {
    FuncallStmt{
        name: String,
        args: Vec<Box<dyn Any>>
    },
    RetStmt{ expr: Box<dyn Any> },
    VarStmt{
        name: String,
        value: Vec<Box<dyn Any>>,
        kind: TypeSystem,
    }
}

pub struct Parser{
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self{
        Self{ lexer }
    }
    pub fn start_parse(&mut self) -> Vec<Result<ContainerKind, String>>{
        let containers:Vec<Result<ContainerKind, String>> = vec![];
        loop{
            let token = self.next_token();
            if self.lexer.is_empty() {break}
            //dbg!(&token);
            let cont = match token.kind{
                TokenKind::Function(v) => self.parse_function(v),
                TokenKind::Struct(v)   => self.parse_struct(v),
                TokenKind::Enum(v)     => self.parse_enum(v),
                _ => {panic!("{}: Expected Container Name not {:?}", self.lexer.loc(),token.kind)}
            };

        }
        return containers;
    }

    pub fn parse_enum(&mut self, name: String)   -> Result<ContainerKind, String>{
        let body = self.parse_enum_body();
        match body {
            Ok(b) => Ok(ContainerKind::Enum { name, body: b}),
            Err(e) => panic!("{}",e)
        }
    }
    pub fn parse_enum_body(&mut self) -> Result<Vec<String>, String>{
        let token = self.next_token();
        match token.kind {
            TokenKind::OParen => {}
            _ => panic!("{}: expected '(' and not {:?} ", self.lexer.loc(), token.kind)
        }

        let mut block = vec![];

        loop {
            let token = self.next_token();
            //dbg!(&token);

            match token.kind{
                TokenKind::CParen => break,
                TokenKind::Name(name) => {
                    block.push(name)
                },
                _ => return Err(format!("{}: Closing Paren is missing ({:?})", self.lexer.loc(), &token.kind)),
            };
        }
        Ok(block)
    }

    pub fn parse_struct(&mut self, name: String) -> Result<ContainerKind, String>{
        Ok(ContainerKind::Struct {  })
    }

    pub fn parse_function(&mut self, name: String) -> Result<ContainerKind, String>{
        self.parse_arguments();

        let has_colon = match self.next_token().kind {
            TokenKind::Colon => true,
            _ => {false}
        };

        let token = self.next_token();
        //dbg!(&token);

        let ret_type = if has_colon {
            let value = match token.kind{
                TokenKind::Name(v) => v,
                _ => panic!("{}: Please provide a type after the Colon", self.lexer.loc()),
            };
            value
        } else {
            "void".to_string()
        };

        let ret_type = match TypeSystem::into_wivl_type(ret_type){
            Ok(v) => v,
            Err(e)=> panic!("{}: `{e}´ is not a valid type", self.lexer.loc())
        };

        let body = match self.parse_body(&ret_type){
            Ok(v) => v,
            Err(e) => panic!("{}: {}", self.lexer.loc(), e)
        };
        Ok(ContainerKind::Func{ name, body, return_type: ret_type })
    }

    fn parse_body(&mut self, req_type: &TypeSystem) -> Result<Vec<StmtKind>,String>{
        let token = self.next_token();
        //dbg!(&token);

        let has_oparen = match token.kind{
            TokenKind::OParen => { true },
            _ => {let e = format!("{}: Opening Paren is missing ({:?})", self.lexer.loc(), &token.kind) ; return Err(e)},
        };

        let mut block = vec![];

        loop {
            let token = self.next_token();
            //dbg!(&token);

            match token.kind{
                TokenKind::CParen => break,
                TokenKind::Name(name) => {
                    match name.as_str() {
                        "let" => {
                            let name = match self.next_token().kind{
                                TokenKind::Name(v) => v,
                                tk => panic!("{}: expected Variable Name got: {:?}",self.lexer.loc() ,tk),
                            };
                            let has_col = match self.next_token().kind {
                                TokenKind::Colon => true,
                                _ => panic!("{}: please add type anotation",self.lexer.loc())
                            };
                            let kind = match self.next_token().kind {
                                TokenKind::Name(v) => {
                                    let x = match TypeSystem::into_wivl_type(v){
                                        Ok(v) => v,
                                        Err(e)=> panic!("{}",e)
                                    };
                                    x
                                },
                                _ => panic!("{}: please add type anotation",self.lexer.loc())
                            };
                            let has_equal = match self.next_token().kind {
                                TokenKind::Equals => true,
                                _ => panic!("{}: to assign a value to a variable pls use '='",self.lexer.loc())
                            };

                            let mut value:Vec<Box<dyn Any>> = vec![];
                            loop {
                                if self.peek_newline() {
                                    break
                                }

                                match self.next_token().kind {
                                    TokenKind::Name(v)   => value.push(Box::new(v)),
                                    TokenKind::String(v) => value.push(Box::new(v)),
                                    TokenKind::Number(v) => value.push(Box::new(v)),
                                    _ => panic!("{}: expected value of variable", self.lexer.loc()),
                                }
                            }
                            block.push(StmtKind::VarStmt { name, value, kind }) 
                        },
                        _ => todo!()
                    }
                },
                TokenKind::Function(name) => {
                    println!("found fn:  {}", name);

                    let mut args: Vec<Box<dyn Any>> = vec![];
                    let mut arg: (Box<dyn Any>,i32) = (Box::new(1),0);

                    loop {
                        if self.peek_newline() {
                            break
                        }

                        let arg_token = self.next_token();
                        //TODO: 

                        match arg_token.kind{
                            TokenKind::Number(v) => { args.push(Box::new(v))},
                            TokenKind::String(v) => { args.push(Box::new(v))},
                            TokenKind::Name(v)   => { args.push(Box::new(format!("§{{{v}}}")))}
                            TokenKind::Coma      => {  }
                            _ => {panic!("{}: {:?}, not implemented (yet)", self.lexer.loc(), arg_token.kind)},
                        };
                    }

                    block.push(StmtKind::FuncallStmt{name, args});

                    //impl jumping to other funktions
                }
                TokenKind::Return => {
                    let ret_token = self.next_token();
                    //dbg!(&ret_token);
                    //dbg!(&req_type);
                    if TypeSystem::matches_kind_type(&ret_token.kind, &req_type){

                        let ret_value: Box<dyn Any> = match ret_token.kind{
                            TokenKind::Number(v) => Box::new(v),
                            TokenKind::String(v) => Box::new(v),
                            _ => todo!("{}: {:?}, not implemented (yet)", self.lexer.loc(), ret_token.kind),
                        };
                        block.push(StmtKind::RetStmt{expr: ret_value})
                    }else {
                        panic!("{}: return type: `{:?}` and returned type: `{:?}` aren't the same", self.lexer.loc(), &ret_token.kind, &req_type)
                    }

                },
                TokenKind::Number(value) => {println!("found num: {}", value) },
                TokenKind::String(value) => {println!("found str: {}", value) },
                _ => return Err(format!("{}: Closing Paren is missing ({:?})", self.lexer.loc(), &token.kind)),
            };
        }
        Ok(block)
    }

    fn parse_arguments(&mut self) -> HashMap<String, TypeSystem> {
        let has_ospiky = match self.next_token().kind{
            TokenKind::OSpiky => true,
            _ => false 
        };

        let mut arg_list:HashMap<String, TypeSystem> = HashMap::new();

        if has_ospiky {
            let mut n_hash:(String, TypeSystem) = ("".to_string(), TypeSystem::Void);
            loop {
                let token = self.next_token();
                //dbg!(&token);
                match token.kind{
                    TokenKind::CSpiky  => break,
                    TokenKind::Name(v) => {n_hash.0 = v},
                    TokenKind::Colon   => {
                        //dbg!(&n_hash);
                        if n_hash.0.is_empty() {
                            panic!("{}: ´name´ of argument ´:´ ´type´ of argument", self.lexer.loc())
                        }
                    }
                    TokenKind::Name(v) => {n_hash.1 = TypeSystem::into_wivl_type(v).expect("type not supported + Handle me th write way: ")},
                    TokenKind::Coma    => {
                        arg_list.insert(n_hash.0.to_owned(), n_hash.1.to_owned());
                    }
                    _ => panic!("{}: {:?} should not be here", self.lexer.loc(), token.kind)
                }
            }
        } else {
            arg_list.insert("".to_string(), TypeSystem::Void);
        }
        return arg_list;
    }

    fn next_token(&mut self) -> Token{
        match self.lexer.next_token(){
            Ok(token) => return token,
            Err(e) => panic!("{}:{}",self.lexer.loc(),e),
        }
    }
    fn peek_newline(&mut self) -> bool{
        match self.lexer.peek_newline(){
            Ok(v) => return v,
            Err(e) => panic!("{}:{}",self.lexer.loc(),e),
        }
    }
}
