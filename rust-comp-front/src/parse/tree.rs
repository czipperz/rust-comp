use crate::pos::Span;
use crate::token::Token;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopLevel {
    pub visibility: Visibility,
    pub kind: TopLevelKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Visibility {
    Private,
    Path(PathVisibility),
    Public(Span),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PathVisibility {
    pub pub_span: Span,
    pub open_paren_span: Span,
    pub path: Path,
    pub close_paren_span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TopLevelKind {
    Function(Function),
    Struct(Struct),
    Enum(Enum),
    ModFile(ModFile),
    Use(Use),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Struct {
    pub struct_span: Span,
    pub name: Span,
    pub open_curly_span: Span,
    pub fields: Vec<Field>,
    pub comma_spans: Vec<Span>,
    pub close_curly_span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub visibility: Visibility,
    pub name: Span,
    pub colon_span: Span,
    pub type_: Type,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Enum {
    pub enum_span: Span,
    pub name: Span,
    pub open_curly_span: Span,
    pub variants: Vec<Variant>,
    pub comma_spans: Vec<Span>,
    pub close_curly_span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant {
    pub name: Span,
    pub data: VariantData,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VariantData {
    None,
    Tuple(TupleType),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModFile {
    pub mod_span: Span,
    pub name: Span,
    pub semicolon_span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Use {
    pub use_span: Span,
    pub path: UsePath,
    pub semicolon_span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsePath {
    pub segments: Vec<Span>,
    pub prefix_separator: Option<Span>,
    /// Same length as segments
    pub separator_spans: Vec<Span>,
    pub suffix: UsePathSuffix,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UsePathSuffix {
    Item(Span),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Path {
    pub segments: Vec<Span>,
    pub prefix_separator: Option<Span>,
    /// One shorter than segments
    pub separator_spans: Vec<Span>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    pub fn_span: Span,
    pub name: Span,
    pub open_paren_span: Span,
    pub parameters: Vec<Parameter>,
    pub comma_spans: Vec<Span>,
    pub close_paren_span: Span,
    pub return_type: Option<ReturnType>,
    pub body: Block,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter {
    pub name: Span,
    pub colon_span: Span,
    pub type_: Type,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReturnType {
    pub thin_arrow_span: Span,
    pub type_: Type,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Statement {
    pub kind: StatementKind,
    pub semicolon_span: Option<Span>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StatementKind {
    Empty,
    Expression(Expression),
    Let(Let),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    Variable(Variable),
    Paren(ParenExpression),
    Block(Block),
    If(If),
    While(While),
    Match(Match),
    Binary(Binary),
    FunctionCall(FunctionCall),
    MemberAccess(MemberAccess),
    Bool(Token),
    Tuple(Tuple),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable {
    pub name: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParenExpression {
    pub open_paren_span: Span,
    pub expression: Box<Expression>,
    pub close_paren_span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct If {
    pub if_span: Span,
    pub condition: Box<Expression>,
    pub then: Block,
    pub else_: Option<Box<Else>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Else {
    pub else_span: Span,
    pub kind: ElseKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ElseKind {
    If(If),
    Block(Block),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct While {
    pub while_span: Span,
    pub condition: Box<Expression>,
    pub block: Block,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Match {
    pub match_span: Span,
    pub value: Box<Expression>,
    pub open_curly_span: Span,
    pub matches: Vec<MatchItem>,
    pub comma_spans: Vec<Option<Span>>,
    pub close_curly_span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchItem {
    pub pattern: Pattern,
    pub fat_arrow_span: Span,
    pub value: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pattern {
    Named(Span),
    Tuple(TuplePattern),
    Paren(ParenPattern),
    NamedTuple(Span, TuplePattern),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TuplePattern {
    pub open_paren_span: Span,
    pub patterns: Vec<Pattern>,
    pub comma_spans: Vec<Span>,
    pub close_paren_span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParenPattern {
    pub open_paren_span: Span,
    pub pattern: Box<Pattern>,
    pub close_paren_span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Binary {
    pub left: Box<Expression>,
    pub op: Token,
    pub right: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionCall {
    pub function: Box<Expression>,
    pub open_paren_span: Span,
    pub arguments: Vec<Expression>,
    pub comma_spans: Vec<Span>,
    pub close_paren_span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemberAccess {
    pub object: Box<Expression>,
    pub dot_span: Span,
    pub member: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tuple {
    pub open_paren_span: Span,
    pub expressions: Vec<Expression>,
    pub comma_spans: Vec<Span>,
    pub close_paren_span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Let {
    pub let_span: Span,
    /// Normal name => Ok, _ => Err.
    pub name: Result<Span, Span>,
    pub type_: Option<LetType>,
    pub value: Option<LetValue>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetType {
    pub colon_span: Span,
    pub type_: Type,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetValue {
    pub set_span: Span,
    pub value: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub open_curly_span: Span,
    pub statements: Vec<Statement>,
    pub expression: Option<Box<Expression>>,
    pub close_curly_span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Named(NamedType),
    Ref(RefType),
    RefMut(RefMutType),
    PtrConst(PtrConstType),
    PtrMut(PtrMutType),
    Tuple(TupleType),
    Paren(ParenType),
    Hole(HoleType),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamedType {
    pub name: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RefType {
    pub ref_span: Span,
    pub type_: Box<Type>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RefMutType {
    pub ref_span: Span,
    pub mut_span: Span,
    pub type_: Box<Type>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PtrConstType {
    pub ptr_span: Span,
    pub const_span: Span,
    pub type_: Box<Type>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PtrMutType {
    pub ptr_span: Span,
    pub mut_span: Span,
    pub type_: Box<Type>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TupleType {
    pub open_paren_span: Span,
    pub types: Vec<Type>,
    pub comma_spans: Vec<Span>,
    pub close_paren_span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParenType {
    pub open_paren_span: Span,
    pub type_: Box<Type>,
    pub close_paren_span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HoleType {
    pub underscore_span: Span,
}
