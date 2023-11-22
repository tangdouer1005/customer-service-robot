
mod command;
mod script;
mod block;
use block::parse_commands_to_blocks;
use command::parse_file_to_cmds;
use script::execute;
use std::io::{self, BufRead};

fn main() {
    let mut script_path = String::new();
    println!("请输入你的脚本名：");
    io::stdin().read_line(&mut script_path);

    let commands = match parse_file_to_cmds(script_path.trim()){
        Ok(commands) => commands,
        Err(msg) => {
            panic!("fail to trans str to command, err : {}", msg);
        }
    };

    let  blocks = match parse_commands_to_blocks(commands){
        Ok(m_blocks) => m_blocks,
        Err(msg) => panic!("fail to trans command to blocks, err : {}", msg),
    };
    println!("{:?}", blocks);

    execute(blocks);
    
}
