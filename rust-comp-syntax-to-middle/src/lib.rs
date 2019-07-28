use rust_comp_middle as middle;
use rust_comp_syntax as syntax;

fn convert_top_level(tl: syntax::TopLevel) -> middle::TopLevel {
    middle::TopLevel {
        span: tl.span,
        visibility: tl.visibility,
        kind: convert_top_level_kind(tl.kind),
    }
}

fn convert_top_level_kind(tl: syntax::TopLevelKind) -> middle::TopLevelKind {
    match tl {
        syntax::TopLevelKind::Function(f) => middle::TopLevelKind::Function(convert_function(f)),
        syntax::TopLevelKind::Struct(s) => middle::TopLevelKind::Struct(s),
        syntax::TopLevelKind::Enum(e) => middle::TopLevelKind::Enum(e),
        syntax::TopLevelKind::ModFile(mf) => middle::TopLevelKind::ModFile(mf),
        syntax::TopLevelKind::Use(u) => middle::TopLevelKind::Use(u),
    }
}

fn convert_function(f: syntax::Function) -> middle::Function {
    middle::Function {
        name: f.name,
        parameters: f.parameters,
        return_type: f.return_type,
        body: convert_block(f.body),
    }
}

fn convert_block(b: syntax::Block) -> middle::Block {
    middle::Block {
        statements: b
            .statements
            .into_iter()
            .filter_map(convert_statement)
            .collect(),
        expression: b.expression.map(|e| Box::new(convert_expression(*e))),
    }
}

fn convert_statement(s: syntax::Statement) -> Option<middle::Statement> {
    let kind = convert_statement_kind(s.kind)?;
    middle::Statement { span: s.span, kind }
}

fn convert_statement_kind(sk: syntax::StatementKind) -> Option<middle::StatementKind> {
    match sk {
        syntax::StatementKind::Empty => None,
        syntax::StatementKind::Expression(e) => {
            middle::StatementKind::Expression(convert_expression(e))
        }
        syntax::StatementKind::Let(l) => middle::StatementKind::Let(convert_let(l)),
    }
}

fn convert_expression(e: syntax::Expression) -> middle::Expression {
    middle::Expression {
        span: e.span,
        kind: convert_expression_kind(e),
    }
}

fn convert_expression_kind(ek: syntax::ExpressionKind) -> middle::ExpressionKind {
    match ek {
        syntax::ExpressionKind::Variable(v) => middle::ExpressionKind::Variable(v),
        syntax::ExpressionKind::Block(v) => middle::ExpressionKind::Block(convert_block(v)),
        syntax::ExpressionKind::If(i) => middle::ExpressionKind::Match(convert_if(i)),
        syntax::ExpressionKind::Loop(l) => middle::ExpressionKind::Loop(convert_loop(l)),
        syntax::ExpressionKind::While(w) => middle::ExpressionKind::Loop(convert_while(w)),
        syntax::ExpressionKind::For(f) => middle::ExpressionKind::For(convert_for(f)),
        syntax::ExpressionKind::Match(m) => middle::ExpressionKind::Match(convert_match(m)),
        syntax::ExpressionKind::Binary(b) => middle::ExpressionKind::Binary(convert_binary(b)),
        syntax::ExpressionKind::FunctionCall(fc) => {
            middle::ExpressionKind::FunctionCall(convert_function_call(fc))
        }
        syntax::ExpressionKind::MemberCall(mc) => {
            middle::ExpressionKind::MemberCall(convert_member_call(mc))
        }
        syntax::ExpressionKind::MemberAccess(ma) => {
            middle::ExpressionKind::MemberAccess(convert_member_access(ma))
        }
        syntax::ExpressionKind::Bool(b) => middle::ExpressionKind::Bool(convert_bool(b)),
        syntax::ExpressionKind::Integer(i) => middle::ExpressionKind::Integer(convert_integer(i)),
        syntax::ExpressionKind::Tuple(t) => {
            middle::ExpressionKind::Tuple(t.into_iter().map(convert_expression).collect())
        }
    }
}

fn convert_if(i: syntax::If) -> middle::Match {
    middle::Match {
        value: Box::new(convert_expression(*i.condition)),
        matches: vec![
            middle::MatchItem {
                span: i.span,
                pattern: middle::Pattern {
                    span: unimplemented!(),
                    kind: middle::PatternKind::Value(middle::Value::Bool(true)),
                },
                value: i.then,
            },
            middle::MatchItem {
                span: unimplemented!(),
                pattern: middle::Pattern {
                    span: unimplemented!(),
                    kind: middle::PatternKind::Hole,
                },
                value: convert_else(i.else_),
            },
        ],
    }
}

fn convert_loop(l: syntax::Loop) -> middle::Loop {
    middle::Loop {}
}

fn convert_let(l: syntax::Let) -> middle::Let {
    middle::Let {
        name_span: l.name_span,
        name: l.name,
        type_: l.type_,
        value: l.value.map(convert_expression),
    }
}
