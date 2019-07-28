use rust_comp_core::pos::Span;
use rust_comp_syntax as syntax;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopLevel {
    pub span: Span,
    pub visibility: Visibility,
    pub kind: TopLevelKind,
}

pub use syntax::PathVisibility;
pub use syntax::Visibility;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TopLevelKind {
    Function(Function),
    Struct(Struct),
    Enum(Enum),
    ModFile(ModFile),
    Use(Use),
}

pub use syntax::Enum;
pub use syntax::Field;
pub use syntax::ModFile;
pub use syntax::Path;
pub use syntax::Struct;
pub use syntax::Use;
pub use syntax::UsePathSuffix;
pub use syntax::Variant;
pub use syntax::VariantData;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    pub name: Symbol,
    pub parameters: Vec<Parameter>,
    pub return_type: Type,
    pub body: Block,
}

pub use syntax::Parameter;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Statement {
    pub span: Span,
    pub kind: StatementKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StatementKind {
    Expression(Expression),
    Let(Let),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expression {
    pub span: Span,
    pub kind: ExpressionKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressionKind {
    Variable(Symbol),
    Block(Block),
    Loop(Loop),
    Match(Match),
    Binary(Binary),
    FunctionCall(FunctionCall),
    MemberCall(MemberCall),
    MemberAccess(MemberAccess),
    Value(Value),
    Tuple(Vec<Expression>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Loop {
    pub block: Block,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Match {
    pub value: Box<Expression>,
    pub matches: Vec<MatchItem>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchItem {
    pub span: Span,
    pub pattern: Pattern,
    pub value: Expression,
}

pub use syntax::Pattern;
pub use syntax::PatternKind;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Binary {
    pub left: Box<Expression>,
    pub op: BinaryOp,
    pub right: Box<Expression>,
}

pub use syntax::BinaryOp;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionCall {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemberCall {
    pub member: MemberAccess,
    pub arguments: Vec<Expression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemberAccess {
    pub object: Box<Expression>,
    pub member: Symbol,
}

pub use syntax::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Let {
    pub name_span: Span,
    /// Normal name => Some, _ => None.
    pub name: Option<SymbolId>,
    pub type_: Option<Type>,
    pub value: Option<Expression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub expression: Option<Box<Expression>>,
}

pub use syntax::Symbol;
pub use syntax::SymbolId;
pub use syntax::Type;
pub use syntax::TypeKind;
