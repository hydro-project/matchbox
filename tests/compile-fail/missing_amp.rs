#[derive(PartialEq, Eq, Debug, Clone)]
enum LispValue {
    Nil,
    Cons(Box<LispValue>, Box<LispValue>),
    Symbol(String),
}
use LispValue::*;

fn main() {
    matchbox::matchbox! {
        match LispValue::Nil {
            Nil => {},
            Cons(_, _) => {},
            Symbol(mb!(** s)) => {},
            _ => panic!(),
        }
    }
}
