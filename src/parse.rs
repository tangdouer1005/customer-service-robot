

use std::fs::File;
use std::io::Read;

use crate::block::parse_commands_to_blocks;
use crate::command::Command;
use crate::command::parse_file_to_cmds;
use crate::command::print_command;
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
    // let mut file = match File::open(&script_path){
    //     Ok(file) => file,
    //     Err(s) => panic!("fail to open {}", s),
    // };

    // let mut content = String:: new();

    // match file.read_to_string(& mut content) {
    //     Ok(__) => {
    //         println!("read from {}, content: \n{}", script_path, content)
    //     }
    //     Err(err) => panic!("fail to read from {}, err: {}", script_path, err),    
    // };
    
    let commands = match parse_file_to_cmds(script_path.as_str()){
        Ok(commands) => commands,
        Err(msg) => {
            panic!("fail to trans str to command, err : {}", msg);
        }
    };
    // for cmd in commands.clone().into_iter(){
    //     print_command(cmd);
    // }

    // let  blocks = match parse_commands_to_blocks(commands){
    //     Ok(m_blocks) => m_blocks,
    //     Err(msg) => panic!("fail to trans command to blocks, err : {}", msg),
    // };


    Err(ParseError::new("todo".to_string()))
}