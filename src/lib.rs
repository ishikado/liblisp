mod lisp{

    use std::str::Chars;
    // リスト表現
    #[derive(Debug, Clone)]
    pub enum LispList{
        Cons(Type, Box<LispList>),
        Nil
    }

    // 許容する型一覧
    #[derive(Debug, Clone)]
    pub enum Type{
        Int(i32),
        Atom(String),
        LispList(Box<LispList>)
    }

    // lispモジュールのエラーを定義
    pub enum LispError {
    }

    // リスト操作を行う関数
    impl LispList {
        
        fn new() -> LispList {
            return LispList::Nil;
        }

        fn push_front(&self, tp : Type) -> LispList {
            return LispList::Cons(tp, Box::new(self.clone()));
        }

        fn head(&self) -> Option<Type> {
            match self {
                &LispList::Nil => {
                    return None;
                }
                &LispList::Cons(ref tp, _) => {
                    return Some(tp.clone());
                }
            }
        }

        fn tail(&self) -> LispList {
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
                },
                &LispList::Cons(_, ref tail) => {
                    return tail.len() + 1;
                }
            }
        }
    }

    // 文字列を受け取り、Type形式に変換する関数　
    pub fn to_type(bytes : &[u8]) -> Type {
        let mut index = 0;
        return to_type_(&mut index, bytes);
    }

    fn to_type_(index : &mut usize, bytes : &[u8]) -> Type {
        // :TODO 実装する
        // list or atom or int
        // atomは かんたんのために、alphabetから始まり、alphabetと数字のみ含むものとする
        let head_ch = char::from(bytes[*index]);
        let mut list = LispList::new();
        // list
        if head_ch == '(' {
            *index += 1;
            loop {
                let result = to_type_(index, bytes);
                list.push_front(result);

                // spaceを飛ばす
                while *index < bytes.len() && char::from(bytes[*index]) == ' ' {
                    *index+=1;
                }

                if *index == bytes.len() {
                    // TODO : error handling
                    panic!("occured unexpected error");
                }
                else if char::from(bytes[*index]) == ')' {
                    // end
                    // TODO : reverse list
                    return Type::LispList(Box::new(list));
                }
                else{
                    continue;
                }
            }
        }
        // int
        else if head_ch.is_ascii_digit() {
            let mut num : i32 = 0;
            while *index < bytes.len() {
                let c = char::from(bytes[*index]);
                if c.is_ascii_digit() {
                    // unwrapしているが、直前のif文で数字かどうかを判定しているので panic は発生しない
                    num = num * 10 + c.to_digit(10).unwrap() as i32;
                }
                else{
                    // 括弧以外の文字が続いていたら異常
                    if c != ')' {
                        // TODO erro handling
                        panic!("occured unexpected error");
                    }
                    break;
                }
            }
            return Type::Int(num);
        }
        // atom
        else if head_ch.is_alphabetic() {
            let mut atom = "".to_string();
            while *index < bytes.len() {
                let c = char::from(bytes[*index]);
                if c.is_ascii_digit() {
                    // unwrapしているが、直前のif文で数字かどうかを判定しているので panic は発生しない
                    // num = num * 10 + c.to_digit(10).unwrap() as i32;
                }
                else if c.is_alphabetic() {

                }
                else{
                    // 括弧以外の文字が続いていたら異常
                    if c != ')' {
                        // TODO erro handling
                        panic!("occured unexpected error");
                    }
                    break;
                }
            }
            return Type::Atom(atom);
        }

        return Type::Int(0);
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::lisp::LispList;
        use super::lisp::Type;

        let test = LispList::Cons(Type::Int(32), Box::new(LispList::Cons(Type::Atom("a".to_string()), Box::new(LispList::Nil))));
        let test2 = LispList::Cons(Type::LispList(Box::new(LispList::Nil)), Box::new(LispList::Nil));
        
        assert_eq!(test.len(), 2);
        assert_eq!(test2.len(), 1);
        


        assert_eq!(2 + 2, 4);
    }
}
