#[test]
fn tests() {
    #[deny(unused_variables)]
    {
        use std::rc::Rc;
        #[derive(PartialEq, Eq, Debug, Clone)]
        enum LispValue {
            Nil,
            Cons(Rc<LispValue>, Rc<LispValue>),
            Symbol(String),
        }
        use LispValue::*;
        let _: i32 = match_deref::match_deref!{
            match &Nil {
                Nil => 0,
                Cons(Deref @ _, Deref @ Nil) => 0,
                Cons(Deref @ Symbol(Deref @ "a"), _) => 0,
                Cons(_, _) => 0,
                Symbol(_) => 0,
            }
        };
        assert_eq!(match_deref::match_deref!{
            match &Cons(Rc::new(Symbol("a".to_owned())), Rc::new(Nil)) {
                Cons(Deref @ Symbol(Deref @ "a"), _) => 1,
                _ => 0
            }
        }, 1);
        assert_eq!(match_deref::match_deref!{
            match &Cons(Rc::new(Symbol("a".to_owned())), Rc::new(Nil)) {
                Cons(Deref @ Symbol(Deref @ "b"), _) => 1,
                _ => 0
            }
        }, 0);
        assert_eq!(match_deref::match_deref!{
            match &Cons(Rc::new(Symbol("a".to_owned())), Rc::new(Nil)) {
                Cons(Deref @ Symbol(Deref @ "a"), Deref @ x) => x,
                _ => panic!()
            }
        }, &Nil);
        assert_eq!((||{
            let _: i32 = match_deref::match_deref!{
                match &Cons(Rc::new(Symbol("a".to_owned())), Rc::new(Nil)) {
                    Cons(Deref @ Symbol(Deref @ "a"), Deref @ x) => return x.clone(),
                    _ => 0
                }
            };
            panic!();
        })(), Nil);
        {
            let a0 = 0;
            assert_eq!(match_deref::match_deref!{
                match &Rc::new(Nil) {
                    Deref @ _ => a0,
                    _ => panic!()
                }
            }, 0);
        }
        match_deref::match_deref!{
            match &Cons(Rc::new(Nil), Rc::new(Nil)) {
                Cons(a @ Deref @ b @ Nil, _) => {
                    assert_eq!(a, &Rc::new(Nil));
                    assert_eq!(b, &Nil);
                },
                _ => panic!(),
            }
        }
        assert_eq!(match_deref::match_deref!{
            match &Cons(Rc::new(Cons(Rc::new(Nil), Rc::new(Nil))), Rc::new(Nil)) {
                Cons(Deref @ a, _) => match_deref::match_deref! {
                    match a {
                        Cons(Deref @ Nil, _) => 5,
                        _ => panic!(),
                    }
                },
                _ => panic!(),
            }
        }, 5);
    }
}
