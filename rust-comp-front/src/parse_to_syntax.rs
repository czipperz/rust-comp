use crate::parse;
use crate::pos::Span;
use crate::token::TokenKind;
use rust_comp_core::diagnostic::Diagnostic;
use rust_comp_syntax as syntax;

pub struct Context<'a> {
    diagnostic: &'a Diagnostic,
}

impl<'a> Context<'a> {
    pub fn new(diagnostic: &'a Diagnostic) -> Self {
        Context { diagnostic }
    }

    pub fn convert_top_level(&mut self, top_level: &parse::TopLevel) -> syntax::TopLevel {
        use parse::TopLevelKind::*;
        let visibility = self.convert_visibility(&top_level.visibility);
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
        let kind = self.convert_top_level_kind(&top_level.kind);
        syntax::TopLevel {
            span: span_encompassing(ks, ke),
            visibility,
            kind,
        }
    }

    pub fn convert_visibility(&mut self, visibility: &parse::Visibility) -> syntax::Visibility {
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
                path: self.convert_path(path),
            }),
            Public(s) => syntax::Visibility::Public(*s),
        }
    }

    pub fn convert_top_level_kind(&mut self, kind: &parse::TopLevelKind) -> syntax::TopLevelKind {
        use parse::TopLevelKind::*;
        match kind {
            Function(f) => syntax::TopLevelKind::Function(self.convert_function(f)),
            Struct(s) => syntax::TopLevelKind::Struct(self.convert_struct(s)),
            Enum(e) => syntax::TopLevelKind::Enum(self.convert_enum(e)),
            ModFile(m) => syntax::TopLevelKind::ModFile(self.convert_mod_file(m)),
            Use(u) => syntax::TopLevelKind::Use(self.convert_use(u)),
        }
    }

    pub fn convert_struct(&mut self, s: &parse::Struct) -> syntax::Struct {
        syntax::Struct {
            name: self.convert_symbol(s.name),
            fields: s.fields.iter().map(|f| self.convert_field(f)).collect(),
        }
    }

    pub fn convert_field(&mut self, f: &parse::Field) -> syntax::Field {
        use parse::Visibility::*;
        let type_ = self.convert_type(&f.type_);
        syntax::Field {
            span: span_encompassing(
                match f.visibility {
                    Private => f.name,
                    Path(parse::PathVisibility { pub_span, .. }) => pub_span,
                    Public(s) => s,
                },
                type_.span,
            ),
            visibility: self.convert_visibility(&f.visibility),
            name: self.convert_symbol(f.name),
            type_,
        }
    }

    pub fn convert_enum(&mut self, e: &parse::Enum) -> syntax::Enum {
        syntax::Enum {
            name: self.convert_symbol(e.name),
            variants: e.variants.iter().map(|v| self.convert_variant(v)).collect(),
        }
    }

    pub fn convert_variant(&mut self, v: &parse::Variant) -> syntax::Variant {
        syntax::Variant {
            name: self.convert_symbol(v.name),
            data: self.convert_variant_data(&v.data),
        }
    }

    pub fn convert_variant_data(&mut self, vd: &parse::VariantData) -> syntax::VariantData {
        use parse::VariantData::*;
        match vd {
            None => syntax::VariantData::None,
            Tuple(t) => {
                syntax::VariantData::Tuple(t.types.iter().map(|t| self.convert_type(t)).collect())
            }
        }
    }

    pub fn convert_mod_file(&mut self, mf: &parse::ModFile) -> syntax::ModFile {
        syntax::ModFile {
            name: self.convert_symbol(mf.name),
        }
    }

    pub fn convert_use(&mut self, u: &parse::Use) -> syntax::Use {
        syntax::Use {
            base: syntax::Path {
                prefix_separator: u.path.prefix_separator.is_some(),
                segments: u
                    .path
                    .segments
                    .iter()
                    .map(|s| self.convert_symbol(*s))
                    .collect(),
            },
            suffix: self.convert_use_path_suffix(&u.path.suffix),
        }
    }

    pub fn convert_use_path_suffix(&mut self, ups: &parse::UsePathSuffix) -> syntax::UsePathSuffix {
        use parse::UsePathSuffix::*;
        match ups {
            Item(s) => syntax::UsePathSuffix::Item(self.convert_symbol(*s)),
        }
    }

    pub fn convert_path(&mut self, path: &parse::Path) -> syntax::Path {
        syntax::Path {
            prefix_separator: path.prefix_separator.is_some(),
            segments: path
                .segments
                .iter()
                .map(|s| self.convert_symbol(*s))
                .collect(),
        }
    }

    pub fn convert_function(&mut self, f: &parse::Function) -> syntax::Function {
        syntax::Function {
            name: self.convert_symbol(f.name),
            parameters: f
                .parameters
                .iter()
                .map(|p| self.convert_parameter(p))
                .collect(),
            return_type: f
                .return_type
                .as_ref()
                .map(|rt| self.convert_type(&rt.type_))
                .unwrap_or(syntax::Type {
                    span: f.body.open_curly_span,
                    kind: syntax::TypeKind::Tuple(vec![]),
                }),
            body: self.convert_block(&f.body),
        }
    }

    pub fn convert_parameter(&mut self, p: &parse::Parameter) -> syntax::Parameter {
        let type_ = self.convert_type(&p.type_);
        syntax::Parameter {
            span: span_encompassing(p.name, type_.span),
            name: self.convert_symbol(p.name),
            type_,
        }
    }

    pub fn convert_statement(&mut self, s: &parse::Statement) -> syntax::Statement {
        use syntax::StatementKind::*;
        let kind = self.convert_statement_kind(&s.kind);
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

    pub fn convert_statement_kind(&mut self, sk: &parse::StatementKind) -> syntax::StatementKind {
        use parse::StatementKind::*;
        match sk {
            Empty => syntax::StatementKind::Empty,
            Expression(e) => syntax::StatementKind::Expression(self.convert_expression(e)),
            Let(l) => syntax::StatementKind::Let(self.convert_let(l)),
        }
    }

    pub fn convert_expression(&mut self, e: &parse::Expression) -> syntax::Expression {
        use parse::Expression::*;
        match e {
            Variable(parse::Variable { name }) => syntax::Expression {
                span: *name,
                kind: syntax::ExpressionKind::Variable(self.convert_symbol(*name)),
            },
            Paren(parse::ParenExpression { expression, .. }) => {
                self.convert_expression(&expression)
            }
            Block(b) => syntax::Expression {
                span: span_encompassing(b.open_curly_span, b.close_curly_span),
                kind: syntax::ExpressionKind::Block(self.convert_block(b)),
            },
            If(i) => {
                let si = self.convert_if(i);
                syntax::Expression {
                    span: if_span(&i, &si),
                    kind: syntax::ExpressionKind::If(si),
                }
            }
            Loop(l) => syntax::Expression {
                span: span_encompassing(l.loop_span, l.block.close_curly_span),
                kind: syntax::ExpressionKind::Loop(self.convert_loop(l)),
            },
            While(w) => syntax::Expression {
                span: span_encompassing(w.while_span, w.block.close_curly_span),
                kind: syntax::ExpressionKind::While(self.convert_while(w)),
            },
            For(f) => syntax::Expression {
                span: span_encompassing(f.for_span, f.block.close_curly_span),
                kind: syntax::ExpressionKind::For(self.convert_for(f)),
            },
            Match(m) => syntax::Expression {
                span: span_encompassing(m.match_span, m.close_curly_span),
                kind: syntax::ExpressionKind::Match(self.convert_match(m)),
            },
            Binary(b) => {
                let sb = self.convert_binary(b);
                syntax::Expression {
                    span: span_encompassing(sb.left.span, sb.right.span),
                    kind: syntax::ExpressionKind::Binary(sb),
                }
            }
            FunctionCall(fc) => {
                if let parse::Expression::MemberAccess(ma) = &*fc.function {
                    let sma = self.convert_member_access(&ma);
                    syntax::Expression {
                        span: span_encompassing(sma.object.span, fc.close_paren_span),
                        kind: syntax::ExpressionKind::MemberCall(syntax::MemberCall {
                            member: sma,
                            arguments: fc
                                .arguments
                                .iter()
                                .map(|a| self.convert_expression(a))
                                .collect(),
                        }),
                    }
                } else {
                    let sfc = self.convert_function_call(fc);
                    syntax::Expression {
                        span: span_encompassing(sfc.function.span, fc.close_paren_span),
                        kind: syntax::ExpressionKind::FunctionCall(sfc),
                    }
                }
            }
            MemberAccess(ma) => {
                let sma = self.convert_member_access(ma);
                syntax::Expression {
                    span: span_encompassing(sma.object.span, ma.member),
                    kind: syntax::ExpressionKind::MemberAccess(sma),
                }
            }
            Bool(b) => syntax::Expression {
                span: b.span,
                kind: syntax::ExpressionKind::Bool(self.convert_bool(b.kind)),
            },
            Integer(parse::Integer { span, value }) => syntax::Expression {
                span: *span,
                kind: syntax::ExpressionKind::Integer(*value),
            },
            Tuple(t) => syntax::Expression {
                span: span_encompassing(t.open_paren_span, t.close_paren_span),
                kind: syntax::ExpressionKind::Tuple(
                    t.expressions
                        .iter()
                        .map(|e| self.convert_expression(e))
                        .collect(),
                ),
            },
        }
    }

    pub fn convert_if(&mut self, i: &parse::If) -> syntax::If {
        syntax::If {
            condition: Box::new(self.convert_expression(&i.condition)),
            then: self.convert_block(&i.then),
            else_: i.else_.as_ref().map(|e| Box::new(self.convert_else(e))),
        }
    }

    pub fn convert_else(&mut self, e: &parse::Else) -> syntax::Else {
        let kind = self.convert_else_kind(&e.kind);
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

    pub fn convert_else_kind(&mut self, ek: &parse::ElseKind) -> syntax::ElseKind {
        use parse::ElseKind::*;
        match ek {
            If(i) => syntax::ElseKind::If(self.convert_if(i)),
            Block(b) => syntax::ElseKind::Block(self.convert_block(b)),
        }
    }

    pub fn convert_loop(&mut self, l: &parse::Loop) -> syntax::Loop {
        syntax::Loop {
            block: self.convert_block(&l.block),
        }
    }

    pub fn convert_while(&mut self, w: &parse::While) -> syntax::While {
        syntax::While {
            condition: Box::new(self.convert_expression(&w.condition)),
            block: self.convert_block(&w.block),
        }
    }

    pub fn convert_for(&mut self, w: &parse::For) -> syntax::For {
        syntax::For {
            var: self.convert_symbol(w.var),
            expr: Box::new(self.convert_expression(&w.expr)),
            block: self.convert_block(&w.block),
        }
    }

    pub fn convert_match(&mut self, m: &parse::Match) -> syntax::Match {
        syntax::Match {
            value: Box::new(self.convert_expression(&m.value)),
            matches: m
                .matches
                .iter()
                .map(|mi| self.convert_match_item(mi))
                .collect(),
        }
    }

    pub fn convert_match_item(&mut self, mi: &parse::MatchItem) -> syntax::MatchItem {
        let pattern = self.convert_pattern(&mi.pattern);
        let value = self.convert_expression(&mi.value);
        syntax::MatchItem {
            span: span_encompassing(pattern.span, value.span),
            pattern,
            value,
        }
    }

    pub fn convert_pattern(&mut self, p: &parse::Pattern) -> syntax::Pattern {
        use parse::Pattern::*;
        match p {
            Named(s) => syntax::Pattern {
                span: *s,
                kind: syntax::PatternKind::Named(self.convert_symbol_id(*s)),
            },
            Tuple(t) => syntax::Pattern {
                span: span_encompassing(t.open_paren_span, t.close_paren_span),
                kind: syntax::PatternKind::Tuple(self.convert_tuple_pattern(t)),
            },
            Paren(p) => self.convert_pattern(&p.pattern),
            NamedTuple(n, t) => syntax::Pattern {
                span: span_encompassing(*n, t.close_paren_span),
                kind: syntax::PatternKind::NamedTuple(
                    self.convert_symbol(*n),
                    self.convert_tuple_pattern(t),
                ),
            },
        }
    }

    fn convert_tuple_pattern(&mut self, t: &parse::TuplePattern) -> Vec<syntax::Pattern> {
        t.patterns.iter().map(|p| self.convert_pattern(p)).collect()
    }

    pub fn convert_binary(&mut self, b: &parse::Binary) -> syntax::Binary {
        syntax::Binary {
            left: Box::new(self.convert_expression(&b.left)),
            op: self.convert_binary_op(b.op.kind),
            right: Box::new(self.convert_expression(&b.right)),
        }
    }

    pub fn convert_binary_op(&mut self, tk: TokenKind) -> syntax::BinaryOp {
        match tk {
            TokenKind::Star => syntax::BinaryOp::Times,
            TokenKind::ForwardSlash => syntax::BinaryOp::DividedBy,
            TokenKind::Plus => syntax::BinaryOp::Plus,
            TokenKind::Minus => syntax::BinaryOp::Minus,
            TokenKind::Ampersand => syntax::BinaryOp::BitAnd,
            TokenKind::Bar => syntax::BinaryOp::BitOr,
            TokenKind::Equals => syntax::BinaryOp::IsEqualTo,
            TokenKind::NotEquals => syntax::BinaryOp::IsNotEqualTo,
            TokenKind::Set => syntax::BinaryOp::SetTo,
            TokenKind::And => syntax::BinaryOp::And,
            TokenKind::Or => syntax::BinaryOp::Or,
            _ => unreachable!("Token {:?} is not a binary operator", tk),
        }
    }

    pub fn convert_function_call(&mut self, fc: &parse::FunctionCall) -> syntax::FunctionCall {
        syntax::FunctionCall {
            function: Box::new(self.convert_expression(&fc.function)),
            arguments: fc
                .arguments
                .iter()
                .map(|e| self.convert_expression(e))
                .collect(),
        }
    }

    pub fn convert_bool(&mut self, kind: TokenKind) -> bool {
        match kind {
            TokenKind::True => true,
            TokenKind::False => false,
            _ => unreachable!("Token {:?} is not a boolean token", kind),
        }
    }

    pub fn convert_member_access(&mut self, ma: &parse::MemberAccess) -> syntax::MemberAccess {
        syntax::MemberAccess {
            object: Box::new(self.convert_expression(&ma.object)),
            member: self.convert_symbol(ma.member),
        }
    }

    pub fn convert_let(&mut self, l: &parse::Let) -> syntax::Let {
        syntax::Let {
            name_span: match l.name {
                Ok(s) => s,
                Err(s) => s,
            },
            name: l.name.ok().map(|n| self.convert_symbol_id(n)),
            type_: l.type_.as_ref().map(|lt| self.convert_type(&lt.type_)),
            value: l
                .value
                .as_ref()
                .map(|lv| self.convert_expression(&lv.value)),
        }
    }

    pub fn convert_block(&mut self, b: &parse::Block) -> syntax::Block {
        syntax::Block {
            statements: b
                .statements
                .iter()
                .map(|s| self.convert_statement(s))
                .collect(),
            expression: b
                .expression
                .as_ref()
                .map(|e| Box::new(self.convert_expression(e))),
        }
    }

    pub fn convert_type(&mut self, t: &parse::Type) -> syntax::Type {
        use parse::Type::*;
        match t {
            Named(parse::NamedType { name }) => syntax::Type {
                span: *name,
                kind: syntax::TypeKind::Named(self.convert_symbol(*name)),
            },
            Ref(parse::RefType {
                ref_span, type_, ..
            }) => {
                let inner = self.convert_type(&type_);
                syntax::Type {
                    span: span_encompassing(*ref_span, inner.span),
                    kind: syntax::TypeKind::Ref(Box::new(inner)),
                }
            }
            RefMut(parse::RefMutType {
                ref_span, type_, ..
            }) => {
                let inner = self.convert_type(&type_);
                syntax::Type {
                    span: span_encompassing(*ref_span, inner.span),
                    kind: syntax::TypeKind::RefMut(Box::new(inner)),
                }
            }
            PtrConst(parse::PtrConstType {
                ptr_span, type_, ..
            }) => {
                let inner = self.convert_type(&type_);
                syntax::Type {
                    span: span_encompassing(*ptr_span, inner.span),
                    kind: syntax::TypeKind::PtrConst(Box::new(inner)),
                }
            }
            PtrMut(parse::PtrMutType {
                ptr_span, type_, ..
            }) => {
                let inner = self.convert_type(&type_);
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
                kind: syntax::TypeKind::Tuple(
                    types.into_iter().map(|t| self.convert_type(t)).collect(),
                ),
            },
            Paren(parse::ParenType { type_, .. }) => self.convert_type(&type_),
            Hole(parse::HoleType { underscore_span }) => syntax::Type {
                span: *underscore_span,
                kind: syntax::TypeKind::Hole,
            },
        }
    }

    pub fn convert_symbol(&mut self, span: Span) -> syntax::Symbol {
        syntax::Symbol {
            span,
            id: self.convert_symbol_id(span),
        }
    }

    pub fn convert_symbol_id(&mut self, span: Span) -> syntax::SymbolId {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let name = self.diagnostic.file_span(span);
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        syntax::SymbolId(hasher.finish())
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

fn span_encompassing(start: Span, end: Span) -> Span {
    debug_assert_eq!(start.file, end.file);
    Span {
        file: start.file,
        start: start.start,
        end: end.end,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex::read_tokens;
    use crate::parse::parse;
    use assert_matches::assert_matches;

    #[test]
    fn test_member_call_is_converted() {
        let file_contents = "fn f() { a.b() }";
        let (tokens, eofpos) = read_tokens(0, file_contents).unwrap();
        let top_levels = parse(file_contents, &tokens, eofpos).unwrap();
        let mut diagnostic = Diagnostic::new(vec!["".to_string()]);
        diagnostic.add_file_contents(file_contents.to_string());

        let top_level = Context::new(&diagnostic).convert_top_level(&top_levels[0]);

        assert_matches!(
            top_level,
            syntax::TopLevel {
                kind: syntax::TopLevelKind::Function(syntax::Function { body, .. }),
                ..
            } => {
                let expression = body.expression.unwrap();
                assert_matches!(expression.kind, syntax::ExpressionKind::MemberCall(mc) => {
                    assert_matches!(mc.member.object.kind, syntax::ExpressionKind::Variable(_));
                    assert_eq!(mc.arguments.len(), 0);
                });
            }
        );
    }

    #[test]
    fn test_function_call_on_member() {
        let file_contents = "fn f() { (a.b)() }";
        let (tokens, eofpos) = read_tokens(0, file_contents).unwrap();
        let top_levels = parse(file_contents, &tokens, eofpos).unwrap();
        let mut diagnostic = Diagnostic::new(vec!["".to_string()]);
        diagnostic.add_file_contents(file_contents.to_string());

        let top_level = Context::new(&diagnostic).convert_top_level(&top_levels[0]);

        assert_matches!(
            top_level,
            syntax::TopLevel {
                kind: syntax::TopLevelKind::Function(syntax::Function { body, .. }),
                ..
            } => {
                let expression = body.expression.unwrap();
                assert_matches!(expression.kind, syntax::ExpressionKind::FunctionCall(mc) => {
                    assert_matches!(mc.function.kind, syntax::ExpressionKind::MemberAccess(_));
                    assert_eq!(mc.arguments.len(), 0);
                });
            }
        );
    }
}
