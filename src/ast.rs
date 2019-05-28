pub enum TopLevel {
    Function(Function),
}

pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Vec<Statement>,
}

pub struct Parameter {}

pub struct Statement {}
