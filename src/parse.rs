use crate::script::Script;

#[allow(dead_code)]
pub struct ParseError{
    detail : String
}

impl ParseError{
    fn new(detail: String) -> ParseError{
        ParseError{detail}
    }
}

#[allow(dead_code)]
pub fn script_parse(src_str: String) -> Result<Script, ParseError>{
    
    Err(ParseError::new("todo".to_string()))
}