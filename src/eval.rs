use crate::ltypes::*;
use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    Unexpected,
    TypeMismatch,
    BadArrity,
    NotImplementation,
    EvalingNonAtomHeadList
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
            let mut embeded_fn_table : HashMap<&str, fn(LispList) -> Result<Type, EvalError >>  = HashMap::new();
            embeded_fn_table.insert("add", add);
            embeded_fn_table.insert("sub", sub);
            
            // リスト形式をevalする時、先頭のatomを関数名として扱う
            // まずは四則演算を扱いたい -> add sub mul div
            if let Some(head) = clist.head() {
                let tail = clist.tail();
                if let Type::Atom(fun_name) = head {
                    // 組み込み関数の適用
                    if let Some(f) = embeded_fn_table.get(fun_name.as_str()) {
                        let result = f(tail)?;
                        return Ok(result);
                    }
                    else{
                        // TODO: ユーザ定義関数の適用
                        return Err(EvalError::Unexpected);
                    }
                }
                // Atomが先頭要素でない場合、評価できない
                else{
                    return Err(EvalError::EvalingNonAtomHeadList);
                }
            } else {
                return Err(EvalError::Unexpected);
            }
        }
    }
}

// 加算を行う関数（型チェック付き）
fn add(l: LispList) -> Result<Type, EvalError> {

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

    let result = Type::Int(aint + bint);

    return Ok(result);
}

// 減算を行う関数（型チェック付き）
fn sub(l: LispList) -> Result<Type, EvalError> {

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

    let result = Type::Int(aint - bint);

    return Ok(result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn exp_tests() {
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
    }
}
