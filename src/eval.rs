use crate::ltypes::*;
use std::collections::HashMap;

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
            embeded_fn_table.insert("head", head);
            embeded_fn_table.insert("list", list);

            // リスト形式をevalする時、先頭のatomを関数名として扱う
            // まずは四則演算を扱いたい -> add sub mul div
            if let Some(head) = clist.head() {
                let tail = clist.tail();
                if let Type::Atom(fun_name) = head {
                    // 組み込み関数の適用
                    if let Some(f) = embeded_fn_table.get(fun_name.as_str()) {
                        // TODO: tailはここで、すべての要素に対して eval を呼んでから関数 f に渡したい
                        // 現在は f の中で eval しているので、同じ実装がそれぞれの f ごとに存在してしまっている

                        let result = f(tail)?;
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
    loop {
        if let Some(c) = tmp.head() {
            // 引数は必ず評価する
            let a = eval(c)?;
            newlist = newlist.cons(&a);
            tmp = tmp.tail();
        } else {
            // headできない場合、リストが空
            break;
        }
    }
    newlist = newlist.reverse();
    return Ok(Type::LispList(Box::new(newlist)));
}

// リストの先頭要素を取り出す
fn head(l: LispList) -> Result<Type, EvalError> {
    if l.len() != 1 {
        return Err(EvalError::BadArrity);
    }
    let a = eval(l.head().unwrap())?;
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

    let a = eval(l.head().unwrap())?;
    let b = eval(l.tail().head().unwrap())?;

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
                Ok(Type::LispList(Box::new(LispList::Cons(
                    Type::Int(1),
                    Box::new(LispList::Cons(
                        Type::Int(2),
                        Box::new(LispList::Cons(Type::Int(3), Box::new(LispList::Nil)))
                    ))
                ))))
            );
        }
        // head
        {
            let exp = eval(Type::from("(head (list 10 20 30))".as_bytes()).unwrap());
            assert_eq!(exp, Ok(Type::Int(10)));
        }
    }

}
