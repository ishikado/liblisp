use std::rc::Rc;


// リスト表現
#[derive(Debug, Clone, PartialEq)]
pub enum CommonList<T : Clone> {
    Cons(T, Rc<Self>),
    Nil,
}

pub struct CommonListIterator<T : Clone> {
    list: CommonList<T>,
}

impl<T : Clone> Iterator for CommonListIterator<T> {
    type Item = CommonList<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.list.clone();
        match &self.list {
            CommonList::<T>::Nil => {
                return None;
            }
            CommonList::<T>::Cons(_, ref r) => {
                self.list = (**r).clone();
                return Some(res);
            }
        }
    }
}

impl<T : Clone> IntoIterator for CommonList<T> {
    type Item = CommonList<T>;
    type IntoIter = CommonListIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        CommonListIterator::<T>{list : self.clone() }
    }
}

// リスト操作を行う関数
impl<T : Clone> CommonList<T> {
    pub fn new() -> CommonList<T> {
        return CommonList::<T>::Nil;
    }

    pub fn cons(&self, tp: &T) -> CommonList<T> {
        return CommonList::<T>::Cons(tp.clone(), Rc::new(self.clone()));
    }

    pub fn head(&self) -> Option<T> {
        match self {
            CommonList::<T>::Nil => {
                return None;
            }
            CommonList::<T>::Cons(ref tp, _) => {
                return Some(tp.clone());
            }
        }
    }

    pub fn tail(&self) -> CommonList<T> {
        match self {
            CommonList::<T>::Nil => return self.clone(),
            CommonList::<T>::Cons(_, ref tail) => {
                return (**tail).clone();
            }
        }
    }
    pub fn len(&self) -> u32 {
        match self {
            CommonList::<T>::Nil => {
                return 0;
            }
            CommonList::<T>::Cons(_, ref tail) => {
                return tail.len() + 1;
            }
        }
    }

    // reverse自要素をreverseしたlistを返す
    pub fn reverse(&self) -> CommonList<T> {
        return Self::reverse_(self.clone(), CommonList::<T>::new());
    }

    fn reverse_(old: CommonList<T>, new: CommonList<T>) -> CommonList<T> {
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
