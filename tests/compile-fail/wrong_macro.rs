#[derive(PartialEq, Eq, Debug, Clone)]
enum LispValue {
    Nil,
    Cons(Box<LispValue>, Box<LispValue>),
    Symbol(String),
}
use LispValue::*;

fn main() {
    match_box::match_box! {
        match LispValue::Nil {
            Nil => {},
            Cons(_, _) => {},
            Symbol(mob!(* s)) => {},
            _ => panic!(),
        }
    }
}
