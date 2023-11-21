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

pub fn parse_line_to_cmd(line: &str) -> Option<Command> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }
    
    let re = Regex::new(r#"(\w+)\s*(".*")?"#).unwrap();

    let cap = re.captures(line)?;

    let cmd = match cap.get(1)?.as_str() {
        "START" => Command::START,
        "MATCH" => Command::MATCH(cap.get(2)?.as_str().trim_matches('"').to_string()),
        "RESPONSE" => Command::RESPONSE(cap.get(2)?.as_str().trim_matches('"').to_string()),
        "CASE" => Command::CASE(cap.get(2)?.as_str().trim_matches('"').to_string()),
        "DEFAULT" => Command::DEFAULT,
        "UNKNOWN" => Command::UNKNOWN,
        "END" => Command::END,
        _ => {
            panic!("bad command 请检查你的脚本语法 {}", line)
        },
    };

    Some(cmd)
}

pub fn parse_file_to_cmds(filename: &str) -> io::Result<Vec<Command>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    //这里可以进行一个脚本的语法检验

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

#[test]
#[should_panic]
fn test_bad_parse_line_to_cmd() {
    assert_eq!(parse_line_to_cmd("BADSTART"), Some(Command::START));
}

#[test]
fn test_parse_file_to_cmds() {
    let command_vec = match parse_file_to_cmds("script.txt"){
        Ok(vec) => vec,
        Err(error) => panic!("bad vec {}", error)
    };
    // for i in command_vec.iter(){
    //     print_command(i.clone());
    // }
    assert_eq!(command_vec.len(), 22);

    assert_eq!(command_vec[0], Command::START);
    assert_eq!(command_vec[3], Command::MATCH("我的订单状态是什么".to_string()));
    assert_eq!(command_vec[11], Command::RESPONSE("你的配送地址已经更改为100号大街。".to_string()));
    assert_eq!(command_vec[16], Command::RESPONSE("对不起，我无法查询到你提供的订单信息，请检查订单号是否正确。".to_string()));
    assert_eq!(command_vec[21], Command::END);
}

#[test]
#[should_panic]
fn test_bad_parse_file_to_cmds() {
    match parse_file_to_cmds("noscript.txt"){
        Ok(vec) => vec,
        Err(error) => panic!("bad vec {}", error)
    };
}