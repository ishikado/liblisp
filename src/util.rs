use std::rc::Rc;

/// 連結リスト
#[derive(Debug, Clone, PartialEq)]
pub enum List<T: Clone> {
    Cons(T, Rc<Self>),
    Nil,
}

/// `List<T>` のイテレータ
pub struct ListIterator<T: Clone> {
    list: List<T>,
}

impl<T: Clone> Iterator for ListIterator<T> {
    type Item = List<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.list.clone();
        match &self.list {
            List::<T>::Nil => {
                return None;
            }
            List::<T>::Cons(_, ref r) => {
                self.list = (**r).clone();
                return Some(res);
            }
        }
    }
}

impl<T: Clone> IntoIterator for List<T> {
    type Item = List<T>;
    type IntoIter = ListIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        ListIterator::<T> { list: self }
    }
}

impl<T: Clone> List<T> {
    /// `List<T>` を新規作成。
    pub fn new() -> List<T> {
        return List::<T>::Nil;
    }

    /// `List<T>` の先頭に、`T` を追加する。
    pub fn cons(&self, tp: &T) -> List<T> {
        return List::<T>::Cons(tp.clone(), Rc::new(self.clone()));
    }

    /// `List<T>` の先頭要素を取り出す。
    /// もしリストが `List::<T>::Nil` の場合、`None` になる。
    pub fn head(&self) -> Option<&T> {
        match self {
            List::<T>::Nil => {
                return None;
            }
            List::<T>::Cons(tp, _) => {
                return Some(tp);
            }
        }
    }

    /// `List<T>` の先頭を取り除いた、残りの要素の `&List<T>` を返す。
    pub fn tail(&self) -> &List<T> {
        match self {
            List::<T>::Nil => return self,
            List::<T>::Cons(_, tail) => {
                return &(**tail);
            }
        }
    }

    /// `List<T>` の長さ。
    pub fn len(&self) -> u32 {
        match self {
            List::<T>::Nil => {
                return 0;
            }
            List::<T>::Cons(_, tail) => {
                return tail.len() + 1;
            }
        }
    }

    /// `List<T>` を反転したのを返す。
    pub fn reverse(&self) -> List<T> {
        return Self::reverse_(self, List::<T>::new());
    }

    fn reverse_(old: &List<T>, new: List<T>) -> List<T> {
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
