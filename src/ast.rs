#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TopLevel<'a> {
    Function(&'a Function<'a>),
    ModFile(&'a ModFile<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModFile<'a> {
    pub mod_: &'a str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function<'a> {
    pub name: &'a str,
    pub parameters: Vec<&'a Parameter<'a>>,
    pub body: &'a Block<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter<'a> {
    pub name: &'a str,
    pub type_: &'a Type<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement<'a> {
    Empty,
    Expression(&'a Expression<'a>),
    Let(&'a Let<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression<'a> {
    Variable(&'a Variable<'a>),
    Block(&'a Block<'a>),
    If(&'a If<'a>),
    While(&'a While<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable<'a> {
    pub name: &'a str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct If<'a> {
    pub condition: &'a Expression<'a>,
    pub then: &'a Block<'a>,
    pub else_: Option<&'a Else<'a>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Else<'a> {
    If(&'a If<'a>),
    Block(&'a Block<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct While<'a> {
    pub condition: &'a Expression<'a>,
    pub block: &'a Block<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Let<'a> {
    pub name: &'a str,
    pub type_: Option<&'a Type<'a>>,
    pub value: Option<&'a Expression<'a>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block<'a> {
    pub statements: Vec<&'a Statement<'a>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type<'a> {
    Named(&'a NamedType<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamedType<'a> {
    pub name: &'a str,
}
