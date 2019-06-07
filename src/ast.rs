#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TopLevel {
    Function(Function),
    ModFile(ModFile),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModFile {
    pub mod_: String,
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
    Let(Let),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    Variable(Variable),
    Block(Block),
    If(If),
    While(While),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct If {
    pub condition: Box<Expression>,
    pub then: Block,
    pub else_: Option<Box<Else>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Else {
    If(If),
    Block(Block),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct While {
    pub condition: Box<Expression>,
    pub block: Block,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Let {
    pub name: String,
    pub type_: Option<Type>,
    pub value: Option<Expression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Named(NamedType),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamedType {
    pub name: String,
}
