use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum List<T>{
    Cons(Rc<RefCell<T>>, Rc<List<T>>),
    Nil,
}