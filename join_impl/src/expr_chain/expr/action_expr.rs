//!
//! Definition of `ActionExpr`, `ProcessActionExpr`, `DefaultActionExpr`.
//!

use syn::Expr;

use super::{DefaultExpr, ExtractExpr, InitialExpr, ProcessExpr};

///
/// `InstantOrDeferredExpr` defines two types of action: `Instant` and `Deferred`
///
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InstantOrDeferredExpr<Expr> {
    ///
    /// Action which will be applied to given value instantly.
    ///
    Instant(Expr),
    ///
    /// Action which will be applied after all chains have finished their actions on current step.
    ///
    Deferred(Expr),
}

///
/// `ProcessActionExpr` is `InstantOrDeferredExpr` which actions are `ProcessExpr`.
///
pub type ProcessActionExpr = InstantOrDeferredExpr<ProcessExpr>;

///
/// `DefaultActionExpr` is `InstantOrDeferredExpr` which actions are `DefaultExpr`.
///
pub type DefaultActionExpr = InstantOrDeferredExpr<DefaultExpr>;

///
/// `ActionExpr` is one of `Process`(`ProcessActionExpr`) or `Default`(`DefaultActionExpr`) expr,
/// each of which can be one of its subtypes.
///
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ActionExpr {
    ///
    /// Action of `ProcessActionExpr` type
    ///
    Process(ProcessActionExpr),
    ///
    /// Action of `DefaultActionExpr` type
    ///
    Default(DefaultActionExpr),
    ///
    /// Action of `InitialExpr` type
    ///
    Initial(InitialExpr),
}

impl ExtractExpr for ProcessActionExpr {
    type InnerExpr = ProcessExpr;

    fn extract_expr(&self) -> &Expr {
        self.extract_inner_expr().extract_expr()
    }

    fn extract_inner_expr(&self) -> &Self::InnerExpr {
        match self {
            ProcessActionExpr::Instant(expr) => expr,
            ProcessActionExpr::Deferred(expr) => expr,
        }
    }
}

impl ExtractExpr for DefaultActionExpr {
    type InnerExpr = DefaultExpr;

    fn extract_expr(&self) -> &Expr {
        self.extract_inner_expr().extract_expr()
    }

    fn extract_inner_expr(&self) -> &Self::InnerExpr {
        match self {
            DefaultActionExpr::Instant(expr) => expr,
            DefaultActionExpr::Deferred(expr) => expr,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_tests_extract_expr_trait_impl_for_process_action_expr() {
        let expr: ::syn::Expr = ::syn::parse2(::quote::quote! { |v| v + 1 }).unwrap();

        for process_action_expr in vec![
            ProcessActionExpr::Instant(ProcessExpr::Then(expr.clone())),
            ProcessActionExpr::Deferred(ProcessExpr::Then(expr.clone())),
        ]
        .into_iter()
        {
            assert_eq!(process_action_expr.extract_expr().clone(), expr);
            assert_eq!(
                process_action_expr.extract_inner_expr().clone(),
                match process_action_expr {
                    ProcessActionExpr::Instant(expr) => expr,
                    ProcessActionExpr::Deferred(expr) => expr,
                }
            );
        }
    }

    #[test]
    fn it_tests_extract_expr_trait_impl_for_default_action_expr() {
        let expr: ::syn::Expr = ::syn::parse2(::quote::quote! { |v| v + 1 }).unwrap();

        for default_action_expr in vec![
            DefaultActionExpr::Instant(DefaultExpr::Or(expr.clone())),
            DefaultActionExpr::Deferred(DefaultExpr::OrElse(expr.clone())),
        ]
        .into_iter()
        {
            assert_eq!(default_action_expr.extract_expr().clone(), expr);
            assert_eq!(
                default_action_expr.extract_inner_expr().clone(),
                match default_action_expr {
                    DefaultActionExpr::Instant(expr) => expr,
                    DefaultActionExpr::Deferred(expr) => expr,
                }
            );
        }
    }
}
