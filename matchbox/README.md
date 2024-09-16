`Deref` patterns in `match` for stable Rust. Now you can match through `Rc`, `String`, etc.

`matchbox::matchbox!{...}` is a procedural macro, which allows you to use deref patterns right now in stable Rust.

For example:
```rust,no_run
use std::rc::Rc;

enum Value {
    Nil,
    Cons(Rc<Value>, Rc<Value>),
    Symbol(String),
}

use Value::*;

let v: &Value = todo!();
matchbox::matchbox!{
    match v {
        Nil => todo!(),
        Cons(derefref!(Symbol(derefref!("quote"))), derefref!(Cons(x, derefref!(Nil)))) => todo!(),
        _ => todo!(),
    }
}
```

But there is a problem in my crate: all arms with `derefref!(...)` are ignored when compiler performs exhaustiveness checking. So sometimes you will need to add `_ => unreachable!()` to the end.

I.e. it is possible that your arms are exhaustive, but the compiler will not be able to check this. But it is not possible that you arms are not exhaustive and the compiler will falsely report them as exhaustive.

(I decided not to implement full exhaustiveness checking, because I hope that truly native support for deref patterns will be implemented in the rustc soon, so my work will be unneeded anyway. But if you want to implement similar macro with full exhaustiveness checking, go ahead, I can even link to your project here.)

The macro calls `Deref::deref` internally. Keep in mind that `Deref::deref` takes REFERENCE to smart pointer and returns REFERENCE to pointee. So this code will work: `match &Nil { derefref!(x) => ... }`, but this will not: `match Nil { derefref!(x) => ... }`.

Consider this code:
```rust,ignore
matchbox::matchbox!{
    match v {
        Symbol(derefref!(x)) => {
            // some_code_here
        }
        _ => {
            // other_code_here,
        }
    }
}
```

It will be desugared to something like this:
```rust,ignore
match v {
    Symbol(a0) if (if let x = Deref::deref(a0) { true } else { false }) => if let x = Deref::deref(a0) {
        // some_code_here
    } else {
        panic!()
    }
    _ => {
        // other_code_here
    }
}
```

My macro is hygienic, i. e. everything will work even if you use variable named `a0`

You don't need to be registered on SourceHut to create bug report.

If you think that this software is not needed or existing software already subsumes its functionality, please, tell me that, I will not be offended.
