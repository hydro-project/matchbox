#![deny(unused_variables)]
#[allow(irrefutable_let_patterns)]

#[derive(PartialEq, Eq, Debug, Clone)]
enum LispValue {
    Nil,
    Cons(Box<LispValue>, Box<LispValue>),
    Symbol(String),
}
use LispValue::*;

#[test]
fn tests() {
    match &"a".to_owned() {
        a0 if {
            #[allow(unused_variables)]
            if let &"a" = &&**a0 {
                true
            } else {
                false
            }
        } =>
        {
            if let "a" = &**a0 {
                {}
            } else {
                panic!()
            }
        }
        _ => {
            panic!()
        }
    }

    let _ = match &Nil {
        Nil => 0,
        Cons(a0, a1)
            if {
                #[allow(unused_variables)]
                if let _ = &&**a0 {
                    if let &Nil = &&**a1 {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            } =>
        {
            if let _ = &**a0 {
                if let Nil = &**a1 {
                    0
                } else {
                    panic!()
                }
            } else {
                panic!()
            }
        }
        Cons(a0, _)
            if {
                #[allow(unused_variables)]
                if let Symbol(a1) = &**a0 {
                    if let "a" = &**a1 {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            } =>
        {
            if let Symbol(a1) = &**a0 {
                if let "a" = &**a1 {
                    0
                } else {
                    panic!()
                }
            } else {
                panic!()
            }
        }
        Cons(_, _) => 0,
        Symbol(_) => 0,
    };

    // match Box::new(Box::new("a".to_owned())) {
    //     a0 if {
    //         #[allow(unused_variables)]
    //         if let a1 = &*a0 {
    //             if let a2 = &**a1 {
    //                 if let "a" = &**a2 {
    //                     true
    //                 } else {
    //                     false
    //                 }
    //             } else {
    //                 false
    //             }
    //         } else {
    //             false
    //         }
    //     } =>
    //     {
    //         if let a1 = *a0 {
    //             if let a2 = *a1 {
    //                 if let "a" = &*a2 {
    //                     {}
    //                 } else {
    //                     panic!();
    //                 }
    //             } else {
    //                 panic!();
    //             }
    //         } else {
    //             panic!();
    //         }
    //     }
    //     _ => panic!(),
    // }

    // matchbox::matchbox! {
    //     match Box::new("a".to_owned()) {
    //         owned!(deref!("a")) => {},
    //         _ => panic!(),
    //     }
    // }
}
