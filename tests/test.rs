#![deny(unused_variables)]

#[derive(PartialEq, Eq, Debug, Clone)]
enum LispValue {
    Nil,
    Cons(Box<LispValue>, Box<LispValue>),
    Symbol(String),
}
use LispValue::*;

#[test]
fn test_ref() {
    let _: i32 = match_box::match_box! {
        match &Nil {
            Nil => 0,
            Cons(mb!(&** _), mb!(&** Nil)) => 0,
            Cons(mb!(&** Symbol(mb!(&** "a"))), _) => 0,
            Cons(_, _) => 0,
            Symbol(_) => 0,
        }
    };
    assert_eq!(
        match_box::match_box! {
            match &Cons(Box::new(Symbol("a".to_owned())), Box::new(Nil)) {
                Cons(mb!(&** Symbol(mb!(&** "a"))), _) => 1,
                _ => 0
            }
        },
        1
    );
    assert_eq!(
        match_box::match_box! {
            match &Cons(Box::new(Symbol("a".to_owned())), Box::new(Nil)) {
                Cons(mb!(&** Symbol(mb!(&** "b"))), _) => 1,
                _ => 0
            }
        },
        0
    );
    assert_eq!(
        match_box::match_box! {
            match &Cons(Box::new(Symbol("a".to_owned())), Box::new(Nil)) {
                Cons(mb!(&** Symbol(mb!(&** "a"))), mb!(&** x)) => x,
                _ => panic!()
            }
        },
        &Nil
    );
    assert_eq!(
        (|| {
            let _: i32 = match_box::match_box! {
                match &Cons(Box::new(Symbol("a".to_owned())), Box::new(Nil)) {
                    Cons(mb!(&** Symbol(mb!(&** "a"))), mb!(&** x)) => return x.clone(),
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
            match_box::match_box! {
                match &Box::new(Nil) {
                    mb!(&** _) => a0,
                    _ => panic!()
                }
            },
            0
        );
    }
    match_box::match_box! {
        match &Cons(Box::new(Nil), Box::new(Nil)) {
            Cons(a @ mb!(&** b @ Nil), _) => {
                assert_eq!(a, &Box::new(Nil));
                assert_eq!(b, &Nil);
            },
            _ => panic!(),
        }
    }
    assert_eq!(
        match_box::match_box! {
            match &Cons(Box::new(Cons(Box::new(Nil), Box::new(Nil))), Box::new(Nil)) {
                Cons(mb!(&** a), _) => match_box::match_box! {
                    match a {
                        Cons(mb!(&** Nil), _) => 5,
                        _ => panic!(),
                    }
                },
                _ => panic!(),
            }
        },
        5
    );
    match_box::match_box! {
        match &Box::new(Box::new(5)) {
            mb!(&** mb!(&** 5)) => {},
            _ => panic!()
        }
    };
}

#[test]
fn test_mut() {
    let _: i32 = match_box::match_box! {
        match &mut Nil {
            Nil => 0,
            Cons(mb!(&mut ** _), mb!(&mut ** x @ Nil)) => {
                *x = Symbol("foo".to_owned());
                0
            },
            Cons(mb!(&mut ** Symbol(mb!(&** "a"))), _) => 0,
            Cons(_, _) => 0,
            Symbol(_) => 0,
        }
    };
    assert_eq!(
        match_box::match_box! {
            match &mut Cons(Box::new(Symbol("a".to_owned())), Box::new(Nil)) {
                Cons(mb!(&mut ** Symbol(mb!(&** "a"))), _) => 1,
                _ => 0
            }
        },
        1
    );
    assert_eq!(
        match_box::match_box! {
            match &mut Cons(Box::new(Symbol("a".to_owned())), Box::new(Nil)) {
                Cons(mb!(&mut ** Symbol(mb!(&** "b"))), _) => 1,
                _ => 0
            }
        },
        0
    );
    assert_eq!(
        match_box::match_box! {
            match &mut Cons(Box::new(Symbol("a".to_owned())), Box::new(Nil)) {
                Cons(mb!(&mut ** Symbol(mb!(&** "a"))), mb!(&mut ** x)) => x,
                _ => panic!()
            }
        },
        &mut Nil
    );
    assert_eq!(
        (|| {
            let _: i32 = match_box::match_box! {
                match &Cons(Box::new(Symbol("a".to_owned())), Box::new(Nil)) {
                    Cons(mb!(&** Symbol(mb!(&** "a"))), mb!(&** x)) => return x.clone(),
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
            match_box::match_box! {
                match &mut Box::new(Nil) {
                    mb!(&mut ** _) => a0,
                    _ => panic!()
                }
            },
            0
        );
    }
    match_box::match_box! {
        match &mut Cons(Box::new(Nil), Box::new(Nil)) {
            Cons(mb!(&mut** b @ Nil), _) => {
                assert_eq!(b, &mut Nil);
            },
            _ => panic!(),
        }
    }
    assert_eq!(
        match_box::match_box! {
            match &mut Cons(Box::new(Cons(Box::new(Nil), Box::new(Nil))), Box::new(Nil)) {
                Cons(mb!(&mut ** a), _) => match_box::match_box! {
                    match a {
                        Cons(mb!(&mut ** Nil), _) => 5,
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
fn test_own() {
    let _: i32 = match_box::match_box! {
        match Nil {
            Nil => 0,
            Cons(mb!(* _), mb!(* Nil)) => 0,
            Cons(mb!(* Symbol(mb!(&* "a"))), _) => 0,
            Cons(_, _) => 0,
            Symbol(_) => 0,
        }
    };
    assert_eq!(
        match_box::match_box! {
            match Cons(Box::new(Symbol("a".to_owned())), Box::new(Nil)) {
                Cons(mb!(* Symbol(mb!(&* "a"))), _) => 1,
                _ => 0
            }
        },
        1
    );
    assert_eq!(
        match_box::match_box! {
            match Cons(Box::new(Symbol("a".to_owned())), Box::new(Nil)) {
                Cons(mb!(* Symbol(mb!(&* "b"))), _) => 1,
                _ => 0
            }
        },
        0
    );
    assert_eq!(
        match_box::match_box! {
            match Cons(Box::new(Symbol("a".to_owned())), Box::new(Nil)) {
                Cons(mb!(* Symbol(mb!(&* "a"))), mb!(* x)) => x,
                _ => panic!()
            }
        },
        Nil
    );
    assert_eq!(
        (|| {
            let _: i32 = match_box::match_box! {
                match Cons(Box::new(Symbol("a".to_owned())), Box::new(Nil)) {
                    Cons(mb!(* Symbol(mb!(&* "a"))), mb!(* x)) => return x.clone(),
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
            match_box::match_box! {
                match Box::new(Nil) {
                    mb!(* _) => a0,
                    _ => panic!(),
                }
            },
            0
        );
    }
    assert_eq!(
        match_box::match_box! {
            match Cons(Box::new(Cons(Box::new(Nil), Box::new(Nil))), Box::new(Nil)) {
                Cons(mb!(* a), _) => match_box::match_box! {
                    match a {
                        Cons(mb!(* Nil), _) => 5,
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
fn test_compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/*.rs");
}