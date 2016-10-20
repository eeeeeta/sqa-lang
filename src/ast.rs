use chrono::Duration;
#[derive(Debug)]
pub enum Expr {
    StringLiteral(String),
    Identifier(String),
    Duration(Duration),
    VolumeDecibels(f64)
}
#[derive(Debug)]
pub enum QuoteStmt {
    Key(String),
    KeyValue(String, Box<Expr>)
}
#[derive(Debug)]
pub struct Command {
    pub label: Option<String>,
    pub after_prev: bool,
    pub pre: Duration,
    pub verb: String,
    pub direct_obj: Option<Expr>,
    pub ind_obj: Option<Expr>,
    pub arguments: Vec<QuoteStmt>
}
#[derive(Debug)]
pub enum Stmt {
    CueBlock(String, Vec<Stmt>),
    ThenBlock(Vec<Stmt>),
    Command(Command)
}
pub type Program = Vec<Stmt>;
