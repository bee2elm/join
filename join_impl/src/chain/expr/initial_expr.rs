//!
//! Contains `InitialExpr` definition.
//!

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Expr;

use super::InnerExpr;

///
/// Used to define expression which is the start value in chain.
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InitialExpr(pub [Expr; 1]);

impl ToTokens for InitialExpr {
    fn to_tokens(&self, output: &mut TokenStream) {
        let expr = self.get_inner_exprs().unwrap();
        let tokens = quote! { #( #expr )* };
        output.extend(tokens);
    }

    fn into_token_stream(self) -> TokenStream {
        let mut output = TokenStream::new();
        self.to_tokens(&mut output);
        output
    }
}

impl InnerExpr for InitialExpr {
    fn get_inner_exprs(&self) -> Option<&[Expr]> {
        Some(&self.0)
    }

    fn replace_inner_exprs(self, exprs: &[Expr]) -> Option<Self> {
        exprs.last().cloned().map(|expr| Self([expr]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_quote, Expr};

    fn are_streams_equal(a: TokenStream, b: TokenStream) -> bool {
        format!("{:#?}", a) == format!("{:#?}", b)
    }

    #[test]
    fn it_tests_inner_expr_trait_impl_for_err_expr() {
        let expr: Expr = parse_quote! { |v| v + 1 };

        assert_eq!(
            InitialExpr([expr.clone()]).get_inner_exprs().clone(),
            Some(&[expr][..])
        );
    }

    #[test]
    fn it_tests_inner_expr_trait_impl_replace_inner_for_initial_expr() {
        let expr: Expr = parse_quote! { |v| v + 1 };
        let replace_inner: Expr = parse_quote! { |v| 1 + v };

        assert_eq!(
            InitialExpr([expr])
                .replace_inner_exprs(&[replace_inner.clone()][..])
                .unwrap()
                .get_inner_exprs()
                .clone(),
            Some(&[replace_inner][..])
        );
    }

    #[test]
    fn it_tests_to_tokens_trait_impl_for_initial_expr() {
        let expr: Expr = parse_quote! { |v| v + 1 };

        assert!(are_streams_equal(
            InitialExpr([expr.clone()]).into_token_stream(),
            expr.into_token_stream()
        ));
    }
}
