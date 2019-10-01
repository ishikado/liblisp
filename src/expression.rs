//!
//! lisp構造の表現型、及び文字列からの変換関数を定義
//!

use crate::util::*;
use std::convert::TryFrom;
use std::rc::Rc;

pub type ExpressionList<'a> = List<Expression<'a>>;

/// Lispの式定義
#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    Int(i32),
    Atom(&'a str), // Expressionをcloneしたとき、Stringがcloneされるとコピーコストが大きくなる恐れがある（未検証）ので、Rcingする
    Var(&'a str),
    ExpressionList(Rc<ExpressionList<'a>>),
}

/// byte列を Expression に変換したときに発生したエラー
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionConversionError {
    InvalidToken,
    Unexpected(String),
}

impl<'a> TryFrom<&'a [u8]> for Expression<'a> {
    type Error = ExpressionConversionError;
    fn try_from(bytes: &'a [u8]) -> Result<Expression<'a>, Self::Error> {
        let mut index = 0;
        let res = Self::try_from_(&mut index, bytes);
        if index != bytes.len() {
            return Err(Self::Error::InvalidToken);
        }
        return res;
    }
}

impl<'a> Expression<'a> {
    fn try_from_(
        index: &mut usize,
        bytes: &'a [u8],
    ) -> Result<Expression<'a>, ExpressionConversionError> {
        let head_ch = char::from(bytes[*index]);
        let mut list = ExpressionList::new();
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
                    return Ok(Expression::ExpressionList(Rc::new(list.reverse())));
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
                        return Err(ExpressionConversionError::InvalidToken);
                    }
                    break;
                }
                *index += 1;
            }
            return Ok(Expression::Int(num));
        }
        // atom
        // atomは 簡単のために、alphabetから始まり、alphabetと数字のみ含むものとする
        else if head_ch.is_alphabetic() {
            let start = *index;
            while *index < bytes.len() {
                let c = char::from(bytes[*index]);
                if c.is_ascii_digit() || c.is_alphabetic() {
                } else {
                    // 括弧 or space or 改行 以外の文字が続いていたら異常
                    if !(c == ')' || c == ' ' || c == '\n') {
                        return Err(ExpressionConversionError::InvalidToken);
                    }
                    break;
                }
                *index += 1;
            }
            let end = *index;

            match std::str::from_utf8(&bytes[start..end]) {
                Ok(res) => {
                    return Ok(Expression::Atom(res));
                }
                Err(e) => {
                    // 失敗することは想定していない
                    return Err(ExpressionConversionError::Unexpected(e.to_string()));
                }
            }
        }
        // var
        // *と*で囲まれた形式を想定
        else if head_ch == '*' {
            let mut asta_count = 1;
            let start = *index;
            *index += 1;
            let second_ch = char::from(bytes[*index]);
            if second_ch.is_alphabetic() {
                while *index < bytes.len() {
                    let c = char::from(bytes[*index]);
                    if c.is_ascii_digit() || c.is_alphabetic() || c == '*' {
                        if c == '*' {
                            asta_count += 1;
                        }
                    } else {
                        // 括弧 or space or 改行 以外の文字が続いていたら異常
                        if !(c == ')' || c == ' ' || c == '\n') {
                            return Err(ExpressionConversionError::InvalidToken);
                        }
                        break;
                    }
                    *index += 1;
                }
                let end = *index;
                // bytes[start..end] の先頭と末尾のみ * が存在
                // 先頭が * になっているのは、ここ以前の条件分岐から明らかなので、末尾だけ調べる
                if asta_count == 2 && bytes[end - 1] == '*' as u8 {
                    match std::str::from_utf8(&bytes[start..end]) {
                        Ok(res) => {
                            return Ok(Expression::Var(res));
                        }
                        Err(e) => {
                            // 失敗することは想定していない
                            return Err(ExpressionConversionError::Unexpected(e.to_string()));
                        }
                    }
                } else {
                    return Err(ExpressionConversionError::InvalidToken);
                }
            } else {
                return Err(ExpressionConversionError::InvalidToken);
            }
        }
        return Err(ExpressionConversionError::InvalidToken);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn type_tests() {
        use crate::expression::*;

        assert_eq!(
            Expression::try_from("12345".as_bytes()),
            Ok(Expression::Int(12345))
        );
        assert_eq!(
            Expression::try_from("atom".as_bytes()),
            Ok(Expression::Atom("atom"))
        );
        assert_eq!(
            Expression::try_from("atom123".as_bytes()),
            Ok(Expression::Atom("atom123"))
        );
        assert_eq!(
            Expression::try_from("123atom".as_bytes()),
            Err(ExpressionConversionError::InvalidToken)
        );
        assert_eq!(
            Expression::try_from("( )".as_bytes()),
            Ok(Expression::ExpressionList(Rc::new(ExpressionList::Nil)))
        );
        assert_eq!(
            Expression::try_from("( ( ) )".as_bytes()),
            Ok(Expression::ExpressionList(Rc::new(ExpressionList::Cons(
                Expression::ExpressionList(Rc::new(ExpressionList::Nil)),
                Rc::new(ExpressionList::Nil)
            ))))
        );
        assert_eq!(
            Expression::try_from("(atom ( ) )".as_bytes()),
            Ok(Expression::ExpressionList(Rc::new(ExpressionList::Cons(
                Expression::Atom("atom"),
                Rc::new(ExpressionList::Cons(
                    Expression::ExpressionList(Rc::new(ExpressionList::Nil)),
                    Rc::new(ExpressionList::Nil)
                ))
            ))))
        );
        assert_eq!(
            Expression::try_from("*abcdefg*".as_bytes()),
            Ok(Expression::Var("*abcdefg*"))
        );

        assert_eq!(
            Expression::try_from("abc def".as_bytes()),
            Err(ExpressionConversionError::InvalidToken)
        );
        assert_eq!(
            Expression::try_from("(abc def) ()".as_bytes()),
            Err(ExpressionConversionError::InvalidToken)
        );
    }

    #[test]
    fn lisplist_tests() {
        use crate::expression::*;

        let list1 = ExpressionList::Cons(
            Expression::Int(32),
            Rc::new(ExpressionList::Cons(
                Expression::Atom("a"),
                Rc::new(ExpressionList::Nil),
            )),
        );
        let list2 = ExpressionList::Cons(
            Expression::ExpressionList(Rc::new(ExpressionList::Nil)),
            Rc::new(ExpressionList::Nil),
        );

        // len test
        assert_eq!(list1.len(), 2);
        assert_eq!(list2.len(), 1);

        // head test
        assert_eq!(list1.head(), Some(&Expression::Int(32)));

        // tail test
        assert_eq!(
            list1.tail(),
            &ExpressionList::Cons(Expression::Atom("a"), Rc::new(ExpressionList::Nil))
        );

        // cons test
        {
            let l1 = ExpressionList::Cons(Expression::Int(10), Rc::new(ExpressionList::Nil));
            assert_eq!(
                l1.cons(&Expression::Int(11)),
                ExpressionList::Cons(Expression::Int(11), Rc::new(l1))
            );
        }

        // partial_eqの挙動をついでにテスト。rcの中身もちゃんと見ている様子。
        {
            let t1 = Expression::Atom("abc");
            let t2 = Expression::Atom("abc");
            assert_eq!(t1, t2);
        }
        {
            let t1 = Expression::Atom("abc");
            let t2 = Expression::Atom("ab");
            assert_ne!(t1, t2);
        }
    }
}
