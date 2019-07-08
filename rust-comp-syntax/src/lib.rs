use rust_comp_core::pos::Span;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopLevel {
    pub span: Span,
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
    pub span: Span,
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
    pub name: Symbol,
    pub open_curly_span: Span,
    pub fields: Vec<Field>,
    pub comma_spans: Vec<Span>,
    pub close_curly_span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub span: Span,
    pub visibility: Visibility,
    pub name: Symbol,
    pub colon_span: Span,
    pub type_: Type,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Enum {
    pub enum_span: Span,
    pub name: Symbol,
    pub open_curly_span: Span,
    pub variants: Vec<Variant>,
    pub comma_spans: Vec<Span>,
    pub close_curly_span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant {
    pub name: Symbol,
    pub data: VariantData,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VariantData {
    None,
    Tuple(Vec<Type>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModFile {
    pub mod_span: Span,
    pub name: Symbol,
    pub semicolon_span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Use {
    pub use_span: Span,
    pub base: Path,
    pub suffix: UsePathSuffix,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UsePathSuffix {
    Item(Symbol),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Path {
    pub prefix_separator: Option<Span>,
    pub segments: Vec<Symbol>,
    /// One shorter than segments
    pub separator_spans: Vec<Span>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    pub name: Symbol,
    pub parameters: Vec<Parameter>,
    pub return_type: Type,
    pub body: Block,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter {
    pub span: Span,
    pub name: Symbol,
    pub type_: Type,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Statement {
    pub span: Span,
    pub kind: StatementKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StatementKind {
    Empty,
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
    If(If),
    Loop(Loop),
    While(While),
    For(For),
    Match(Match),
    Binary(Binary),
    FunctionCall(FunctionCall),
    MemberCall(MemberCall),
    MemberAccess(MemberAccess),
    Tuple(Vec<Expression>),
    Value(Value),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct If {
    pub condition: Box<Expression>,
    pub then: Block,
    pub else_: Option<Box<Else>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Else {
    pub span: Span,
    pub kind: ElseKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ElseKind {
    If(If),
    Block(Block),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Loop {
    pub block: Block,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct While {
    pub condition: Box<Expression>,
    pub block: Block,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct For {
    pub var: Symbol,
    pub expr: Box<Expression>,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pattern {
    pub span: Span,
    pub kind: PatternKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PatternKind {
    Named(SymbolId),
    Hole,
    Tuple(Vec<Pattern>),
    NamedTuple(Symbol, Vec<Pattern>),
    Value(Value),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Binary {
    pub left: Box<Expression>,
    pub op: BinaryOp,
    pub right: Box<Expression>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOp {
    Times,
    DividedBy,
    Plus,
    Minus,
    BitAnd,
    BitOr,
    IsEqualTo,
    IsNotEqualTo,
    SetTo,
    And,
    Or,
}

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Bool(bool),
    Integer(u128),
}

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Type {
    pub span: Span,
    pub kind: TypeKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeKind {
    Named(Symbol),
    Ref(Box<Type>),
    RefMut(Box<Type>),
    PtrConst(Box<Type>),
    PtrMut(Box<Type>),
    Tuple(Vec<Type>),
    Hole,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Symbol {
    pub span: Span,
    pub id: SymbolId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SymbolId(pub u64);
