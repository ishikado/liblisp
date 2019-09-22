// リスト表現
enum List{
    Cons(Types, Box<List>),
    Nil
}

// 許容する型一覧
enum Types{
    Int(i32),
    Atom(String),
    List(Box<List>)
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::List;
        use super::Types;

        let _test = List::Cons(Types::Int(32), Box::new(List::Cons(Types::Atom("a".to_string()), Box::new(List::Nil))));
        let _test2 = List::Cons(Types::List(Box::new(List::Nil)), Box::new(List::Nil));


        assert_eq!(2 + 2, 4);
    }
}
