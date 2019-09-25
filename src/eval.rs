use crate::ltypes::*;

pub enum EvalError {
    Unexpected,
    TypeMismatch,
}

pub fn eval(exp: Type) -> Result<Type, EvalError> {
    // TODO: 実装する
    // Typeを受け取り、式を評価、結果をTypeで返す
    match exp {
        Type::Int(_) => {
            return Ok(exp);
        }
        Type::Atom(_) => {
            return Ok(exp);
        }
        Type::LispList(clist) => {
            // リスト形式をevalする時、先頭のatomを関数名として扱う
            // まずは四則演算を扱いたい -> plus minus mul div
            if let Some(fun_name) = clist.head() {
                let tail_list = clist.tail();
                // TODO: matchは組み込み関数とユーザ定義関数の２つに分けて、atomが一致する関数の探索は、matchではなく、mapから関数名をkeyとして検索できるようにしたい
                // fun_nameにmatchする関数がリストにあるか探し、もし見つかったら tail_list を引数として渡す
                match fun_name {
                    Type::Atom(ref b)
                        if &Box::new("plus".to_string()) == b && tail_list.len() == 2 =>
                    {
                        let arg1 = eval(tail_list.head().unwrap())?;
                        let arg2 = eval(tail_list.tail().head().unwrap())?;
                        let result = add(arg1, arg2)?;
                        return Ok(result);
                    }
                    Type::Atom(ref b)
                        if &Box::new("sub".to_string()) == b && tail_list.len() == 2 =>
                    {
                        let arg1 = eval(tail_list.head().unwrap())?;
                        let arg2 = eval(tail_list.tail().head().unwrap())?;
                        let result = sub(arg1, arg2)?;
                        return Ok(result);
                    }
                    _ => {
                        // TODO: ユーザ定義関数を呼ぶ
                        return Err(EvalError::Unexpected);
                    }
                }
            } else {
                return Err(EvalError::Unexpected);
            }
        }
    }
}

// 加算を行う関数（型チェック付き）
fn add(a: Type, b: Type) -> Result<Type, EvalError> {
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
fn sub(a: Type, b: Type) -> Result<Type, EvalError> {
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
            let exp = Type::from("(plus 1 2)".as_bytes()).unwrap();
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
            let exp = Type::from("(plus (plus (sub 1 2) 3) 4)".as_bytes()).unwrap();
            match eval(exp) {
                Ok(Type::Int(6)) => assert!(true),
                _ => assert!(false),
            }
        }
    }
}
