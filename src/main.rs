mod command;
mod script;
mod block;
use block::parse_commands_to_blocks;
use command::parse_file_to_cmds;
use script::execute;
use std::io::{self};

fn main() {
    let mut script_path = String::new();
    println!("请输入你的脚本路径：");
    let _ = io::stdin().read_line(&mut script_path);


    // 将脚本中的自然语言转化为command向量
    let commands = match parse_file_to_cmds(script_path.trim()){
        Ok(commands) => commands,
        Err(msg) => {
            panic!("fail to trans str to command, err : {}", msg);
        }
    };

    // 将command向量转化为block结构体
    let  block = match parse_commands_to_blocks(commands){
        Ok(block) => block,
        Err(msg) => panic!("fail to trans command to blocks, err : {}", msg),
    };
    println!("脚本编译完毕，运行脚本成功");

    // 根据block结构体运行脚本
    execute(block);

}
