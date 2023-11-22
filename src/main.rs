
mod command;
mod script;
mod block;
use block::parse_commands_to_blocks;
use command::parse_file_to_cmds;
use script::execute;

fn main() {
    let script_path = "script_1.txt";
    let commands = match parse_file_to_cmds(script_path){
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
