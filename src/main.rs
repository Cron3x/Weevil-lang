mod lexer;
mod parser;
mod type_system;

use lexer::*;
use crate::parser::{Parser,StmtKind};

fn main() {
    let file_path = "./program.wivl";

    let lexer = Lexer::new(file_path);
    let mut parser = Parser::new(lexer);
    let containers = parser.start_parse();

    println!("------");

    for cont in containers {
        if cont.is_ok() {
            println!("{:?}",cont.ok().unwrap());
        }
        else {
            let err = cont.err().unwrap();
            panic!("{}",err);
        }

    }

    /*
       let mut token = lexer.next_token();
       while token.is_some() {
       match token {
       Some(t) =>  println!("{:?}", t),
       None    =>  {println!("EOF"); break}
       }
       token = lexer.next_token();
       }
       */
}
