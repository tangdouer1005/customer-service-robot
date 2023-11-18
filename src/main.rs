use crate::parse::script_parse;


mod command;
mod parse;
mod script;
mod block;

fn main() {
    let script_path = "script.txt".to_string();

    #[allow(unused_variables)]
    let script = match script_parse(script_path){
        Ok(script) => script,
        Err(err) => panic!("fail to script_parse, err: {:?}", err),
    };

    
}
