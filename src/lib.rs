#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod test;

struct MyFold {
    binds: Vec<(syn::Ident, syn::Pat)>,
    counter: i32,
}

impl syn::fold::Fold for MyFold {
    fn fold_pat_ident(&mut self, i: syn::PatIdent) -> syn::PatIdent {
        if i.by_ref.is_some() || i.mutability.is_some() || i.ident != "Deref" {
            syn::fold::fold_pat_ident(self, i)
        } else if let Some(subpat) = i.subpat {
            let id = syn::Ident::new(
                &format!("a{}", self.counter),
                proc_macro2::Span::mixed_site(),
            );
            self.counter += 1;
            self.binds.push((id.clone(), *subpat.1));
            syn::PatIdent {
                attrs: vec![],
                by_ref: None,
                mutability: None,
                ident: id,
                subpat: None,
            }
        } else {
            syn::fold::fold_pat_ident(self, i)
        }
    }
}

fn tower(binds: &[(syn::Ident, syn::Pat)], yes: syn::Expr, no: &syn::Expr) -> syn::Expr {
    if binds.is_empty() {
        yes
    } else {
        let id = &binds[0].0;
        let pat = &binds[0].1;
        let rec = tower(&binds[1..], yes, no);
        syn::parse_quote! {
            if let #pat = ::core::ops::Deref::deref(#id) {
                #rec
            } else {
                #no
            }
        }
    }
}

fn do_match_deref(mut m: syn::ExprMatch) -> syn::ExprMatch {
    let mut new_arms = vec![];
    for mut arm in m.arms {
        use syn::fold::Fold;
        let mut my_fold = MyFold {
            binds: vec![],
            counter: 0,
        };
        arm.pat = my_fold.fold_pat(arm.pat);
        {
            let mut i = 0;
            while i < my_fold.binds.len() {
                let a = std::mem::replace(
                    &mut my_fold.binds[i].1,
                    syn::Pat::Verbatim(Default::default()),
                );
                my_fold.binds[i].1 = my_fold.fold_pat(a);
                i += 1;
            }
        }
        if !my_fold.binds.is_empty() {
            if let Some((if_token, src_guard)) = arm.guard {
                let t = tower(&my_fold.binds, *src_guard, &syn::parse_quote! { false });
                arm.guard = Some((
                    if_token,
                    Box::new(syn::parse_quote! { { #[allow(unused_variables)] #t } }),
                ));
            } else {
                let t = tower(
                    &my_fold.binds,
                    syn::parse_quote! { true },
                    &syn::parse_quote! { false },
                );
                arm.guard = Some((
                    Default::default(),
                    Box::new(syn::parse_quote! { { #[allow(unused_variables)] #t } }),
                ));
            }
            *arm.body = tower(
                &my_fold.binds,
                *arm.body,
                &syn::parse_quote! { panic!("Two invocations of Deref::deref returned different outputs on same inputs") },
            );
        }
        new_arms.push(arm);
    }
    m.arms = new_arms;
    m
}

/// See [crate].
#[proc_macro]
pub fn match_deref(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let a = do_match_deref(syn::parse_macro_input!(tokens as syn::ExprMatch));
    quote::quote! { #a }.into()
}
