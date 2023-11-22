use regex::Regex;
use std::io::{self, BufRead};
use std::fs::File;
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Command {
    START,
    MATCH(String),
    RESPONSE(String),
    UNKNOWN,
    CASE(String),
    DEFAULT,
    END,
}
// 将一行字符串转化为Command枚举类型 
fn parse_line_to_cmd(line: &str) -> Option<Command> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    // 使用正则表达式对传入脚本行进行匹配
    let re = Regex::new(r#"(\w+)\s*(".*")?"#).unwrap();
    let cap = re.captures(line)?;

    // 模式匹配，第一块为脚本命令，第二块为脚本信息
    let cmd = match cap.get(1)?.as_str() {
        "START" => Command::START,
        "MATCH" => Command::MATCH(cap.get(2)?.as_str().trim_matches('"').to_string()),
        "RESPONSE" => Command::RESPONSE(cap.get(2)?.as_str().trim_matches('"').to_string()),
        "CASE" => Command::CASE(cap.get(2)?.as_str().trim_matches('"').to_string()),
        "DEFAULT" => Command::DEFAULT,
        "UNKNOWN" => Command::UNKNOWN,
        "END" => Command::END,
        _ => {
            panic!("bad command {}", line)
        },
    };

    Some(cmd)
}

// 打印Command
pub fn print_command(ref cmd : Command){
    match cmd{
        Command::START => println!("STAET"),
        Command::MATCH(msg) => println!("MATCH: {}", msg),
        Command::RESPONSE(msg)=> println!("RESPONSE: {}", msg),
        Command::UNKNOWN => println!("UNKNOWN"),
        Command::CASE(msg)=> println!("CASE: {}", msg),
        Command::DEFAULT=> println!("DEFAULT"),
        Command::END=> println!("END"),
    }
}

// 打开脚本将脚本中的自然语言转化为Command向量
pub fn parse_file_to_cmds(filename: &str) -> io::Result<Vec<Command>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut commands: Vec<Command> = vec![];
    for line in reader.lines() {
        if let Ok(ln) = line {
            if let Some(cmd) = parse_line_to_cmd(&ln) {
                commands.push(cmd);
            }
        }
    }

    Ok(commands)
}


#[test]
fn test_parse_line_to_cmd() {
    assert_eq!(parse_line_to_cmd("START"), Some(Command::START));
    assert_eq!(parse_line_to_cmd("END"), Some(Command::END));
    assert_eq!(parse_line_to_cmd("UNKNOWN"), Some(Command::UNKNOWN));
    assert_eq!(parse_line_to_cmd("MATCH \"good\""), Some(Command::MATCH("good".to_string())));
    assert_eq!(parse_line_to_cmd("RESPONSE \"hello\""), Some(Command::RESPONSE("hello".to_string())));
    assert_eq!(parse_line_to_cmd("CASE \"hi\""), Some(Command::CASE("hi".to_string())));
    assert_eq!(parse_line_to_cmd("DEFAULT"), Some(Command::DEFAULT));

    assert_eq!(parse_line_to_cmd("CASE"), None);
}

// 测试脚本中未定义的命令
#[test]
#[should_panic]
fn test_bad_parse_line_to_cmd() {
    assert_eq!(parse_line_to_cmd("BADSTART"), Some(Command::START));
}

// 测试从script.txt中读取脚本为command
#[test]
fn test_parse_file_to_cmds() {
    let command_vec = match parse_file_to_cmds("scripts/script.txt"){
        Ok(vec) => vec,
        Err(error) => panic!("bad vec {}", error)
    };
    assert_eq!(command_vec.len(), 22);
    assert_eq!(command_vec[0], Command::START);
    assert_eq!(command_vec[3], Command::MATCH("我的订单状态是什么".to_string()));
    assert_eq!(command_vec[11], Command::RESPONSE("你的配送地址已经更改为100号大街。".to_string()));
    assert_eq!(command_vec[16], Command::RESPONSE("对不起，我无法查询到你提供的订单信息，请检查订单号是否正确。".to_string()));
    assert_eq!(command_vec[21], Command::END);
}

// 测试读取不存在的脚本
#[test]
#[should_panic]
fn test_bad_parse_file_to_cmds() {
    match parse_file_to_cmds("scripts/noscript.txt"){
        Ok(vec) => vec,
        Err(error) => panic!("bad vec {}", error)
    };
}