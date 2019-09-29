// liblispで、lisp構造の表現に用いる型定義

use std::convert::TryFrom;
use std::rc::Rc;

// リスト表現
#[derive(Debug, Clone, PartialEq)]
pub enum LispList {
    Cons(Type, Rc<LispList>),
    Nil,
}

pub struct LispListIterator {
    list: LispList,
}

impl Iterator for LispListIterator {
    type Item = LispList;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.list.clone();
        match &self.list {
            LispList::Nil => {
                return None;
            }
            LispList::Cons(_, ref r) => {
                self.list = (**r).clone();
                return Some(res);
            }
        }
    }
}

impl IntoIterator for LispList {
    type Item = LispList;
    type IntoIter = LispListIterator;
    fn into_iter(self) -> Self::IntoIter {
        LispListIterator { list: self.clone() }
    }
}

// 許容する型一覧
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int(i32),
    Atom(Rc<String>), // Typeをcloneしたとき、Stringがcloneされるとコピーコストが大きくなる恐れがある（未検証）ので、Rcingする
    Var(Rc<String>),
    LispList(Rc<LispList>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeConversionError {
    InvalidToken,
}

// リスト操作を行う関数
impl LispList {
    pub fn new() -> LispList {
        return LispList::Nil;
    }

    pub fn cons(&self, tp: &Type) -> LispList {
        return LispList::Cons(tp.clone(), Rc::new(self.clone()));
    }

    pub fn head(&self) -> Option<Type> {
        match self {
            LispList::Nil => {
                return None;
            }
            LispList::Cons(ref tp, _) => {
                return Some(tp.clone());
            }
        }
    }

    pub fn tail(&self) -> LispList {
        match self {
            LispList::Nil => return self.clone(),
            LispList::Cons(_, ref tail) => {
                return (**tail).clone();
            }
        }
    }
    pub fn len(&self) -> u32 {
        match self {
            LispList::Nil => {
                return 0;
            }
            LispList::Cons(_, ref tail) => {
                return tail.len() + 1;
            }
        }
    }

    // reverse自要素をreverseしたlistを返す
    pub fn reverse(&self) -> LispList {
        return Self::reverse_(self.clone(), LispList::new());
    }

    fn reverse_(old: LispList, new: LispList) -> LispList {
        match old.head() {
            None => {
                return new;
            }
            Some(hd) => {
                return Self::reverse_(old.tail(), new.cons(&hd));
            }
        }
    }
}

impl TryFrom<&[u8]> for Type {
    type Error = TypeConversionError;
    fn try_from(bytes: &[u8]) -> Result<Type, Self::Error> {
        let mut index = 0;
        let res = Self::try_from_(&mut index, bytes);
        if index != bytes.len() {
            return Err(Self::Error::InvalidToken);
        }
        return res;
    }
}

impl Type {
    fn try_from_(index: &mut usize, bytes: &[u8]) -> Result<Type, TypeConversionError> {
        let head_ch = char::from(bytes[*index]);
        let mut list = LispList::new();
        // list
        if head_ch == '(' {
            *index += 1;
            loop {
                // space or \n を飛ばす
                while *index < bytes.len()
                    && (char::from(bytes[*index]) == ' ' || char::from(bytes[*index]) == '\n')
                {
                    *index += 1;
                }

                // 終端判定
                if *index == bytes.len() {
                    // TODO : error handling
                    panic!("occured unexpected error");
                } else if char::from(bytes[*index]) == ')' {
                    // end
                    *index += 1;
                    return Ok(Type::LispList(Rc::new(list.reverse())));
                }

                // 新しい要素を追加
                let result = Self::try_from_(index, bytes)?;
                list = list.cons(&result);
            }
        }
        // int
        else if head_ch.is_ascii_digit() {
            let mut num: i32 = 0;
            while *index < bytes.len() {
                let c = char::from(bytes[*index]);
                if c.is_ascii_digit() {
                    // unwrapしているが、直前のif文で数字かどうかを判定しているので panic は発生しない
                    num = num * 10 + c.to_digit(10).unwrap() as i32;
                } else {
                    // 括弧 or space or 改行 以外の文字が続いていたら異常
                    if !(c == ')' || c == ' ' || c == '\n') {
                        return Err(TypeConversionError::InvalidToken);
                    }
                    break;
                }
                *index += 1;
            }
            return Ok(Type::Int(num));
        }
        // atom
        // atomは 簡単のために、alphabetから始まり、alphabetと数字のみ含むものとする
        else if head_ch.is_alphabetic() {
            let mut atom = "".to_string();
            while *index < bytes.len() {
                let c = char::from(bytes[*index]);
                if c.is_ascii_digit() || c.is_alphabetic() {
                    atom.push(c);
                } else {
                    // 括弧 or space or 改行 以外の文字が続いていたら異常
                    if !(c == ')' || c == ' ' || c == '\n') {
                        return Err(TypeConversionError::InvalidToken);
                    }
                    break;
                }
                *index += 1;
            }
            return Ok(Type::Atom(Rc::new(atom)));
        }
        // var
        // *と*で囲まれた形式を想定
        else if head_ch == '*' {
            let mut var = "*".to_string();
            let mut asta_count = 1;
            *index += 1;
            let second_ch = char::from(bytes[*index]);
            if second_ch.is_alphabetic() {
                while *index < bytes.len() {
                    let c = char::from(bytes[*index]);
                    if c.is_ascii_digit() || c.is_alphabetic() || c == '*' {
                        var.push(c);
                        if c == '*' {
                            asta_count += 1;
                        }
                    } else {
                        // 括弧 or space or 改行 以外の文字が続いていたら異常
                        if !(c == ')' || c == ' ' || c == '\n') {
                            return Err(TypeConversionError::InvalidToken);
                        }
                        break;
                    }
                    *index += 1;
                }
                // varの先頭と末尾のみ * が存在
                // 先頭が * になっているのは、ここ以前の条件分岐から明らかなので、末尾だけ調べる
                let var_len = var.as_bytes().len();
                if asta_count == 2 && char::from(var.as_bytes()[var_len - 1]) == '*' {
                    return Ok(Type::Var(Rc::new(var)));
                } else {
                    return Err(TypeConversionError::InvalidToken);
                }
            } else {
                return Err(TypeConversionError::InvalidToken);
            }
        }
        return Err(TypeConversionError::InvalidToken);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn type_tests() {
        use crate::ltypes::*;

        assert_eq!(Type::try_from("12345".as_bytes()), Ok(Type::Int(12345)));
        assert_eq!(
            Type::try_from("atom".as_bytes()),
            Ok(Type::Atom(Rc::new("atom".to_string())))
        );
        assert_eq!(
            Type::try_from("atom123".as_bytes()),
            Ok(Type::Atom(Rc::new("atom123".to_string())))
        );
        assert_eq!(
            Type::try_from("123atom".as_bytes()),
            Err(TypeConversionError::InvalidToken)
        );
        assert_eq!(
            Type::try_from("( )".as_bytes()),
            Ok(Type::LispList(Rc::new(LispList::Nil)))
        );
        assert_eq!(
            Type::try_from("( ( ) )".as_bytes()),
            Ok(Type::LispList(Rc::new(LispList::Cons(
                Type::LispList(Rc::new(LispList::Nil)),
                Rc::new(LispList::Nil)
            ))))
        );
        assert_eq!(
            Type::try_from("(atom ( ) )".as_bytes()),
            Ok(Type::LispList(Rc::new(LispList::Cons(
                Type::Atom(Rc::new("atom".to_string())),
                Rc::new(LispList::Cons(
                    Type::LispList(Rc::new(LispList::Nil)),
                    Rc::new(LispList::Nil)
                ))
            ))))
        );
        assert_eq!(
            Type::try_from("*abcdefg*".as_bytes()),
            Ok(Type::Var(Rc::new("*abcdefg*".to_string())))
        );

        assert_eq!(Type::try_from("abc def".as_bytes()),
                   Err(TypeConversionError::InvalidToken));
        assert_eq!(Type::try_from("(abc def) ()".as_bytes()),
                   Err(TypeConversionError::InvalidToken));

    }

    #[test]
    fn lisplist_tests() {
        use crate::ltypes::*;

        let list1 = LispList::Cons(
            Type::Int(32),
            Rc::new(LispList::Cons(
                Type::Atom(Rc::new("a".to_string())),
                Rc::new(LispList::Nil),
            )),
        );
        let list2 = LispList::Cons(
            Type::LispList(Rc::new(LispList::Nil)),
            Rc::new(LispList::Nil),
        );

        // len test
        assert_eq!(list1.len(), 2);
        assert_eq!(list2.len(), 1);

        // head test
        assert_eq!(list1.head(), Some(Type::Int(32)));

        // tail test
        assert_eq!(
            list1.tail(),
            LispList::Cons(Type::Atom(Rc::new("a".to_string())), Rc::new(LispList::Nil))
        );

        // cons test
        {
            let l1 = LispList::Cons(Type::Int(10), Rc::new(LispList::Nil));
            assert_eq!(
                l1.cons(&Type::Int(11)),
                LispList::Cons(Type::Int(11), Rc::new(l1))
            );
        }

        // partial_eqの挙動をついでにテスト。rcの中身もちゃんと見ている様子。
        {
            let t1 = Type::Atom(Rc::new("abc".to_string()));
            let t2 = Type::Atom(Rc::new("abc".to_string()));
            assert_eq!(t1, t2);
        }
        {
            let t1 = Type::Atom(Rc::new("abc".to_string()));
            let t2 = Type::Atom(Rc::new("ab".to_string()));
            assert_ne!(t1, t2);
        }
    }
}
