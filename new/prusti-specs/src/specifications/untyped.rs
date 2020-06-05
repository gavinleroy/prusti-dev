use super::common::{self, ExpressionIdGenerator};
use proc_macro2::{Span, TokenStream};
use std::collections::HashMap;
use syn::parse::{Parse, ParseStream};

pub use common::{ExpressionId, SpecType};

#[derive(Debug, Clone)]
pub struct Arg {
    name: syn::Ident,
    typ: syn::Type,
}

/// A specification that has no types associated with it.
pub type Specification = common::Specification<ExpressionId, syn::Expr, Arg>;
/// A set of untyped specifications associated with a single element.
pub type SpecificationSet = common::SpecificationSet<ExpressionId, syn::Expr, Arg>;
/// A map of untyped specifications for a specific crate.
pub type SpecificationMap = HashMap<common::SpecificationId, SpecificationSet>;
/// An assertion that has no types associated with it.
pub type Assertion = common::Assertion<ExpressionId, syn::Expr, Arg>;
/// An assertion kind that has no types associated with it.
pub type AssertionKind = common::AssertionKind<ExpressionId, syn::Expr, Arg>;
/// An expression that has no types associated with it.
pub type Expression = common::Expression<ExpressionId, syn::Expr>;
/// A trigger set that has not types associated with it.
pub type TriggerSet = common::TriggerSet<ExpressionId, syn::Expr>;

impl Assertion {
    pub(crate) fn true_assertion(id_generator: &mut ExpressionIdGenerator) -> Self {
        Self {
            kind: box AssertionKind::Expr(Expression {
                id: id_generator.generate(),
                expr: syn::parse_quote!(true),
            }),
        }
    }
}

impl Assertion {
    pub(crate) fn parse(
        tokens: TokenStream,
        id_generator: &mut ExpressionIdGenerator,
    ) -> syn::Result<Self> {
        let assertion: common::Assertion<(), syn::Expr, Arg> = syn::parse2(tokens)?;
        Ok(assertion.assign_id(id_generator))
    }
}

impl Parse for common::Expression<(), syn::Expr> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            id: (),
            expr: input.parse()?,
        })
    }
}

impl Parse for common::Assertion<(), syn::Expr, Arg> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // TODO: Implement parsing of the full spec. Some code can be taken from
        // here:
        // https://gitlab.inf.ethz.ch/OU-PMUELLER/prusti-dev/-/commits/new-parser/
        Ok(Self {
            kind: box common::AssertionKind::Expr(input.parse()?),
        })
    }
}

pub(crate) trait AssignExpressionId<Target> {
    fn assign_id(self, id_generator: &mut ExpressionIdGenerator) -> Target;
}

impl AssignExpressionId<Expression> for common::Expression<(), syn::Expr> {
    fn assign_id(self, id_generator: &mut ExpressionIdGenerator) -> Expression {
        Expression {
            id: id_generator.generate(),
            expr: self.expr,
        }
    }
}

impl AssignExpressionId<AssertionKind> for common::AssertionKind<(), syn::Expr, Arg> {
    fn assign_id(self, id_generator: &mut ExpressionIdGenerator) -> AssertionKind {
        use common::AssertionKind::*;
        match self {
            Expr(expr) => Expr(expr.assign_id(id_generator)),
            x => unimplemented!("{:?}", x),
        }
    }
}

impl AssignExpressionId<Box<AssertionKind>> for Box<common::AssertionKind<(), syn::Expr, Arg>> {
    fn assign_id(self, id_generator: &mut ExpressionIdGenerator) -> Box<AssertionKind> {
        box (*self).assign_id(id_generator)
    }
}

impl AssignExpressionId<Assertion> for common::Assertion<(), syn::Expr, Arg> {
    fn assign_id(self, id_generator: &mut ExpressionIdGenerator) -> Assertion {
        Assertion {
            kind: self.kind.assign_id(id_generator),
        }
    }
}