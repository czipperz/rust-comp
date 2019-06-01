#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TopLevel {
    Function(Function),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Empty,
    Expression(Expression),
    Let(String, Option<Type>, Option<Expression>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    Variable(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Named(String),
}
