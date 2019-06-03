#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TopLevel {
    Function(Function),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Block,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub type_: Type,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Empty,
    Expression(Expression),
    Let(String, Option<Type>, Option<Expression>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    Variable(String),
    Block(Block),
    If(IfExpression),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IfExpression {
    pub condition: Box<Expression>,
    pub then: Block,
    pub else_: Option<Box<ElseExpression>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ElseExpression {
    If(IfExpression),
    Block(Block),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Named(String),
}
