#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TopLevel<'a> {
    Function(Function<'a>),
    ModFile(ModFile<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModFile<'a> {
    pub mod_: &'a str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function<'a> {
    pub name: &'a str,
    pub parameters: Vec<Parameter<'a>>,
    pub body: Block<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter<'a> {
    pub name: &'a str,
    pub type_: Type<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement<'a> {
    Empty,
    Expression(Expression<'a>),
    Let(Let<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression<'a> {
    Variable(Variable<'a>),
    Paren(Box<Expression<'a>>),
    Block(Block<'a>),
    If(If<'a>),
    While(While<'a>),
    Binary(Binary<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable<'a> {
    pub name: &'a str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct If<'a> {
    pub condition: Box<Expression<'a>>,
    pub then: Block<'a>,
    pub else_: Option<Box<Else<'a>>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Else<'a> {
    If(If<'a>),
    Block(Block<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct While<'a> {
    pub condition: Box<Expression<'a>>,
    pub block: Block<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Binary<'a> {
    pub left: Box<Expression<'a>>,
    pub op: BinOp,
    pub right: Box<Expression<'a>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinOp {
    Plus,
    Minus,
    Times,
    DividedBy,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Let<'a> {
    pub name: &'a str,
    pub type_: Option<Type<'a>>,
    pub value: Option<Expression<'a>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block<'a> {
    pub statements: Vec<Statement<'a>>,
    pub expression: Option<Box<Expression<'a>>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type<'a> {
    Named(NamedType<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamedType<'a> {
    pub name: &'a str,
}
