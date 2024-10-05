use rust_smartpointers_refcell_cons_sample::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(5)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a: {:?}", a);
    println!("a: {:?}", b);
    println!("a: {:?}", c);
}
