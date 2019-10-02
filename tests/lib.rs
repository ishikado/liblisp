use std::convert::TryFrom;
use liblisp::eval::*;
use liblisp::expression::*;
use liblisp::types::*;


#[test]
fn make_expression_from_string_and_eval_test() {
    // モジュール内のテストだけだと、lib.rs の内容のうち、 pub をつけなかったものまで公開されてしまうため、pub つけ忘れに気が付かない
    // そのため、ここで、公開インターフェース全体に対するテストを書くようにしたい
    let exp = Expression::try_from("(progn (set *i* 0) (set *a* 0) (while (lt *i* 10) (progn (set *a* (add *i* *a*)) (set *i* (add *i* 1)))))".as_bytes()).unwrap();
    match eval(&exp) {
        Ok(Type::Void) => assert!(true),
        _ => assert!(false),
    }

    assert!(true);
}
