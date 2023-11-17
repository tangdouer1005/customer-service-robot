#[allow(dead_code)]
pub enum Command{
    Begin,
    Match(String),
    Response(String),
    Unknow (String),
    Case(String),
    Default(String),
    End,
}
