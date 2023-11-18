use std::fs::File;
use std::io::Read;

use crate::command::Command;
use crate::script::Script;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ParseError{
    detail : String
}
impl ParseError{
    fn new(detail: String) -> ParseError{
        ParseError{detail}
    }
}

#[allow(dead_code)]
pub fn script_parse(script_path: String) -> Result<Script, ParseError>{
    let mut file = match File::open(&script_path){
        Ok(file) => file,
        Err(s) => panic!("fail to open {}", s),
    };

    let mut content = String:: new();

    match file.read_to_string(& mut content) {
        Ok(__) => {
            println!("read from {}, content: \n{}", script_path, content)
        }
        Err(err) => panic!("fail to read from {}, err: {}", script_path, err),    
    };
    
    let commands : Vec<Command>;
    


    Err(ParseError::new("todo".to_string()))
}