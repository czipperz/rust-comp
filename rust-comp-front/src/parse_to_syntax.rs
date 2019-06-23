use crate::parse;
use crate::pos::Span;
use crate::syntax;
use crate::token::TokenKind;

pub fn convert_top_level(top_level: &parse::TopLevel) -> syntax::TopLevel {
    use parse::TopLevelKind::*;
    let visibility = convert_visibility(&top_level.visibility);
    let (ks, ke) = match &top_level.kind {
        Function(parse::Function { fn_span, body, .. }) => (*fn_span, body.close_curly_span),
        Struct(parse::Struct {
            struct_span,
            close_curly_span,
            ..
        }) => (*struct_span, *close_curly_span),
        Enum(parse::Enum {
            enum_span,
            close_curly_span,
            ..
        }) => (*enum_span, *close_curly_span),
        ModFile(parse::ModFile {
            mod_span,
            semicolon_span,
            ..
        }) => (*mod_span, *semicolon_span),
        Use(parse::Use {
            use_span,
            semicolon_span,
            ..
        }) => (*use_span, *semicolon_span),
    };
    let kind = convert_top_level_kind(&top_level.kind);
    syntax::TopLevel {
        span: span_encompassing(ks, ke),
        visibility,
        kind,
    }
}

pub fn convert_visibility(visibility: &parse::Visibility) -> syntax::Visibility {
    use parse::Visibility::*;
    match visibility {
        Private => syntax::Visibility::Private,
        Path(parse::PathVisibility {
            pub_span,
            path,
            close_paren_span,
            ..
        }) => syntax::Visibility::Path(syntax::PathVisibility {
            span: span_encompassing(*pub_span, *close_paren_span),
            path: convert_path(path),
        }),
        Public(s) => syntax::Visibility::Public(*s),
    }
}

pub fn convert_top_level_kind(kind: &parse::TopLevelKind) -> syntax::TopLevelKind {
    use parse::TopLevelKind::*;
    match kind {
        Function(f) => syntax::TopLevelKind::Function(convert_function(f)),
        Struct(s) => syntax::TopLevelKind::Struct(convert_struct(s)),
        Enum(e) => syntax::TopLevelKind::Enum(convert_enum(e)),
        ModFile(m) => syntax::TopLevelKind::ModFile(convert_mod_file(m)),
        Use(u) => syntax::TopLevelKind::Use(convert_use(u)),
    }
}

pub fn convert_struct(s: &parse::Struct) -> syntax::Struct {
    syntax::Struct {
        name: convert_symbol(s.name),
        fields: s.fields.iter().map(convert_field).collect(),
    }
}

pub fn convert_field(f: &parse::Field) -> syntax::Field {
    use parse::Visibility::*;
    let type_ = convert_type(&f.type_);
    syntax::Field {
        span: span_encompassing(
            match f.visibility {
                Private => f.name,
                Path(parse::PathVisibility { pub_span, .. }) => pub_span,
                Public(s) => s,
            },
            type_.span,
        ),
        visibility: convert_visibility(&f.visibility),
        name: convert_symbol(f.name),
        type_,
    }
}

pub fn convert_enum(e: &parse::Enum) -> syntax::Enum {
    syntax::Enum {
        name: convert_symbol(e.name),
        variants: e.variants.iter().map(convert_variant).collect(),
    }
}

pub fn convert_variant(v: &parse::Variant) -> syntax::Variant {
    syntax::Variant {
        name: convert_symbol(v.name),
        data: convert_variant_data(&v.data),
    }
}

pub fn convert_variant_data(vd: &parse::VariantData) -> syntax::VariantData {
    use parse::VariantData::*;
    match vd {
        None => syntax::VariantData::None,
        Tuple(t) => syntax::VariantData::Tuple(t.types.iter().map(convert_type).collect()),
    }
}

pub fn convert_mod_file(mf: &parse::ModFile) -> syntax::ModFile {
    syntax::ModFile {
        name: convert_symbol(mf.name),
    }
}

pub fn convert_use(u: &parse::Use) -> syntax::Use {
    syntax::Use {
        base: syntax::Path {
            prefix_separator: u.path.prefix_separator.is_some(),
            segments: u.path.segments.iter().map(|s| convert_symbol(*s)).collect(),
        },
        suffix: convert_use_path_suffix(&u.path.suffix),
    }
}

pub fn convert_use_path_suffix(ups: &parse::UsePathSuffix) -> syntax::UsePathSuffix {
    use parse::UsePathSuffix::*;
    match ups {
        Item(s) => syntax::UsePathSuffix::Item(convert_symbol(*s)),
    }
}

pub fn convert_path(path: &parse::Path) -> syntax::Path {
    syntax::Path {
        prefix_separator: path.prefix_separator.is_some(),
        segments: path.segments.iter().map(|s| convert_symbol(*s)).collect(),
    }
}

pub fn convert_function(f: &parse::Function) -> syntax::Function {
    syntax::Function {
        name: convert_symbol(f.name),
        parameters: f.parameters.iter().map(convert_parameter).collect(),
        return_type: f
            .return_type
            .as_ref()
            .map(|rt| convert_type(&rt.type_))
            .unwrap_or(syntax::Type {
                span: f.body.open_curly_span,
                kind: syntax::TypeKind::Tuple(vec![]),
            }),
        body: convert_block(&f.body),
    }
}

pub fn convert_parameter(p: &parse::Parameter) -> syntax::Parameter {
    let type_ = convert_type(&p.type_);
    syntax::Parameter {
        span: span_encompassing(p.name, type_.span),
        name: convert_symbol(p.name),
        type_,
    }
}

pub fn convert_statement(s: &parse::Statement) -> syntax::Statement {
    use syntax::StatementKind::*;
    let kind = convert_statement_kind(&s.kind);
    syntax::Statement {
        span: match &kind {
            Empty => s.semicolon_span.unwrap(),
            Expression(e) => match s.semicolon_span {
                Some(semicolon_span) => span_encompassing(e.span, semicolon_span),
                None => e.span,
            },
            Let(_) => span_encompassing(
                if let parse::StatementKind::Let(pl) = &s.kind {
                    pl.let_span
                } else {
                    unreachable!()
                },
                s.semicolon_span.unwrap(),
            ),
        },
        kind,
    }
}

pub fn convert_statement_kind(sk: &parse::StatementKind) -> syntax::StatementKind {
    use parse::StatementKind::*;
    match sk {
        Empty => syntax::StatementKind::Empty,
        Expression(e) => syntax::StatementKind::Expression(convert_expression(e)),
        Let(l) => syntax::StatementKind::Let(convert_let(l)),
    }
}

pub fn convert_expression(e: &parse::Expression) -> syntax::Expression {
    use parse::Expression::*;
    match e {
        Variable(parse::Variable { name }) => syntax::Expression {
            span: *name,
            kind: syntax::ExpressionKind::Variable(convert_symbol(*name)),
        },
        Paren(parse::ParenExpression { expression, .. }) => convert_expression(&expression),
        Block(b) => syntax::Expression {
            span: span_encompassing(b.open_curly_span, b.close_curly_span),
            kind: syntax::ExpressionKind::Block(convert_block(b)),
        },
        If(i) => {
            let si = convert_if(i);
            syntax::Expression {
                span: if_span(&i, &si),
                kind: syntax::ExpressionKind::If(si),
            }
        }
        While(w) => syntax::Expression {
            span: span_encompassing(w.while_span, w.block.close_curly_span),
            kind: syntax::ExpressionKind::While(convert_while(w)),
        },
        Match(m) => syntax::Expression {
            span: span_encompassing(m.match_span, m.close_curly_span),
            kind: syntax::ExpressionKind::Match(convert_match(m)),
        },
        Binary(b) => {
            // this is a rust compiler bug, remove when
            // https://github.com/rust-lang/rust/issues/62083 is closed
            #[allow(unused_variables)]
            let sb = convert_binary(b);
            syntax::Expression {
                span: span_encompassing(sb.left.span, sb.right.span),
                kind: syntax::ExpressionKind::Binary(sb),
            }
        }
        FunctionCall(fc) => {
            let sfc = convert_function_call(fc);
            syntax::Expression {
                span: span_encompassing(sfc.function.span, fc.close_paren_span),
                kind: syntax::ExpressionKind::FunctionCall(sfc),
            }
        }
        Bool(b) => syntax::Expression {
            span: *b,
            kind: syntax::ExpressionKind::Bool(convert_bool(*b)),
        },
        Tuple(t) => syntax::Expression {
            span: span_encompassing(t.open_paren_span, t.close_paren_span),
            kind: syntax::ExpressionKind::Tuple(
                t.expressions.iter().map(convert_expression).collect(),
            ),
        },
    }
}

pub fn convert_if(i: &parse::If) -> syntax::If {
    syntax::If {
        condition: Box::new(convert_expression(&i.condition)),
        then: convert_block(&i.then),
        else_: i.else_.as_ref().map(|e| Box::new(convert_else(e))),
    }
}

fn if_span(i: &parse::If, si: &syntax::If) -> Span {
    span_encompassing(
        i.if_span,
        if let Some(e) = &si.else_ {
            e.span
        } else {
            i.then.close_curly_span
        },
    )
}

pub fn convert_else(e: &parse::Else) -> syntax::Else {
    let kind = convert_else_kind(&e.kind);
    syntax::Else {
        span: match (&e.kind, &kind) {
            (parse::ElseKind::If(i), syntax::ElseKind::If(si)) => if_span(i, si),
            (parse::ElseKind::Block(b), syntax::ElseKind::Block(_)) => {
                span_encompassing(b.open_curly_span, b.close_curly_span)
            }
            _ => unreachable!(),
        },
        kind,
    }
}

pub fn convert_else_kind(ek: &parse::ElseKind) -> syntax::ElseKind {
    use parse::ElseKind::*;
    match ek {
        If(i) => syntax::ElseKind::If(convert_if(i)),
        Block(b) => syntax::ElseKind::Block(convert_block(b)),
    }
}

pub fn convert_while(w: &parse::While) -> syntax::While {
    syntax::While {
        condition: Box::new(convert_expression(&w.condition)),
        block: convert_block(&w.block),
    }
}

pub fn convert_match(m: &parse::Match) -> syntax::Match {
    syntax::Match {
        value: Box::new(convert_expression(&m.value)),
        matches: m.matches.iter().map(convert_match_item).collect(),
    }
}

pub fn convert_match_item(mi: &parse::MatchItem) -> syntax::MatchItem {
    let pattern = convert_pattern(&mi.pattern);
    let value = convert_expression(&mi.value);
    syntax::MatchItem {
        span: span_encompassing(pattern.span, value.span),
        pattern,
        value,
    }
}

pub fn convert_pattern(p: &parse::Pattern) -> syntax::Pattern {
    fn convert_tuple_pattern(t: &parse::TuplePattern) -> Vec<syntax::Pattern> {
        t.patterns.iter().map(convert_pattern).collect()
    }

    use parse::Pattern::*;
    match p {
        Named(s) => syntax::Pattern {
            span: *s,
            kind: syntax::PatternKind::Named(convert_symbol(*s).id),
        },
        Tuple(t) => syntax::Pattern {
            span: span_encompassing(t.open_paren_span, t.close_paren_span),
            kind: syntax::PatternKind::Tuple(convert_tuple_pattern(t)),
        },
        Paren(p) => convert_pattern(&p.pattern),
        NamedTuple(n, t) => syntax::Pattern {
            span: span_encompassing(*n, t.close_paren_span),
            kind: syntax::PatternKind::NamedTuple(convert_symbol(*n), convert_tuple_pattern(t)),
        },
    }
}

pub fn convert_binary(b: &parse::Binary) -> syntax::Binary {
    syntax::Binary {
        left: Box::new(convert_expression(&b.left)),
        op: convert_binary_op(b.op.kind),
        right: Box::new(convert_expression(&b.right)),
    }
}

pub fn convert_binary_op(_tk: TokenKind) -> syntax::BinaryOp {
    unimplemented!()
}

pub fn convert_function_call(fc: &parse::FunctionCall) -> syntax::FunctionCall {
    syntax::FunctionCall {
        function: Box::new(convert_expression(&fc.function)),
        arguments: fc.arguments.iter().map(convert_expression).collect(),
    }
}

pub fn convert_bool(_s: Span) -> bool {
    unimplemented!()
}

pub fn convert_let(l: &parse::Let) -> syntax::Let {
    syntax::Let {
        name_span: match l.name {
            Ok(s) => s,
            Err(s) => s,
        },
        name: l.name.ok().map(|n| convert_symbol(n).id),
        type_: l.type_.as_ref().map(|lt| convert_type(&lt.type_)),
        value: l.value.as_ref().map(|lv| convert_expression(&lv.value)),
    }
}

pub fn convert_block(b: &parse::Block) -> syntax::Block {
    syntax::Block {
        statements: b.statements.iter().map(convert_statement).collect(),
        expression: b
            .expression
            .as_ref()
            .map(|e| Box::new(convert_expression(e))),
    }
}

pub fn convert_type(t: &parse::Type) -> syntax::Type {
    use parse::Type::*;
    match t {
        Named(parse::NamedType { name }) => syntax::Type {
            span: *name,
            kind: syntax::TypeKind::Named(convert_symbol(*name)),
        },
        Ref(parse::RefType {
            ref_span, type_, ..
        }) => {
            let inner = convert_type(&type_);
            syntax::Type {
                span: span_encompassing(*ref_span, inner.span),
                kind: syntax::TypeKind::Ref(Box::new(inner)),
            }
        }
        RefMut(parse::RefMutType {
            ref_span, type_, ..
        }) => {
            let inner = convert_type(&type_);
            syntax::Type {
                span: span_encompassing(*ref_span, inner.span),
                kind: syntax::TypeKind::RefMut(Box::new(inner)),
            }
        }
        PtrConst(parse::PtrConstType {
            ptr_span, type_, ..
        }) => {
            let inner = convert_type(&type_);
            syntax::Type {
                span: span_encompassing(*ptr_span, inner.span),
                kind: syntax::TypeKind::PtrConst(Box::new(inner)),
            }
        }
        PtrMut(parse::PtrMutType {
            ptr_span, type_, ..
        }) => {
            let inner = convert_type(&type_);
            syntax::Type {
                span: span_encompassing(*ptr_span, inner.span),
                kind: syntax::TypeKind::PtrMut(Box::new(inner)),
            }
        }
        Tuple(parse::TupleType {
            open_paren_span,
            types,
            close_paren_span,
            ..
        }) => syntax::Type {
            span: span_encompassing(*open_paren_span, *close_paren_span),
            kind: syntax::TypeKind::Tuple(types.into_iter().map(convert_type).collect()),
        },
        Paren(parse::ParenType { type_, .. }) => convert_type(&type_),
        Hole(parse::HoleType { underscore_span }) => syntax::Type {
            span: *underscore_span,
            kind: syntax::TypeKind::Hole,
        },
    }
}

pub fn convert_symbol(_span: Span) -> syntax::Symbol {
    unimplemented!()
}

fn span_encompassing(start: Span, end: Span) -> Span {
    debug_assert_eq!(start.file, end.file);
    Span {
        file: start.file,
        start: start.start,
        end: end.end,
    }
}
