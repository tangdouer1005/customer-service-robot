
use regex::RegexBuilder;
use std::io::{self, BufRead};
use crate::block::Block;
use crate::block::MatchBlock;
use crate::block::CaseBlock;

 // 判断给定字符串是否符合正则表达式
fn str_match(str1: &str, reg: &str) -> bool{
    let re = RegexBuilder::new(reg.trim()).unicode(true)
    .build()
    .unwrap();
    re.is_match(str1.trim())
}
// 传入脚本对应的Block，运行脚本
pub fn execute(m_block :Block){
    let Block {
        matches,
        unknowing,
    } = m_block;
    let mut current_match: Option<MatchBlock> = None;
    let mut current_case: Option<CaseBlock> = None;
    'outer: loop{
        let stdin = io::stdin();
        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("无法读取行");
        if let Some(case_block) = current_case{
            match case_block.matches{
                Some(match_vec) =>{
                    for match_block in match_vec.iter(){
                        if str_match(line.as_str(), match_block.mtch.as_str()){
                            println!("{}", match_block.response);
                            current_match = Some(match_block.to_owned());
                            current_case = None;
                            continue 'outer;
                        }
                    }
                    println!("{}", unknowing.response);
                    current_match = None;
                    current_case = None;
                    continue 'outer;
                },
                None =>{
                    current_case = None;
                }
            }
        }
        if let Some(match_block) = current_match{
            match match_block.cases{
                Some(case_vec) => {
                    for case_block in case_vec.iter(){
                        if str_match(line.as_str(), case_block.case.as_str()){
                            println!("{}", case_block.response);
                            current_match = None;
                            current_case = Some(case_block.to_owned());
                            continue 'outer;
                        }
                    }
                    println!("{}", match_block.default.unwrap());
                    current_match = None;
                    current_case = None;
                    continue 'outer;
                }
                None => {
                    current_match = None;
                }
            }
        }
        for match_block in matches.iter(){
            if str_match(line.as_str(), match_block.mtch.as_str()){
                println!("{}", match_block.response);
                current_match = Some(match_block.to_owned());
                current_case = None;
                continue 'outer;
            }
        }
        println!("{}", unknowing.response);
        current_match = None;
        current_case = None;
        continue 'outer;
    }
}
// 测试str_match
#[test]
fn test_str_match() {
    assert!(str_match("测试一下吧", ".*测试.*"));
    assert!(!str_match("一下吧", ".*测试.*"));
    assert!(str_match("测啥都行", ".*"));
}
