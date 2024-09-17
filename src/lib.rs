#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use syn::spanned::Spanned;

mod test;

const INNER_MACRO_NAME: &str = "mb";

struct DerefPattern {
    amp: Option<syn::Token![&]>,
    mutability: Option<syn::Token![mut]>,
    deref1: syn::Token![*],
    deref2: Option<syn::Token![*]>,
    pat: syn::Pat,
}
impl syn::parse::Parse for DerefPattern {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let amp: Option<syn::Token![&]> = input.parse()?;
        let mutability = if amp.is_some() { input.parse()? } else { None };
        let deref1 = input.parse()?;
        let deref2 = if amp.is_some() { input.parse()? } else { None };
        let pat = syn::Pat::parse_single(input)?;
        Ok(Self {
            amp,
            mutability,
            deref1,
            deref2,
            pat,
        })
    }
}
impl DerefPattern {
    fn as_guard_op(&self, span: proc_macro2::Span) -> proc_macro2::TokenStream {
        let Self {
            amp,
            deref1,
            deref2,
            ..
        } = self;
        match (amp, deref2) {
            (None, None) => quote::quote_spanned!(span=> & #deref1),
            (Some(amp), None) => quote::quote_spanned!(span=> #amp #deref1 *),
            (Some(amp), Some(deref2)) => quote::quote_spanned!(span=> #amp #deref2 #deref1),
            (None, Some(_)) => panic!("Invalid DerefPattern state."),
        }
    }

    fn as_body_op(&self) -> proc_macro2::TokenStream {
        let Self {
            amp,
            mutability,
            deref1,
            deref2,
            ..
        } = self;
        quote::quote!(#amp #mutability #deref1 #deref2)
    }
}

struct Bind {
    id: syn::Ident,
    deref_pat: DerefPattern,
    span: proc_macro2::Span,
}

#[derive(Default)]
struct MyFold {
    binds: Vec<Bind>,
    counter: u32,
}
impl MyFold {
    fn handle(&mut self, deref_pat: DerefPattern, span: proc_macro2::Span) -> syn::PatIdent {
        let id = syn::Ident::new(
            &format!("a{}", self.counter),
            deref_pat
                .pat
                .span()
                .resolved_at(proc_macro2::Span::mixed_site()),
        );
        self.counter += 1;
        self.binds.push(Bind {
            id: id.clone(),
            deref_pat,
            span,
        });
        syn::PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident: id,
            subpat: None,
        }
    }
}

impl syn::fold::Fold for MyFold {
    fn fold_pat(&mut self, i: syn::Pat) -> syn::Pat {
        if let syn::Pat::Macro(expr_macro) = i {
            let span = expr_macro.mac.path.span();
            let macro_name = expr_macro.mac.path.get_ident().map(ToString::to_string);

            if let Some(INNER_MACRO_NAME) = macro_name.as_deref() {
                match syn::parse2::<DerefPattern>(expr_macro.mac.tokens) {
                    Ok(mut deref_pat) => {
                        deref_pat.pat = self.fold_pat(deref_pat.pat);
                        let pat_ident = self.handle(deref_pat, span);
                        syn::Pat::Ident(pat_ident)
                    }
                    Err(err) => {
                        let compile_error = err.into_compile_error();
                        syn::parse_quote_spanned!(span=> #compile_error)
                    }
                }
            } else {
                syn::Pat::Macro(syn::fold::fold_expr_macro(self, expr_macro))
            }
        } else {
            syn::fold::fold_pat(self, i)
        }
    }
}

fn tower(binds: &[Bind], yes: syn::Expr, no: &syn::Expr, add_ref: bool) -> syn::Expr {
    let mut out = yes;
    for bind in binds {
        let Bind {
            id,
            deref_pat,
            span,
        } = bind;

        let op = if add_ref {
            deref_pat.as_guard_op(*span)
        } else {
            deref_pat.as_body_op()
        };
        let pat = &deref_pat.pat;
        out = syn::parse_quote_spanned! {*span=>
            if let #pat = #op #id {
                #out
            } else {
                #no
            }
        };
    }
    out
}

fn matchbox_impl(mut m: syn::ExprMatch) -> syn::ExprMatch {
    let mut new_arms = vec![];
    for mut arm in m.arms {
        use syn::fold::Fold;

        let span = arm.pat.span();
        let mut my_fold = MyFold::default();
        arm.pat = my_fold.fold_pat(arm.pat);

        if !my_fold.binds.is_empty() {
            let t = {
                let yes = if let Some((_if_token, src_guard)) = arm.guard {
                    *src_guard
                } else {
                    syn::parse_quote_spanned! {span=> true }
                };
                let no = syn::parse_quote_spanned! {span=> false };
                tower(&my_fold.binds, yes, &no, true)
            };
            arm.guard = Some((
                syn::Token![if](span),
                Box::new(syn::parse_quote_spanned! {span=> { #[allow(unused_variables)] #t } }),
            ));
            *arm.body = tower(
                &my_fold.binds,
                *arm.body,
                &syn::parse_quote_spanned! {span=> panic!("Two invocations of Deref::deref returned different outputs on same inputs") },
                false,
            );
        }
        new_arms.push(arm);
    }
    m.arms = new_arms;
    m
}

/// See [crate].
#[proc_macro]
pub fn matchbox(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expr = matchbox_impl(syn::parse_macro_input!(tokens as syn::ExprMatch));
    quote::quote! { #expr }.into()
}
