use crate::ltypes::*;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    Unexpected,
    TypeMismatch,
    BadArrity,
    NotImplementation,
    DoHeadForNil,
    EvaluatingNonAtomHeadList,
}

// exp を評価する
pub fn eval(exp: Type) -> Result<Type, EvalError> {
    match exp {
        Type::Int(_) => {
            return Ok(exp);
        }
        Type::Atom(_) => {
            return Ok(exp);
        }
        Type::LispList(clist) => {
            // 組み込み関数のテーブル
            let mut embeded_fn_table: HashMap<&str, fn(LispList) -> Result<Type, EvalError>> =
                HashMap::new();
            embeded_fn_table.insert("add", add);
            embeded_fn_table.insert("sub", sub);
            embeded_fn_table.insert("list", list);
            embeded_fn_table.insert("head", head);
            embeded_fn_table.insert("tail", tail);

            // リスト形式をevalする時、先頭のatomを関数名として扱う
            // まずは四則演算を扱いたい -> add sub mul div
            if let Some(head) = clist.head() {
                let tail = clist.tail();
                if let Type::Atom(fun_name) = head {
                    // 組み込み関数の適用
                    if let Some(f) = embeded_fn_table.get(fun_name.as_str()) {
                        let mut evaluated = LispList::new();
                        let mut now = tail;
                        while now != LispList::Nil {
                            let c = eval(now.head().unwrap())?;
                            evaluated = evaluated.cons(&c);
                            now = now.tail();
                        }
                        evaluated = evaluated.reverse();
                        let result = f(evaluated)?;
                        return Ok(result);
                    } else {
                        // TODO: ユーザ定義関数の適用
                        return Err(EvalError::Unexpected);
                    }
                }
                // Atomが先頭要素でない場合、評価できない
                else {
                    return Err(EvalError::EvaluatingNonAtomHeadList);
                }
            } else {
                return Err(EvalError::Unexpected);
            }
        }
    }
}

// リストを作成する
fn list(l: LispList) -> Result<Type, EvalError> {
    // リストが空になるまで調べる
    let mut tmp = l;
    let mut newlist = LispList::new();
    while let Some(c) = tmp.head() {
        // 引数は必ず評価する
        newlist = newlist.cons(&c);
        tmp = tmp.tail();
    }
    newlist = newlist.reverse();
    return Ok(Type::LispList(Rc::new(newlist)));
}

// リストの先頭要素を取り出す
fn head(l: LispList) -> Result<Type, EvalError> {
    if l.len() != 1 {
        return Err(EvalError::BadArrity);
    }
    let a = l.head().unwrap();
    if let Type::LispList(b) = a {
        if let Some(c) = b.head() {
            return Ok(c);
        } else {
            return Err(EvalError::DoHeadForNil);
        }
    } else {
        return Err(EvalError::TypeMismatch);
    }
}

// リストの先頭要素外を取り除いたものを返す
fn tail(l: LispList) -> Result<Type, EvalError> {
    if l.len() != 1 {
        return Err(EvalError::BadArrity);
    }
    let a = l.head().unwrap();
    if let Type::LispList(b) = a {
        return Ok(Type::LispList(Rc::new(b.tail())));
    } else {
        return Err(EvalError::TypeMismatch);
    }
}

enum ArithType {
    Add,
    Sub,
    Mul,
    Div,
}

// 加算を行う
fn add(l: LispList) -> Result<Type, EvalError> {
    return arith_op(l, ArithType::Add);
}
// 減算を行う
fn sub(l: LispList) -> Result<Type, EvalError> {
    return arith_op(l, ArithType::Sub);
}
// 乗算を行う
fn mul(l: LispList) -> Result<Type, EvalError> {
    return arith_op(l, ArithType::Mul);
}
// 除算を行う
fn div(l: LispList) -> Result<Type, EvalError> {
    return arith_op(l, ArithType::Div);
}

// 加減乗除の演算を行う
fn arith_op(l: LispList, tp: ArithType) -> Result<Type, EvalError> {
    if l.len() != 2 {
        return Err(EvalError::BadArrity);
    }

    let a = l.head().unwrap();
    let b = l.tail().head().unwrap();

    let aint;
    let bint;

    if let Type::Int(num) = a {
        aint = num;
    } else {
        return Err(EvalError::TypeMismatch);
    }

    if let Type::Int(num) = b {
        bint = num;
    } else {
        return Err(EvalError::TypeMismatch);
    }

    let calc_result = match tp {
        ArithType::Add => aint + bint,
        ArithType::Sub => aint - bint,
        ArithType::Mul => aint * bint,
        ArithType::Div => aint / bint,
    };
    return Ok(Type::Int(calc_result));
}

#[cfg(test)]
mod tests {
    #[test]
    fn arithmetic_tests() {
        use crate::eval::*;

        // 四則演算の関数呼び出し
        {
            let exp = Type::from("(add 1 2)".as_bytes()).unwrap();
            match eval(exp) {
                Ok(Type::Int(3)) => assert!(true),
                _ => assert!(false),
            }
        }

        {
            let exp = Type::from("(sub 1 2)".as_bytes()).unwrap();
            match eval(exp) {
                Ok(Type::Int(-1)) => assert!(true),
                _ => assert!(false),
            }
        }

        // 関数をネストできる
        {
            let exp = Type::from("(add (add (sub 1 2) 3) 4)".as_bytes()).unwrap();
            match eval(exp) {
                Ok(Type::Int(6)) => assert!(true),
                _ => assert!(false),
            }
        }

        // 引数の数が足りない
        {
            let exp = Type::from("(add 1)".as_bytes()).unwrap();
            match eval(exp) {
                Ok(_) => assert!(false),
                Err(e) => assert_eq!(EvalError::BadArrity, e),
            }
        }

        // atomが先頭要素でない
        {
            let exp = Type::from("(1 2)".as_bytes()).unwrap();
            match eval(exp) {
                Ok(_) => assert!(false),
                Err(e) => assert_eq!(EvalError::EvaluatingNonAtomHeadList, e),
            }
        }
    }

    #[test]
    fn list_tests() {
        use crate::eval::*;

        // list
        {
            let exp = eval(Type::from("(list 1 2 3)".as_bytes()).unwrap());
            assert_eq!(
                exp,
                Ok(Type::LispList(Rc::new(LispList::Cons(
                    Type::Int(1),
                    Rc::new(LispList::Cons(
                        Type::Int(2),
                        Rc::new(LispList::Cons(Type::Int(3), Rc::new(LispList::Nil)))
                    ))
                ))))
            );
        }
        // head
        {
            let exp = eval(Type::from("(head (list 10 (list 20) 30))".as_bytes()).unwrap());
            assert_eq!(exp, Ok(Type::Int(10)));
        }

        // tail
        {
            let exp = eval(Type::from("(tail (list 1 2 3))".as_bytes()).unwrap());
            assert_eq!(
                exp,
                Ok(Type::LispList(Rc::new(LispList::Cons(
                    Type::Int(2),
                    Rc::new(LispList::Cons(Type::Int(3), Rc::new(LispList::Nil)))
                ))))
            );
        }
    }

}
