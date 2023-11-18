#[allow(dead_code)]
pub struct Block {
    matches: Vec<MatchBlock>,
    unknowing: UnknowingBlock,
}
#[allow(dead_code)]
pub struct MatchBlock {
    mtch: String,
    response: String,
    cases: CaseBlock,
    default: Option<String>,
}
#[allow(dead_code)]
pub struct CaseBlock {
    case: String,
    response: String,
    matches: Vec<MatchBlock>,
}

#[allow(dead_code)]
pub struct UnknowingBlock {
    unknowing: String,
    response: String,
}