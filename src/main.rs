use rust_smartpointers_refcell_cons_sample::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(5)), Rc::clone(&a));

    // Same as below
    // *(*value).borrow_mut() += 10;
    *value.borrow_mut() += 10;

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
}
