#![cfg(test)]

macro_rules! snapshot {
    ( $( $x:tt )* ) => {
        {
            let match_expr = crate::matchbox_impl(::syn::parse_quote! {
                $( $x )*
            });
            ::prettyplease::unparse(&::syn::parse_quote! {
                fn main() {
                    #match_expr
                }
            })
        }
    }
}

macro_rules! snapshot_test {
    ( $( $x:tt )* ) => {
        {
            ::insta::assert_snapshot!(snapshot! {
                $( $x )*
            });
        }
    };
}

#[test]
fn test_basic() {
    snapshot_test! {
        match () {
        }
    }

    // Every arm starts with 0
    snapshot_test! {
        match () {
            deref!((deref!(x),)) => {}
            Deref @ (Deref @ x,) => {}
        }
    };

    // More difficult test
    snapshot_test! {
        match () {
            deref!((deref!((deref!(x),)), deref!(y))) => {}
            Deref @ (Deref @ (Deref @ x,), Deref @ y) => {}
        }
    };
}

#[test]
fn test_basic_owned() {
    snapshot_test! {
        match () {
        }
    }

    // Every arm starts with 0
    snapshot_test! {
        match () {
            owned!((owned!(x),)) => {}
        }
    };

    // More difficult test
    snapshot_test! {
        match () {
            owned!((owned!((owned!(x),)), owned!(y))) => {}
        }
    };
}

#[test]
fn test_spelling() {
    // "Deref" works, "Dereff" doesn't
    snapshot_test! {
        match () {
            Deref @ x => ()
        }
    };
    snapshot_test! {
        match () {
            Dereff @ x => ()
        }
    };
}

#[test]
fn test_other() {
    // matchbox! doesn't insert unneded guard "if true"
    snapshot_test! {
        match () {
            Deref @ x => ()
        }
    };
    snapshot_test! {
        match () {
            x => ()
        }
    };

    snapshot_test! {
        match () {
            Deref @ Deref @ x => ()
        }
    };
    snapshot_test! {
        match &Rc::new(Nil) {
            Deref @ _ => a0,
            _ => panic!()
        }
    };
}

#[test]
fn test_bindings() {
    snapshot_test! {
        match () {
            a @ Deref @ b @ () => {},
            _ => panic!(),
        }
    }
}

#[test]
fn test_guards() {
    // Guards
    snapshot_test! {
        match () {
            Deref @ x if x == x => ()
        }
    };
    snapshot_test! {
        match () {
            x if x == x => ()
        }
    };
}
