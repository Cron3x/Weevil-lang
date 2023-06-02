use std::fmt;

use crate::lexer::TokenKind;

#[derive(Debug, Clone, Copy)]
pub enum TypeSystem{
    Void,
    Integer,
    String,
    Float,
}
impl fmt::Display for TypeSystem{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self)
    }
}

impl TypeSystem {
    pub fn into_wivl_type(value: String) -> Result<TypeSystem, String>{ 
        let real_type = match Some(value.as_str()) {
            Some("int")   => Ok(TypeSystem::Integer),
            Some("void")  => Ok(TypeSystem::Void),
            Some("str")   => Ok(TypeSystem::String),
            Some("float") => Ok(TypeSystem::Float),
            _ => Err(value),
        };
        return real_type
    }
    pub fn matches_kind_type(token_kind: &TokenKind, required_type: &TypeSystem) -> bool{
        let real_type = match token_kind{
            TokenKind::Number(_) => Self::Integer,
            TokenKind::String(_) => Self::String,
            _ => {
                panic!("{:?} is not implementet (yet)", token_kind);
            }
        };
        let same = match (&real_type, &required_type) {
             (Self::Integer, Self::Integer) => true,
             (Self::Void, Self::Void)       => true,
             (Self::String, Self::String)   => true,
             (Self::Float, Self::Float)     => true,
             _ => false
        };
        same
    }
}

