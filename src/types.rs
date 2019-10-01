//!
//! Lisp の型に関する定義
//!

use crate::util::*;
use std::rc::Rc;

pub type TypeList<'a> = List<Type<'a>>;

/// Lispの型一覧
#[derive(Debug, Clone, PartialEq)]
pub enum Type<'a> {
    Int(i32),
    Atom(&'a str),
    TypeList(Rc<TypeList<'a>>),
    Void,
}
