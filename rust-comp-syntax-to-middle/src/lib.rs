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
    Some(middle::Statement { span: s.span, kind })
}

fn convert_statement_kind(sk: syntax::StatementKind) -> Option<middle::StatementKind> {
    match sk {
        syntax::StatementKind::Empty => None,
        syntax::StatementKind::Expression(e) => {
            Some(middle::StatementKind::Expression(convert_expression(e)))
        }
        syntax::StatementKind::Let(l) => Some(middle::StatementKind::Let(convert_let(l))),
    }
}

fn convert_expression(e: syntax::Expression) -> middle::Expression {
    middle::Expression {
        span: e.span,
        kind: convert_expression_kind(e.kind),
    }
}

fn convert_expression_kind(ek: syntax::ExpressionKind) -> middle::ExpressionKind {
    match ek {
        syntax::ExpressionKind::Variable(v) => middle::ExpressionKind::Variable(v),
        syntax::ExpressionKind::Block(v) => middle::ExpressionKind::Block(convert_block(v)),
        syntax::ExpressionKind::If(i) => middle::ExpressionKind::Match(convert_if(i)),
        syntax::ExpressionKind::Loop(l) => middle::ExpressionKind::Loop(convert_loop(l)),
        syntax::ExpressionKind::While(w) => middle::ExpressionKind::Loop(convert_while(w)),
        syntax::ExpressionKind::For(f) => middle::ExpressionKind::Loop(convert_for(f)),
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
        syntax::ExpressionKind::Value(v) => middle::ExpressionKind::Value(v),
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

fn convert_else(e: syntax::Else) -> middle::Expression {
    middle::Expression {
        span: e.span,
        kind: match e.kind {
            syntax::ElseKind::If(i) => middle::ExpressionKind::Match(convert_if(i)),
            syntax::ElseKind::Block(b) => middle::ExpressionKind::Block(convert_block(b)),
        },
    }
}

fn convert_loop(l: syntax::Loop) -> middle::Loop {
    middle::Loop {
        block: convert_block(l.block),
    }
}

fn convert_while(w: syntax::While) -> middle::Loop {
    middle::Loop {
        block: middle::Block {
            statements: vec![middle::Statement {
                span: w.condition.span,
                kind: middle::StatementKind::Expression(middle::Expression {
                    span: w.condition.span,
                    kind: middle::ExpressionKind::Match(convert_if(syntax::If {
                        condition: w.condition,
                        then: w.block,
                        else_: None,
                    })),
                }),
            }],
            expression: None,
        },
    }
}

fn convert_for(f: syntax::For) -> middle::Loop {
    unimplemented!()
}

fn convert_match(m: syntax::Match) -> middle::Match {
    middle::Match {
        value: Box::new(convert_expression(*m.value)),
        matches: m.matches.map(convert_match_item).collect(),
    }
}

fn convert_match_item(mi: syntax::MatchItem) -> middle::MatchItem {
    middle::MatchItem {
        span: mi.span,
        pattern: mi.pattern,
        value: convert_expression(mi.value),
    }
}

fn convert_binary(b: syntax::Binary) -> middle::Binary {
    middle::Binary {
        left: Box::new(convert_expression(*b.left)),
        op: b.op,
        right: Box::new(convert_expression(*b.right)),
    }
}

fn convert_function_call(fc: syntax::FunctionCall) -> middle::FunctionCall {
    middle::FunctionCall {
        function: Box::new(convert_expression(*fc.function)),
        arguments: fc.arguments.into_iter().map(convert_expression).collect(),
    }
}

fn convert_member_call(mc: syntax::MemberCall) -> middle::MemberCall {
    middle::FunctionCall {
        member: Box::new(convert_member_access(*mc.member)),
        arguments: mc.arguments.into_iter().map(convert_expression).collect(),
    }
}

fn convert_member_access(ma: syntax::MemberAccess) -> middle::MemberAccess {
    middle::MemberAccess {
        object: Box::new(convert_expression(*ma.object)),
        member: ma.member,
    }
}

fn convert_let(l: syntax::Let) -> middle::Let {
    middle::Let {
        name_span: l.name_span,
        name: l.name,
        type_: l.type_,
        value: l.value.map(convert_expression),
    }
}
