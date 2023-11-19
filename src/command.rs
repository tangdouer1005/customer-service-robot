use regex::Regex;
use std::io::{self, BufRead};
use std::fs::File;
#[derive(Clone)]
pub enum Command {
    START,
    Match(String),
    Answer(String),
    Unknown(String),
    Case(String),
    Defaul(String),
    End,
}

fn parse_line_to_cmd(line: &str) -> Option<Command> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }
    
    let re = Regex::new(r#"(\w+)\s*(".*")?"#).unwrap();

    let cap = re.captures(line)?;

    let cmd = match cap.get(1)?.as_str() {
        "BEGIN" => Command::START,
        "MATCH" => Command::Match(cap.get(2)?.as_str().trim_matches('"').to_string()),
        "RESPONSE" => Command::Answer(cap.get(2)?.as_str().trim_matches('"').to_string()),
        "CASE" => Command::Case(cap.get(2)?.as_str().trim_matches('"').to_string()),
        "DEFAULT" => Command::Defaul(cap.get(2)?.as_str().trim_matches('"').to_string()),
        "UNKNOWN" => Command::Unknown(cap.get(2)?.as_str().trim_matches('"').to_string()),
        "END" => Command::End,
        _ => return None,
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
        Command::Match(msg) => println!("MATCH: {}", msg),
        Command::Answer(msg)=> println!("Answer: {}", msg),
        Command::Unknown(msg)=> println!("Unknown: {}", msg),
        Command::Case(msg)=> println!("Case: {}", msg),
        Command::Defaul(msg)=> println!("MATCH: {}", msg),
        Command::End=> println!("MATCH"),
    }
}