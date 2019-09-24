use crate::ltypes::*;

pub enum EvalError {
    Unexpected
}

pub fn eval(exp : Type) -> Result<Type, EvalError> {
    // TODO: 実装する
    // Typeを受け取り、式を評価、結果をTypeで返す
    match exp {
        Type::Int(_) => {
            return Ok(exp);
        },
        Type::Atom(_) => {
            return Ok(exp);
        },
        Type::LispList(l) => {
            // リスト形式をevalする時、先頭のatomを関数名として扱う
            // まずは四則演算を扱いたい -> plus minus mul div
            // return Ok(exp);
            let mut clist = l;
            if let Some(fun_name) = clist.head() {
                let tail_list = clist.tail();
                // fun_nameにmatchする関数がリストにあるか探し、もし見つかったら tail_list を引数として渡す
            }
            else{
                return Err(EvalError::Unexpected);
            }
            return Ok(Type::Int(0));
        }
    }
}
