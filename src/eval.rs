use crate::ltypes::*;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    Unexpected,
    TypeMismatch,
    BadArrity,
    NotImplementation,
    NotFoundFunctionName,
    DoHeadForNil,
    UndefinedVariableReference,
    EvaluatingNonAtomHeadList,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int(i32),
    Atom(Rc<String>),
    List(Rc<List>),
}


// リスト表現
#[derive(Debug, Clone, PartialEq)]
pub enum List {
    Cons(Type, Rc<List>),
    Nil,
}

pub struct ListIterator {
    list: List,
}


impl Iterator for ListIterator {
    type Item = List;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.list.clone();
        match &self.list {
            List::Nil => {
                return None;
            }
            List::Cons(_, ref r) => {
                self.list = (**r).clone();
                return Some(res);
            }
        }
    }
}

impl IntoIterator for List {
    type Item = List;
    type IntoIter = ListIterator;
    fn into_iter(self) -> Self::IntoIter {
        ListIterator { list: self.clone() }
    }
}


impl List {
    fn try_from(l : ExpressionList, context: &mut Context) -> Result<List, EvalError> {
        match l {
            ExpressionList::Nil => {
                return Ok(List::Nil);
            }
            ExpressionList::Cons(e, left) => {
                let r = eval_with_context(e, context)?;
                let r2 = Self::try_from((*left).clone(), context)?;
                return Ok(List::Cons(r, Rc::new(r2)));
            }
        }
    }

    pub fn new() -> List {
        return List::Nil;
    }

    pub fn cons(&self, tp: &Type) -> List {
        return List::Cons(tp.clone(), Rc::new(self.clone()));
    }

    pub fn head(&self) -> Option<Type> {
        match self {
            List::Nil => {
                return None;
            }
            List::Cons(ref tp, _) => {
                return Some(tp.clone());
            }
        }
    }

    pub fn tail(&self) -> List {
        match self {
            List::Nil => return self.clone(),
            List::Cons(_, ref tail) => {
                return (**tail).clone();
            }
        }
    }
    pub fn len(&self) -> u32 {
        match self {
            List::Nil => {
                return 0;
            }
            List::Cons(_, ref tail) => {
                return tail.len() + 1;
            }
        }
    }

    // reverse自要素をreverseしたlistを返す
    pub fn reverse(&self) -> List {
        return Self::reverse_(self.clone(), List::new());
    }

    fn reverse_(old: List, new: List) -> List {
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


// exp を評価する
pub fn eval(exp: Expression) -> Result<Type, EvalError> {
    let mut context = Context::new();
    return eval_with_context(exp, &mut context);
}

// 評価時に持ち回す情報を管理する
pub struct Context {
    // TODO: vartableのvalueにType::Varが含まれることはありえないので、Type::Varを含まないようなenumを新しく定義してvalueの型としたい
    vartable: HashMap<String, Type>, // 変数テーブル
}

impl Context {
    fn new() -> Context {
        return Context {
            vartable: HashMap::new(),
        };
    }
}

// exp を、 context 付きで評価する
pub fn eval_with_context(exp: Expression, context: &mut Context) -> Result<Type, EvalError> {
    match exp {
        Expression::Int(i) => {
            return Ok(Type::Int(i));
        }
        Expression::Atom(a) => {
            return Ok(Type::Atom(a));
        }
        Expression::Var(var) => {
            if let Some(val) = context.vartable.get(&*var) {
                return Ok(val.clone());
            } else {
                return Err(EvalError::UndefinedVariableReference);
            }
        }
        Expression::ExpressionList(clist) => {
            // 組み込み関数のテーブル
            let mut embeded_fn_table: HashMap<&str, fn(List) -> Result<Type, EvalError>> =
                HashMap::new();
            embeded_fn_table.insert("add", add);
            embeded_fn_table.insert("sub", sub);
            embeded_fn_table.insert("mul", mul);
            embeded_fn_table.insert("div", div);
            embeded_fn_table.insert("list", list);
            embeded_fn_table.insert("head", head);
            embeded_fn_table.insert("tail", tail);
            embeded_fn_table.insert("gt", gt);
            embeded_fn_table.insert("lt", lt);
            embeded_fn_table.insert("eq", eq);

            // 引数を関数内部で評価する組み込み関数のテーブル
            let mut embeded_fn_table2: HashMap<
                &str,
                fn(ExpressionList, &mut Context) -> Result<Type, EvalError>,
            > = HashMap::new();
            embeded_fn_table2.insert("cond", cond);
            embeded_fn_table2.insert("set", set);
            embeded_fn_table2.insert("progn", progn);
            embeded_fn_table2.insert("while", wloop);

            // リスト形式をevalする時、先頭のatomを関数名として扱う
            if let Some(head) = clist.head() {
                if let Expression::Atom(fun_name) = head {
                    // 引数を関数内部で評価する組み込み関数の適用
                    if let Some(f) = embeded_fn_table2.get(fun_name.as_str()) {
                        let r = f(clist.tail(), context)?;
                        return Ok(r);
                    }
                    // 組み込み関数の適用
                    else if let Some(f) = embeded_fn_table.get(fun_name.as_str()) {
                        // 引数をそれぞれ評価する
                        let evaluated: List =
                            clist
                                .tail()
                                .into_iter()
                                .try_fold(List::new(), |acc, e| {
                                    let res = eval_with_context(e.head().unwrap(), context)?;
                                    Ok(acc.cons(&res))
                                })?;
                        let result = f(evaluated.reverse())?;
                        return Ok(result);
                    } else {
                        // TODO: ユーザ定義関数の適用
                        return Err(EvalError::NotFoundFunctionName);
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

// (wloop cond body) という形式の while loop
// cond が 1 である限りループを続ける
fn wloop(l: ExpressionList, context: &mut Context) -> Result<Type, EvalError> {
    if l.len() != 2 {
        return Err(EvalError::BadArrity);
    }

    let cond = l.head().unwrap();
    let body = l.tail().head().unwrap();

    loop {
        let evaluated_cond = eval_with_context(cond.clone(), context)?;
        if let Type::Int(i) = evaluated_cond {
            if i == 0 {
                // 便宜的にType::Int(0) を返す
                return Ok(Type::Int(0));
            } else {
                eval_with_context(body.clone(), context)?;
            }
        } else {
            return Err(EvalError::TypeMismatch);
        }
    }
}

// リストの要素を順番に評価する
// 最後に評価した値を戻り値とする
fn progn(l: ExpressionList, context: &mut Context) -> Result<Type, EvalError> {
    if l.len() == 0 {
        return Err(EvalError::BadArrity);
    }
    // 各要素を順番に評価していく
    let res = l.into_iter().try_fold(Type::Int(0) /* dummy */, |_, e| {
        let res = eval_with_context(e.head().unwrap(), context)?;
        return Ok(res);
    })?;
    return Ok(res);
}

// 変数に指定された値をセットする
fn set(l: ExpressionList, context: &mut Context) -> Result<Type, EvalError> {
    if l.len() != 2 {
        return Err(EvalError::BadArrity);
    }

    let var = l.head().unwrap();
    let val = eval_with_context(l.tail().head().unwrap(), context)?; // valはset関数に渡されてから評価する

    // varは Var である必要がある
    if let Expression::Var(varstr) = var {
        context.vartable.insert((*varstr).clone(), val.clone());
        return Ok(val);
    } else {
        return Err(EvalError::TypeMismatch);
    }
}

// リストを作成する
fn list(l: List) -> Result<Type, EvalError> {
    return Ok(Type::List(Rc::new(l)));
}

// リストの先頭要素を取り出す
fn head(l: List) -> Result<Type, EvalError> {
    if l.len() != 1 {
        return Err(EvalError::BadArrity);
    }
    let a = l.head().unwrap();
    if let Type::List(b) = a {
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
fn tail(l: List) -> Result<Type, EvalError> {
    if l.len() != 1 {
        return Err(EvalError::BadArrity);
    }
    let a = l.head().unwrap();
    if let Type::List(b) = a {
        return Ok(Type::List(Rc::new(b.tail())));
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
fn add(l: List) -> Result<Type, EvalError> {
    return arith_op(l, ArithType::Add);
}
// 減算を行う
fn sub(l: List) -> Result<Type, EvalError> {
    return arith_op(l, ArithType::Sub);
}
// 乗算を行う
fn mul(l: List) -> Result<Type, EvalError> {
    return arith_op(l, ArithType::Mul);
}
// 除算を行う
fn div(l: List) -> Result<Type, EvalError> {
    return arith_op(l, ArithType::Div);
}

// 加減乗除の演算を行う
fn arith_op(l: List, tp: ArithType) -> Result<Type, EvalError> {
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

// > 演算を行う
// a > b なら 1 、そうでないなら 0 を返す
// Atom同士、Int同士の場合のみ演算を許容する
fn gt(l: List) -> Result<Type, EvalError> {
    if l.len() != 2 {
        return Err(EvalError::BadArrity);
    }

    let a = l.head().unwrap();
    let b = l.tail().head().unwrap();

    if let Type::Int(aint) = a {
        if let Type::Int(bint) = b {
            let res;
            if aint > bint {
                res = 1;
            } else {
                res = 0;
            }
            return Ok(Type::Int(res));
        } else {
            return Err(EvalError::TypeMismatch);
        }
    } else if let Type::Atom(aatom) = a {
        if let Type::Atom(batom) = b {
            let res;
            if aatom > batom {
                res = 1;
            } else {
                res = 0;
            }
            return Ok(Type::Int(res));
        } else {
            return Err(EvalError::TypeMismatch);
        }
    } else {
        return Err(EvalError::TypeMismatch);
    }
}

// < 演算を行う
// a < b なら 1 、そうでないなら 0 を返す
// Atom同士、Int同士の場合のみ演算を許容する
fn lt(l: List) -> Result<Type, EvalError> {
    if l.len() != 2 {
        return Err(EvalError::BadArrity);
    }

    let a = l.head().unwrap();
    let b = l.tail().head().unwrap();

    if let Type::Int(aint) = a {
        if let Type::Int(bint) = b {
            let res = if aint < bint { 1 } else { 0 };
            return Ok(Type::Int(res));
        } else {
            return Err(EvalError::TypeMismatch);
        }
    } else if let Type::Atom(aatom) = a {
        if let Type::Atom(batom) = b {
            let res = if aatom < batom { 1 } else { 0 };
            return Ok(Type::Int(res));
        } else {
            return Err(EvalError::TypeMismatch);
        }
    } else {
        return Err(EvalError::TypeMismatch);
    }
}

// == 演算を行う
// a == b なら 1 、そうでないなら 0 を返す
// Atom同士、Int同士の場合のみ演算を許容する
fn eq(l: List) -> Result<Type, EvalError> {
    let res1 = gt(l.clone())?;
    let res2 = lt(l.clone())?;

    if res1 == Type::Int(0) && res2 == Type::Int(0) {
        return Ok(Type::Int(1));
    }
    return Ok(Type::Int(0));
}

// (条件 成立 不成立) という３つ組のリストを受け取り、
// 条件の評価結果が 0以外 である場合、成立の値を評価する
// 0である場合、不成立の値を評価する
// なお、この3つの値は、cond に渡す前に評価しないこと
// 成立か不成立どちらを実行するか、判明してから評価したいのが理由
//（条件に関しては評価しても問題ないが、一貫性のため、評価しないこととする）
fn cond(l: ExpressionList, context: &mut Context) -> Result<Type, EvalError> {
    if l.len() != 3 {
        return Err(EvalError::BadArrity);
    }

    let cond = l.head().unwrap();
    let ok = l.tail().head().unwrap();
    let ng = l.tail().tail().head().unwrap();

    let r = eval_with_context(cond, context)?;

    match r {
        Type::Int(0) => {
            return eval_with_context(ng, context);
        }
        Type::Int(_) => {
            return eval_with_context(ok, context);
        }
        _ => {
            return Err(EvalError::TypeMismatch);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::*;
    #[test]
    fn arithmetic_tests() {
        // 四則演算の関数呼び出し
        {
            let exp = Expression::try_from("(add 1 2)".as_bytes()).unwrap();
            match eval(exp) {
                Ok(Type::Int(3)) => assert!(true),
                _ => assert!(false),
            }
        }

        {
            let exp = Expression::try_from("(sub 1 2)".as_bytes()).unwrap();
            match eval(exp) {
                Ok(Type::Int(-1)) => assert!(true),
                _ => assert!(false),
            }
        }

        // 関数をネストできる
        {
            let exp = Expression::try_from("(add (add (sub 1 2) 3) 4)".as_bytes()).unwrap();
            match eval(exp) {
                Ok(Type::Int(6)) => assert!(true),
                _ => assert!(false),
            }
        }

        // 引数の数が足りない
        {
            let exp = Expression::try_from("(add 1)".as_bytes()).unwrap();
            match eval(exp) {
                Ok(_) => assert!(false),
                Err(e) => assert_eq!(EvalError::BadArrity, e),
            }
        }

        // atomが先頭要素でない
        {
            let exp = Expression::try_from("(1 2)".as_bytes()).unwrap();
            match eval(exp) {
                Ok(_) => assert!(false),
                Err(e) => assert_eq!(EvalError::EvaluatingNonAtomHeadList, e),
            }
        }
    }

    #[test]
    fn comparision_operation_tests() {
        // gt
        {
            let exp = Expression::try_from("(gt 3 2)".as_bytes()).unwrap();
            match eval(exp) {
                Ok(Type::Int(1)) => assert!(true),
                _ => assert!(false),
            }
        }
        {
            let exp = Expression::try_from("(gt 2 3)".as_bytes()).unwrap();
            match eval(exp) {
                Ok(Type::Int(0)) => assert!(true),
                _ => assert!(false),
            }
        }

        // lt
        {
            let exp = Expression::try_from("(lt 3 2)".as_bytes()).unwrap();
            match eval(exp) {
                Ok(Type::Int(0)) => assert!(true),
                _ => assert!(false),
            }
        }
        {
            let exp = Expression::try_from("(lt 2 3)".as_bytes()).unwrap();
            match eval(exp) {
                Ok(Type::Int(1)) => assert!(true),
                _ => assert!(false),
            }
        }

        // eq
        {
            let exp = Expression::try_from("(eq 3 3)".as_bytes()).unwrap();
            match eval(exp) {
                Ok(Type::Int(1)) => assert!(true),
                _ => assert!(false),
            }
        }
        {
            let exp = Expression::try_from("(eq 2 3)".as_bytes()).unwrap();
            match eval(exp) {
                Ok(Type::Int(0)) => assert!(true),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn list_tests() {
        // list
        {
            let exp = eval(Expression::try_from("(list 1 2 3)".as_bytes()).unwrap());
            assert_eq!(
                exp,
                Ok(Type::List(Rc::new(List::Cons(
                    Type::Int(1),
                    Rc::new(List::Cons(
                        Type::Int(2),
                        Rc::new(List::Cons(Type::Int(3), Rc::new(List::Nil)))
                    ))
                ))))
            );
        }
        {
            let exp = eval(Expression::try_from("(list a b c)".as_bytes()).unwrap());
            assert_eq!(
                exp,
                Ok(Type::List(Rc::new(List::Cons(
                    Type::Atom(Rc::new("a".to_string())),
                    Rc::new(List::Cons(
                        Type::Atom(Rc::new("b".to_string())),
                        Rc::new(List::Cons(
                            Type::Atom(Rc::new("c".to_string())),
                            Rc::new(List::Nil)
                        ))
                    ))
                ))))
            );
        }

        // head
        {
            let exp = eval(Expression::try_from("(head (list 10 (list 20) 30))".as_bytes()).unwrap());
            assert_eq!(exp, Ok(Type::Int(10)));
        }

        // tail
        {
            let exp = eval(Expression::try_from("(tail (list 1 2 3))".as_bytes()).unwrap());
            assert_eq!(
                exp,
                Ok(Type::List(Rc::new(List::Cons(
                    Type::Int(2),
                    Rc::new(List::Cons(Type::Int(3), Rc::new(List::Nil)))
                ))))
            );
        }
    }

    #[test]
    fn cond_tests() {
        {
            let exp = Expression::try_from("(cond (eq 3 2) 10 (mul 20 10))".as_bytes()).unwrap();
            match eval(exp) {
                Ok(Type::Int(200)) => assert!(true),
                _ => assert!(false),
            }
        }
        {
            let exp = Expression::try_from("(cond (eq 3 3) (div 10 2) 20)".as_bytes()).unwrap();
            match eval(exp) {
                Ok(Type::Int(5)) => assert!(true),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn progn_tests() {
        {
            let exp =
                Expression::try_from("(progn (set *a* 10) (add *a* (add *a* 20)))".as_bytes()).unwrap();
            match eval(exp) {
                Ok(Type::Int(40)) => assert!(true),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn while_tests() {
        {
            let exp = Expression::try_from("(progn (set *i* 0) (set *a* 0) (while (lt *i* 10) (progn (set *a* (add *i* *a*)) (set *i* (add *i* 1)))) *a*)".as_bytes()).unwrap();
            match eval(exp) {
                Ok(Type::Int(45)) => assert!(true),
                _ => assert!(false),
            }
        }
    }

}
