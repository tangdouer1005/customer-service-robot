use crate::command::Command;

#[allow(dead_code)]
pub struct Script{
    commands: Vec<Command>
}

#[allow(dead_code)]
impl Script{
    pub fn new(commands: Vec<Command>) -> Script{
        Script { commands }
    }

    pub fn execute(&self){
        
    }
}