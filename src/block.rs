use crate::command::Command;
use crate::command::parse_file_to_cmds;
use crate::command::print_command;
#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Block {
    pub matches: Vec<MatchBlock>,
    pub unknowing: UnknowingBlock,
}
#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct MatchBlock {
    pub mtch: String,
    pub response: String,
    pub cases: Option<Vec<CaseBlock>>,
    pub default: Option<String>,
}
#[allow(dead_code)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct CaseBlock {
    pub case: String,
    pub response: String,
    pub matches: Option<Vec<MatchBlock>>,
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct UnknowingBlock {
    pub response: String,
}
#[derive(Debug)]
#[derive(Clone, Copy)]
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

//将command向量转化为block
pub fn parse_commands_to_blocks(commands: Vec<Command>) -> Result<Block, &'static str> {
    // 主block的match向量
    let mut matches: Vec<MatchBlock> = Vec::new();
    // 进入block子块时将原current_match入栈保存
    let mut matches_stack: Vec<MatchBlock> = Vec::new();
    // 当前matchblock
    let mut current_match: Option<MatchBlock> = None;
    // 当前caseblock
    let mut current_case: Option<CaseBlock> = None;
    // 主块unknowblock
    let mut unknowing: Option<UnknowingBlock> = None;
    // 当前状态
    let mut state = Status::INITIAL;
    // 块嵌套层数
    let mut nest_level = 0;

    for command in commands {
        //print_command(command.clone());
        match command {
            Command::START => {
                match state{
                    Status::INITIAL =>{
                        state = Status::START;
                        nest_level += 1;
                    }
                    // 在Status::CASEANSWER遇到Command::START，表明进入嵌套子块，将current_case放入current_match，将current_match入栈
                    Status::CASEANSWER =>{
                        match current_match{
                            Some(ref mut match_block) => {
                                match match_block.cases{
                                    Some(ref mut case_vec) => case_vec.push(current_case.unwrap().clone()),
                                    None => match_block.cases = Some(vec![current_case.unwrap().clone()]),
                                }
                                matches_stack.push(match_block.clone());
                            },
                            None => return Err("进入嵌套子块时current_match为空")
                        }
                        current_match = None;
                        current_case = None;
                        state = Status::START;
                        nest_level += 1;
                    }
                    __ => {
                        return Err("接收到Command::START时处于错误的状态");
                    }
                }
            }
            Command::MATCH(mtch) => {
                match state{
                    Status::START => state = Status::MATCH,
                    // 接收到Command::MATCH时，如果处于子块则将match_block放入matches_stack的栈顶match，若处于主块中则放入matches
                    Status::MATCHANSWER | Status::DEFAULTANSWER => {
                        state = Status::MATCH;
                        if nest_level > 1{
                            let mut last_stack = matches_stack.pop();
                            match last_stack{
                                Some(ref mut match_block) => {
                                    match match_block.cases{
                                        Some(ref mut case_vec) =>{
                                            let mut last_case = case_vec.pop().unwrap();
                                            match last_case.matches{
                                                Some(ref mut match_vec) =>{
                                                    match_vec.push(current_match.unwrap());
                                                }
                                                None => {
                                                    last_case.matches = Some(vec![current_match.unwrap()]);
                                                }
                                            }
                                            current_match= None;
                                            case_vec.push(last_case);
                                        }
                                        None => return Err("接受Command::MATCH处于子块,栈顶matchblock不存在cases成员")
                                    }
                                }
                                None => return Err("接受Command::MATCH处于子块,栈顶matchblock不存在")
                            }
                            matches_stack.push(last_stack.unwrap());
                        }else if nest_level == 1{
                            match current_match{
                                Some(ref match_block) => {
                                    matches.push(match_block.clone())
                                }
                                None => return Err("接受Command::MATCH处于主块,current_match不存在")
                            }
                        }
                    }
                    __ => return Err("接受Command::MATCH处于错误的状态")
                }
                //初始化current_match
                current_match = Some(MatchBlock {
                    mtch,
                    response: String::new(),
                    cases: None,
                    default: None,
                });
            }
            Command::CASE(case) => {
                match state{
                    Status::MATCHANSWER => state = Status::CASE,
                    // 将上一个caseblock放入current_case
                    Status::CASEANSWER => {
                        state = Status::CASE;
                        match current_case{
                            Some(case_block) =>{
                                match current_match{
                                    Some(ref mut m_match) => {
                                        match m_match.cases{
                                            Some(ref mut vec) => vec.push(case_block.clone()),
                                            None => m_match.cases = Some(vec![case_block.clone()])
                                        }
                                    }
                                    None => return Err("接收Command::CASE处于Status::CASEANSWER, current_match为空")
                                }
                            }
                            None => return Err("接收Command::CASE处于Status::CASEANSWER, current_case为空")
                        }
                        current_case = None;
                        
                    } 
                    __ => return Err("bad status try to trans to CASE")
                }
                // 初始化current_case
                current_case = Some(CaseBlock {
                    case,
                    response: String::new(),
                    matches: None,
                })
            }
            Command::RESPONSE(response) => {
                // 接收到RESPONSE根据状态补充对应的RESPONSE
                match state {
                    Status::MATCH => {
                        state = Status::MATCHANSWER;
                        match current_match{
                            Some(ref mut match_block) => match_block.response = response,
                            None => return Err("接收到Command::RESPONSE处于Status::MATCH,current_match为空")
                        }
                    }
                    Status::CASE => {
                        state = Status::CASEANSWER;
                        match current_case{
                            Some(ref mut case_block) =>{
                                case_block.response = response;
                            }
                            None => return Err("接收到Command::RESPONSE处于Status::CASE,current_case为空")
                        }
                    }
                    Status::DEFAULT => {
                        state = Status::DEFAULTANSWER;
                        match current_match{
                            Some(ref mut match_block) => match_block.default = Some(response),
                            None => return Err("接收到Command::RESPONSE处于Status::DEFAULT,current_match为空")
                        }
                    }
                    Status::UNKNOWN => {
                        state = Status::UNKNOWNANSWER;
                        match unknowing{
                            Some(ref mut unknow_block) => unknow_block.response = response,
                            None => return Err("接收到Command::RESPONSE处于Status::UNKNOWN,unknowing为空")
                        }
                    }
                    __ => {
                        return Err("接收到Command::RESPONSE处于错误的状态");
                    }
                }
            }
            // 接收Command::END处于主块直接返回，处于子块matches_stack栈顶元素cases添加current_match
            Command::END => {
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
                        __ => return Err("主块中接收到Command::END处于错误状态")
                    }
                }else if nest_level > 1{
                    match state {
                        Status::MATCHANSWER | Status::DEFAULTANSWER => {
                            let mut tem_match = match matches_stack.pop(){
                                Some(match_block) => match_block,
                                None => panic!("Some(match_block) => match_block,")
                            };
                            let mut clone_case_vec = tem_match.clone().cases.unwrap();
                            current_case = clone_case_vec.pop();
                            tem_match.cases = Some(clone_case_vec);
                            
                            if let Some(ref mut case_block) = current_case{
                                match case_block.matches{
                                    Some(ref mut match_vec) => match_vec.push(current_match.unwrap()),
                                    None => case_block.matches = Some(vec![current_match.unwrap()])
                                }

                            }
                            current_match = Some(tem_match);
                            
                            nest_level -= 1;
                            state = Status::CASEANSWER;
                        }
                        __ => return Err("子块中接收到Command::END处于错误状态")
                    }
                }else{
                    return Err("nest_level错误");
                }
            }
            // 接收Command::UNKNOWN
            Command::UNKNOWN => {
                match state {
                    Status::MATCHANSWER | Status::DEFAULTANSWER => {
                        match current_match{
                            Some(match_block) => matches.push(match_block),
                            None => panic!("Command::UNKNOWN")
                        }
                        current_match = None;
                        state = Status::UNKNOWN;
                    }
                    __ => return Err("nest_level == 1, end state wrong")
                }
                if unknowing.is_some(){
                    return Err("repeated unknowing");
                }
                let unknow_block = UnknowingBlock {
                    response: String::new(),
                };
                unknowing = Some(unknow_block);
            }
            //接收Command::DEFAULT命令
            Command::DEFAULT => {
                match state{
                    Status::CASEANSWER =>{
                        state = Status::DEFAULT;
                        match current_match{
                            Some(ref mut match_block) => {
                                match current_case{
                                    Some(case_block) =>{
                                        match match_block.cases{
                                            Some(ref mut vec) => vec.push(case_block.clone()),
                                            None => match_block.cases = Some(vec![case_block.clone()])
                                        }
                                    }
                                    None => panic!("current_case is None")
                                }
                                current_case = None;
                            }
                            None => panic!("current_match is None")
                        }
                    }
                    __ => panic!("bad cmd on DEFAULT")
                }
            }
        }
    }
    Err("未正常退出parse_commands_to_blocks")
}
#[test]
fn test_parse_commands_to_blocks() {
    let cmd_vec = match parse_file_to_cmds("script.txt") {
        Ok(vec) => vec,
        Err(error) => panic!("error: {}", error),
    };
    let m_block = match parse_commands_to_blocks(cmd_vec) {
        Ok(block) => block,
        Err(error) => panic!("error: {}", error)
    };
    let m_str = format!("{:?}", m_block.clone());
    let answer_str = r#"Block { matches: [MatchBlock { mtch: "我需要帮助", response: "当然，我很乐意帮助你。你遇到什么问题了呢？", cases: None, default: None }, MatchBlock { mtch: "我的订单状态是什么", response: "让我为你查询。请你提供下订单号。", cases: Some([CaseBlock { case: "我的订单号是12345", response: "已查询到该地址，请问需要什么服务？", matches: Some([MatchBlock { mtch: "我想改变配送地址", response: "好的，你想更改为哪个地址？", cases: Some([CaseBlock { case: "我想改为100号大街", response: "你的配送地址已经更改为100号大街。", matches: None }]), default: Some("对不起，这个地址我们无法配送。") }]) }]), default: Some("对不起，我无法查询到你提供的订单信息，请检查订单号是否正确。") }, MatchBlock { mtch: "我无法登录我的账户", response: "抱歉给您带来不便。你是否忘记了密码，或被告知你的账户被冻结了?", cases: None, default: None }], unknowing: UnknowingBlock { response: "抱歉，我不明白你的问题。能否请你再详细描述一下？" } }"#;
    assert_eq!(m_str, answer_str);
}

#[test]
#[should_panic (expected = "error: 接受Command::MATCH处于错误的状态")]
fn test_bad_parse_commands_to_blocks() {
    let cmd_vec = match parse_file_to_cmds("bad_script.txt") {
        Ok(vec) => vec,
        Err(error) => panic!("error: {}", error),
    };
    let m_block = match parse_commands_to_blocks(cmd_vec) {
        Ok(block) => block,
        Err(error) => panic!("error: {}", error)
    };
}
