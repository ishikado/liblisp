// TODO ドキュメントコメントを書く

mod lisp {

    // リスト表現
    #[derive(Debug, Clone, PartialEq)]
    pub enum LispList {
        Cons(Type, Box<LispList>),
        Nil,
    }

    // 許容する型一覧
    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        Int(i32),
        Atom(Box<String>), // Typeをcloneしたとき、Stringがcloneされるとコピーコストが大きくなる恐れがある（未検証）ので、Boxingする
        LispList(Box<LispList>),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum TypeConversionError {
        InvalidToken
    }

    // リスト操作を行う関数
    impl LispList {
        pub fn new() -> LispList {
            return LispList::Nil;
        }

        pub fn cons(&self, tp: &Type) -> LispList {
            return LispList::Cons(tp.clone(), Box::new(self.clone()));
        }

        pub fn head(&self) -> Option<Type> {
            match self {
                &LispList::Nil => {
                    return None;
                }
                &LispList::Cons(ref tp, _) => {
                    return Some(tp.clone());
                }
            }
        }

        pub fn tail(&self) -> LispList {
            match self {
                LispList::Nil => return self.clone(),
                LispList::Cons(_, ref tail) => {
                    return *tail.clone();
                }
            }
        }
        pub fn len(&self) -> u32 {
            match self {
                &LispList::Nil => {
                    return 0;
                }
                &LispList::Cons(_, ref tail) => {
                    return tail.len() + 1;
                }
            }
        }
    }

    // 文字列を受け取り、Type形式に変換する関数
    // TODO Type::from(String) など、Typeに文字列からTypeを生成する関数を impl したい
    pub fn to_type(bytes: &[u8]) -> Result<Type, TypeConversionError> {
        let mut index = 0;
        return to_type_(&mut index, bytes);
    }

    fn to_type_(index: &mut usize, bytes: &[u8]) -> Result<Type, TypeConversionError> {
        let head_ch = char::from(bytes[*index]);
        let mut list = LispList::new();
        // list
        if head_ch == '(' {
            *index += 1;
            loop {
                // spaceを飛ばす
                while *index < bytes.len() && char::from(bytes[*index]) == ' ' {
                    *index += 1;
                }

                // 終端判定
                if *index == bytes.len() {
                    // TODO : error handling
                    panic!("occured unexpected error");
                } else if char::from(bytes[*index]) == ')' {
                    // end
                    *index += 1;
                    // TODO : reverse list
                    return Ok(Type::LispList(Box::new(list)));
                }                
                
                // 新しい要素を追加
                let result = to_type_(index, bytes)?;
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
                    // 括弧 or space 以外の文字が続いていたら異常
                    if !(c == ')' || c == ' ') {
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
                    // 括弧 or space 以外の文字が続いていたら異常
                    if !(c == ')' || c == ' ') {
                        return Err(TypeConversionError::InvalidToken);
                    }
                    break;
                }
                *index += 1;
            }
            return Ok(Type::Atom(Box::new(atom)));
        }
        return Err(TypeConversionError::InvalidToken);
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn to_type_tests() {
        use super::lisp::*;
        
        assert_eq!(to_type("12345".as_bytes()), Ok(Type::Int(12345)));
        assert_eq!(to_type("atom".as_bytes()), Ok(Type::Atom(Box::new("atom".to_string()))));
        assert_eq!(to_type("atom123".as_bytes()), Ok(Type::Atom(Box::new("atom123".to_string()))));
        assert_eq!(to_type("123atom".as_bytes()), Err(TypeConversionError::InvalidToken));
        assert_eq!(to_type("( )".as_bytes()), Ok(Type::LispList(Box::new(LispList::Nil))));
        assert_eq!(to_type("( ( ) )".as_bytes()), Ok(Type::LispList(
            Box::new(LispList::Cons(Type::LispList(Box::new(LispList::Nil)), Box::new(LispList::Nil))))));
        
    }

    #[test]
    fn lisplist_tests() {
        use super::lisp::LispList;
        use super::lisp::Type;

        let list1 = LispList::Cons(
            Type::Int(32),
            Box::new(LispList::Cons(
                Type::Atom(Box::new("a".to_string())),
                Box::new(LispList::Nil),
            )),
        );
        let list2 = LispList::Cons(
            Type::LispList(Box::new(LispList::Nil)),
            Box::new(LispList::Nil),
        );

        // len test
        assert_eq!(list1.len(), 2);
        assert_eq!(list2.len(), 1);

        // head test
        assert_eq!(list1.head(), Some(Type::Int(32)));

        // tail test
        assert_eq!(
            list1.tail(),
            LispList::Cons(
                Type::Atom(Box::new("a".to_string())),
                Box::new(LispList::Nil)
            )
        );

        // cons test
        {
            let l1 = LispList::Cons(Type::Int(10), Box::new(LispList::Nil));
            assert_eq!(
                l1.cons(&Type::Int(11)),
                LispList::Cons(Type::Int(11), Box::new(l1))
            );
        }

        // partial_eqの挙動をついでにテスト。boxの中身もちゃんと見ている様子。
        {
            let t1 = Type::Atom(Box::new("abc".to_string()));
            let t2 = Type::Atom(Box::new("abc".to_string()));
            assert_eq!(t1, t2);
        }
        {
            let t1 = Type::Atom(Box::new("abc".to_string()));
            let t2 = Type::Atom(Box::new("ab".to_string()));
            assert_ne!(t1, t2);
        }
    }
}
