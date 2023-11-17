#[allow(dead_code)]
pub struct Script{
    commands: Vec<String>
}

#[allow(dead_code)]
impl Script{
    pub fn new(commands: Vec<String>) -> Script{
        Script { commands }
    }

    pub fn execute(&self){
        
    }
}