#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use std::str::FromStr;

use syn::spanned::Spanned;

mod test;

struct PatSingle(syn::Pat);
impl syn::parse::Parse for PatSingle {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let pat = syn::Pat::parse_single(input)?;
        Ok(Self(pat))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
enum Type {
    Owned,
    Stamp,
    Deref,
}
impl Type {
    fn add_ref(self) -> Self {
        match self {
            Self::Owned => Self::Stamp,
            Self::Stamp => Self::Deref,
            Self::Deref => Self::Deref,
        }
    }
    fn as_op(self, span: proc_macro2::Span) -> proc_macro2::TokenStream {
        match self {
            Self::Owned => quote::quote_spanned! {span=> * },
            Self::Stamp => quote::quote_spanned! {span=> &* },
            Self::Deref => quote::quote_spanned! {span=> &** },
        }
    }
}
impl FromStr for Type {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "owned" => Ok(Self::Owned),
            "stamp" => Ok(Self::Stamp),
            "deref" => Ok(Self::Deref),
            other => Err(other.to_owned()),
        }
    }
}

struct Bind {
    id: syn::Ident,
    pat: syn::Pat,
    typ: Type,
    span: proc_macro2::Span,
}

#[derive(Default)]
struct MyFold {
    binds: Vec<Bind>,
    counter: u32,
    diagnostics: Vec<proc_macro_error2::Diagnostic>,
}
impl MyFold {
    fn handle(&mut self, subpat: syn::Pat, typ: Type, span: proc_macro2::Span) -> syn::PatIdent {
        let id = syn::Ident::new(
            &format!("a{}", self.counter),
            subpat.span().resolved_at(proc_macro2::Span::mixed_site()),
        );
        self.counter += 1;
        self.binds.push(Bind {
            id: id.clone(),
            pat: subpat,
            typ,
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
            if let Some(typ @ ("deref" | "owned" | "stamp")) = expr_macro
                .mac
                .path
                .get_ident()
                .map(ToString::to_string)
                .as_deref()
            {
                match syn::parse2::<PatSingle>(expr_macro.mac.tokens) {
                    Ok(PatSingle(subpat)) => {
                        let subpat = syn::fold::fold_pat(self, subpat);
                        let typ = typ.parse().unwrap();
                        let pat_ident = self.handle(subpat, typ, span);
                        syn::Pat::Ident(pat_ident)
                    }
                    Err(err) => {
                        self.diagnostics.push(err.into());
                        syn::parse_quote_spanned!(span=> _error) // Error placeholder pattern.
                    }
                }
            } else {
                syn::Pat::Macro(syn::fold::fold_expr_macro(self, expr_macro))
            }
        } else {
            syn::fold::fold_pat(self, i)
        }
    }

    fn fold_pat_ident(&mut self, i: syn::PatIdent) -> syn::PatIdent {
        if i.by_ref.is_some() || i.mutability.is_some() || i.ident != "Deref" {
            syn::fold::fold_pat_ident(self, i)
        } else if let Some((_at, subpat)) = i.subpat {
            let subpat = syn::fold::fold_pat(self, *subpat);
            self.handle(subpat, Type::Deref, i.ident.span())
        } else {
            syn::fold::fold_pat_ident(self, i)
        }
    }
}

fn tower(binds: &[Bind], yes: syn::Expr, no: &syn::Expr, add_ref: bool) -> syn::Expr {
    let mut out = yes;
    for bind in binds {
        let &Bind {
            ref id,
            ref pat,
            mut typ,
            span,
        } = bind;

        if add_ref {
            typ = typ.add_ref();
        }
        let op: proc_macro2::TokenStream = typ.as_op(span);

        out = syn::parse_quote_spanned! {span=>
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
            let (yes, no) = if let Some((_if_token, src_guard)) = arm.guard {
                (*src_guard, syn::parse_quote_spanned! {span=> false })
            } else {
                (
                    syn::parse_quote_spanned! {span=> true },
                    syn::parse_quote_spanned! {span=> false },
                )
            };
            let t = tower(&my_fold.binds, yes, &no, true);
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
    let a = matchbox_impl(syn::parse_macro_input!(tokens as syn::ExprMatch));
    quote::quote! { #a }.into()
}
