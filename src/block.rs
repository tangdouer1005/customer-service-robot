use crate::command::Command;
#[allow(dead_code)]
pub struct Block {
    matches: Vec<MatchBlock>,
    unknowing: UnknowingBlock,
}
#[allow(dead_code)]
#[derive(Clone)]
pub struct MatchBlock {
    mtch: String,
    response: String,
    cases: Option<Vec<CaseBlock>>,
    default: Option<String>,
}
#[allow(dead_code)]
#[derive(Clone)]
pub struct CaseBlock {
    case: String,
    response: String,
    matches: Option<Vec<MatchBlock>>,
}

#[allow(dead_code)]
pub struct UnknowingBlock {
    unknowing: String,
    response: String,
}
// pub fn parse_commands_to_blocks(commands: Vec<Command>) -> Option<Block>{
//     None
// }

enum Status{
    INITIAL,
    START,
    MATCH,
    MATCHANSWER,
    CASE,
    CASEANSWER,
    DEFAULT,
    DEFAULTANSWER,
    UNKNOWN,
    UNKNOWNANSWER,
    END
}

pub fn parse_commands_to_blocks(commands: Vec<Command>) -> Result<Block, &'static str> {
    let mut matches: Vec<MatchBlock> = Vec::new();
    let mut current_match: Option<MatchBlock> = None;
    let mut current_case: Option<CaseBlock> = None;
    let mut unknowing: Option<UnknowingBlock> = None;
    let mut state = Status::INITIAL;
    let mut nest_level = 0;

    for command in commands {
        match command {
            Command::START => {
                match state{
                    Status::INITIAL => state = Status::START,
                    __ => return Err("bad status try to trans to START")
                }
            }
            Command::Match(mtch) => {

                match state{
                    Status::START => state = Status::MATCH,
                    Status::MATCHANSWER => state = Status::MATCH,
                    Status::DEFAULTANSWER => state = Status::MATCH,
                    __ => return Err("bad status try to trans to MATCH")
                }
                match current_match{
                    Some(ref match_block) => {
                        matches.push(match_block.clone())
                    }
                    None => ()
                }
                current_match = Some(MatchBlock {
                    mtch,
                    response: String::new(),
                    cases: None,
                    default: None,
                });
            }
            Command::Case(case) => {
                match state{
                    Status::MATCHANSWER => state = Status::MATCH,
                    Status::CASEANSWER => state = Status::MATCH, 
                    __ => return Err("bad status try to trans to MATCH")
                }
                current_case = Some(CaseBlock {
                    case,
                    response: String::new(),
                    matches: None,
                })
            }
            Command::Answer(response) => {
                match state {
                    Status::MATCH => {
                        state = Status::MATCHANSWER;
                        if let Some(match_block) = &mut current_match {
                            match_block.response = response;
                        }
                    }
                    Status::CASE => {
                        state = Status::CASEANSWER;
                        match current_case{
                            Some(ref mut case_block) =>{
                                case_block.response = response;
                                match current_match{
                                    Some(ref mut m_match) => {
                                        match m_match.cases{
                                            Some(ref mut vec) => vec.push(case_block.clone()),
                                            None => m_match.cases = Some(vec![case_block.clone()])
                                        }
                                    }
                                    None => panic!("current_match is None")
                                }
                            }
                            None => panic!("current_case is None")
                        }
                        current_case = None;
                    }
                    Status::DEFAULT => {
                        state = Status::DEFAULTANSWER;
                        if let Some(match_block) = &mut current_match {
                            match_block.default = Some(response);
                        }
                    }
                    Status::UNKNOWN => {
                        state = Status::UNKNOWNANSWER;
                        match unknowing{
                            Some(ref mut unknow_block) => unknow_block.response = response,
                            None => panic!("answer to no noknowing")
                        }
                    }
                    __ => return Err("Unexpected Response command")
                }
            }
            Command::End => {
                if nest_level == 1{
                    match state {
                        Status::MATCHANSWER | Status::UNKNOWNANSWER => {
                            if matches.is_empty() || unknowing.is_none(){
                                return Err("Invalid command sequence");
                            }
                        
                            let block = Block {
                                matches:matches.iter_mut().map(|ref mut x| x.clone()).collect(),
                                unknowing:unknowing.unwrap()
                            };
                        
                            return Ok(block)
                        }
                        __ => return Err("nest_level == 1, end state wrong")
                    }
                }else if nest_level > 1{
                    match state {
                        Status::MATCHANSWER | Status::DEFAULTANSWER => {
                            nest_level -= 1;
                            state = Status::CASEANSWER;
                        }
                        __ => return Err("nest_level > 1, end state wrong")
                    }
                }else{
                    return Err("nest_level < 1, end state wrong");
                }

            }
            Command::Unknown(unknowing_des) => {
                match state {
                    Status::MATCHANSWER | Status::DEFAULTANSWER => {
                        state = Status::UNKNOWNANSWER;
                    }
                    __ => return Err("nest_level == 1, end state wrong")
                }
                if unknowing.is_some(){
                    return Err("repeated unknowing");
                }
                let unknow_block = UnknowingBlock {
                    unknowing: unknowing_des,
                    response: String::new(),
                };
                unknowing = Some(unknow_block);
            }
            Command::Defaul(default) => {
                match state {
                    Status::CASEANSWER => {
                        state = Status::DEFAULT;
                        match current_match{
                            Some(ref mut match_block) => {
                                match_block.default = Some(default);
                            }
                            None => return Err("Default command no match"),
                        }
                    }
                    __ => return Err("Default command bad state")
                }
            }
        }
    }
    Err("Unexpected Default command")
}