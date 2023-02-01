use proc_macro::{Span, TokenStream};
use proc_macro2::Ident;
use quote::quote;
use syn::{
    fold::{fold_expr, Fold},
    parse_quote, ExprForLoop, Lifetime,
};

struct RedirectLoopBreak;

impl RedirectLoopBreak {
    fn fold_continue(&mut self, i: syn::ExprContinue) -> syn::Expr {
        let label: Lifetime = parse_quote!('tupleforcontinue);
        match i.label {
            Some(_) => syn::Expr::Continue(i),
            None => syn::Expr::Break(syn::ExprBreak {
                attrs: i.attrs,
                break_token: parse_quote!(break),
                expr: None,
                label: Some(label),
            }),
        }
    }
}

impl Fold for RedirectLoopBreak {
    fn fold_expr_loop(&mut self, i: syn::ExprLoop) -> syn::ExprLoop {
        i
    }
    fn fold_expr_while(&mut self, i: syn::ExprWhile) -> syn::ExprWhile {
        i
    }
    fn fold_expr_for_loop(&mut self, i: ExprForLoop) -> ExprForLoop {
        i
    }
    fn fold_expr_closure(&mut self, i: syn::ExprClosure) -> syn::ExprClosure {
        i
    }

    fn fold_item(&mut self, i: syn::Item) -> syn::Item {
        i
    }

    fn fold_expr_break(&mut self, i: syn::ExprBreak) -> syn::ExprBreak {
        let label: Lifetime = parse_quote!('tupleforbreak);
        match i.label {
            Some(_) => i,
            None => syn::ExprBreak {
                label: Some(label),
                ..i
            },
        }
    }

    fn fold_expr(&mut self, i: syn::Expr) -> syn::Expr {
        match i {
            syn::Expr::Break(b) => syn::Expr::Break(self.fold_expr_break(b)),
            syn::Expr::Continue(c) => self.fold_continue(c),
            _ => fold_expr(self, i),
        }
    }
}

#[proc_macro]
pub fn generate_for_loop(stream: TokenStream) -> TokenStream {
    let letters = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L"];
    let idents: Vec<_> = letters
        .iter()
        .map(|s| Ident::new(s, Span::call_site().into()))
        .collect();

    let for_expr: ExprForLoop = match syn::parse(stream) {
        Ok(n) => n,
        Err(e) => return e.into_compile_error().into(),
    };

    let pat = for_expr.pat;
    let expr = for_expr.expr;
    let body = RedirectLoopBreak.fold_block(for_expr.body);

    quote!(
        'tupleforbreak:{
            let (#(#idents),*) = ::tuple_for::OptTuple::to_opt(#expr);
            #(
                match #idents {
                    Some(#pat) => 'tupleforcontinue:{#body},
                    None => ()
                }
            );*
        }
    )
    .into()
}
