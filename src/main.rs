use crate::parse::script_parse;


mod command;
mod parse;
mod script;

fn main() {
    let script_path = "script.txt".to_string();

    #[allow(unused_variables)]
    let script = script_parse(script_path);


    println!("Hello, world!");
}
