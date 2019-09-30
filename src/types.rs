use crate::util::*;
use std::rc::Rc;


pub type TypeList = List<Type>;

/// Lispの型一覧
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int(i32),
    Atom(Rc<String>),
    TypeList(Rc<TypeList>),
    Void,
}
