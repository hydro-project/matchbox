#![deny(unused_variables)]

#[derive(PartialEq, Eq, Debug, Clone)]
enum LispValue {
    Nil,
    Cons(Box<LispValue>, Box<LispValue>),
    Symbol(String),
}
use LispValue::*;

#[test]
fn tests() {
    let _: i32 = matchbox::matchbox! {
        match &Nil {
            Nil => 0,
            Cons(deref!(_), deref!(Nil)) => 0,
            Cons(deref!(Symbol(deref!("a"))), _) => 0,
            Cons(_, _) => 0,
            Symbol(_) => 0,
        }
    };
    assert_eq!(
        matchbox::matchbox! {
            match &Cons(Box::new(Symbol("a".to_owned())), Box::new(Nil)) {
                Cons(deref!(Symbol(deref!("a"))), _) => 1,
                _ => 0
            }
        },
        1
    );
    assert_eq!(
        matchbox::matchbox! {
            match &Cons(Box::new(Symbol("a".to_owned())), Box::new(Nil)) {
                Cons(deref!(Symbol(deref!("b"))), _) => 1,
                _ => 0
            }
        },
        0
    );
    assert_eq!(
        matchbox::matchbox! {
            match &Cons(Box::new(Symbol("a".to_owned())), Box::new(Nil)) {
                Cons(deref!(Symbol(deref!("a"))), deref!(x)) => x,
                _ => panic!()
            }
        },
        &Nil
    );
    assert_eq!(
        (|| {
            let _: i32 = matchbox::matchbox! {
                match &Cons(Box::new(Symbol("a".to_owned())), Box::new(Nil)) {
                    Cons(deref!(Symbol(deref!("a"))), deref!(x)) => return x.clone(),
                    _ => 0
                }
            };
            panic!();
        })(),
        Nil
    );
    {
        let a0 = 0;
        assert_eq!(
            matchbox::matchbox! {
                match &Box::new(Nil) {
                    deref!(_) => a0,
                    _ => panic!()
                }
            },
            0
        );
    }
    matchbox::matchbox! {
        match &Cons(Box::new(Nil), Box::new(Nil)) {
            Cons(a @ deref!(b @ Nil), _) => {
                assert_eq!(a, &Box::new(Nil));
                assert_eq!(b, &Nil);
            },
            _ => panic!(),
        }
    }
    assert_eq!(
        matchbox::matchbox! {
            match &Cons(Box::new(Cons(Box::new(Nil), Box::new(Nil))), Box::new(Nil)) {
                Cons(deref!(a), _) => matchbox::matchbox! {
                    match a {
                        Cons(deref!(Nil), _) => 5,
                        _ => panic!(),
                    }
                },
                _ => panic!(),
            }
        },
        5
    );
}

#[test]
fn tests_owned() {
    let _: i32 = matchbox::matchbox! {
        match Nil {
            Nil => 0,
            Cons(owned!(_), owned!(Nil)) => 0,
            Cons(owned!(Symbol(stamp!("a"))), _) => 0,
            Cons(_, _) => 0,
            Symbol(_) => 0,
        }
    };
    assert_eq!(
        matchbox::matchbox! {
            match Cons(Box::new(Symbol("a".to_owned())), Box::new(Nil)) {
                Cons(owned!(Symbol(stamp!("a"))), _) => 1,
                _ => 0
            }
        },
        1
    );
    assert_eq!(
        matchbox::matchbox! {
            match Cons(Box::new(Symbol("a".to_owned())), Box::new(Nil)) {
                Cons(owned!(Symbol(stamp!("b"))), _) => 1,
                _ => 0
            }
        },
        0
    );
    assert_eq!(
        matchbox::matchbox! {
            match Cons(Box::new(Symbol("a".to_owned())), Box::new(Nil)) {
                Cons(owned!(Symbol(stamp!("a"))), owned!(x)) => x,
                _ => panic!()
            }
        },
        Nil
    );
    assert_eq!(
        (|| {
            let _: i32 = matchbox::matchbox! {
                match Cons(Box::new(Symbol("a".to_owned())), Box::new(Nil)) {
                    Cons(owned!(Symbol(stamp!("a"))), owned!(x)) => return x.clone(),
                    _ => 0
                }
            };
            panic!();
        })(),
        Nil
    );
    {
        let a0 = 0;
        assert_eq!(
            matchbox::matchbox! {
                match Box::new(Nil) {
                    owned!(_) => a0,
                    _ => panic!(),
                }
            },
            0
        );
    }
    // matchbox::matchbox! {
    //     match &Cons(Box::new(Nil), Box::new(Nil)) {
    //         Cons(a @ deref!(b @ Nil), _) => {
    //             assert_eq!(a, &Box::new(Nil));
    //             assert_eq!(b, &Nil);
    //         },
    //         _ => panic!(),
    //     }
    // }
    assert_eq!(
        matchbox::matchbox! {
            match Cons(Box::new(Cons(Box::new(Nil), Box::new(Nil))), Box::new(Nil)) {
                Cons(owned!(a), _) => matchbox::matchbox! {
                    match a {
                        Cons(owned!(Nil), _) => 5,
                        _ => panic!(),
                    }
                },
                _ => panic!(),
            }
        },
        5
    );
}
