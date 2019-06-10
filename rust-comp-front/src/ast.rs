#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopLevel<'a> {
    pub kind: TopLevelKind<'a>,
    pub visibility: Visibility<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Visibility<'a> {
    Private,
    Path(Path<'a>),
    Public,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TopLevelKind<'a> {
    Function(Function<'a>),
    Struct(Struct<'a>),
    ModFile(ModFile<'a>),
    Use(Use<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Struct<'a> {
    pub name: &'a str,
    pub fields: Vec<Field<'a>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field<'a> {
    pub visibility: Visibility<'a>,
    pub name: &'a str,
    pub type_: Type<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModFile<'a> {
    pub mod_: &'a str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Use<'a> {
    pub path: Path<'a>,
    pub item: &'a str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Path<'a> {
    pub path: Vec<&'a str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function<'a> {
    pub name: &'a str,
    pub parameters: Vec<Parameter<'a>>,
    pub return_type: Type<'a>,
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
    FunctionCall(FunctionCall<'a>),
    Bool(bool),
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
    EqualTo,
    NotEqualTo,
    SetTo,
    And,
    Or,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionCall<'a> {
    pub function: Box<Expression<'a>>,
    pub arguments: Vec<Expression<'a>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Let<'a> {
    pub name: Option<&'a str>,
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
    Ref(Box<Type<'a>>),
    RefMut(Box<Type<'a>>),
    PtrConst(Box<Type<'a>>),
    PtrMut(Box<Type<'a>>),
    Tuple(Vec<Type<'a>>),
    Hole,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamedType<'a> {
    pub name: &'a str,
}
