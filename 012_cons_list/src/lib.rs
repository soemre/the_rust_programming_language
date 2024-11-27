use std::{fmt::Display, iter::Iterator};

#[derive(Debug)]
pub struct ConsList<T> {
    list: Box<ListItem<T>>,
}

#[derive(Debug)]
enum ListItem<T> {
    Cons(T, Box<ListItem<T>>),
    Nil,
}

impl<T> ConsList<T> {
    pub fn new() -> Self {
        Self {
            list: Box::new(ListItem::Nil),
        }
    }

    pub fn push(&mut self, item: T) {
        let mut next = &mut *self.list;
        while let ListItem::Cons(_, n) = next {
            next = n;
        }
        *next = ListItem::Cons(item, Box::new(ListItem::Nil));
    }

    pub fn merge(&mut self, other: ConsList<T>) {
        let mut next = &mut *self.list;
        while let ListItem::Cons(_, n) = next {
            next = n;
        }
        *next = *other.list;
    }

    pub fn len(&self) -> usize {
        self.into_iter().count()
    }
}

impl<T> Display for ConsList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.into_iter();

        write!(f, "{{")?;

        if let Ok(v) = iter.next().ok_or(std::fmt::Error) {
            write!(f, "{v}")?;
        }

        for v in iter {
            write!(f, ", {v}")?;
        }

        write!(f, "}}")
    }
}

impl<T, U> From<U> for ConsList<T>
where
    U: Iterator<Item = T>,
{
    fn from(value: U) -> Self {
        let mut l = ConsList::new();
        value.for_each(|item| l.push(item));
        l
    }
}

pub struct IntoIter<'a, T> {
    iter: &'a ListItem<T>,
}

impl<'a, T> Iterator for IntoIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let ListItem::Cons(v, n) = self.iter {
            self.iter = &*n;
            return Some(v);
        }
        None
    }
}

impl<'a, T> IntoIterator for &'a ConsList<T> {
    type Item = &'a T;

    type IntoIter = IntoIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        return IntoIter { iter: &self.list };
    }
}

impl<T> PartialEq for ConsList<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        for (s, o) in self.into_iter().zip(other) {
            if s != o {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let l = ConsList::from([1, 2, 3, 4, 5, 6].iter());

        assert_eq!(6, l.len());
    }

    #[test]
    fn push() {
        let mut l = ConsList::from([10, 20].iter());
        l.push(&20);

        assert_eq!(3, l.len());
    }

    #[test]
    fn display_empty() {
        assert_eq!("{}", ConsList::<i32>::new().to_string())
    }

    #[test]
    fn display() {
        let l = ConsList::from([2, 3, 5].iter());
        assert_eq!("{2, 3, 5}", l.to_string())
    }

    #[test]
    fn equal() {
        let l1 = ConsList::from([2, 3, 5].iter());
        let l2 = ConsList::from([2, 3, 5].iter());

        assert_eq!(l1, l2);
    }

    #[test]
    fn not_equal() {
        let l1 = ConsList::from([2, 3, 5].iter());
        let l2 = ConsList::from([2, 3, 2].iter());

        assert_ne!(l1, l2);
    }

    #[test]
    fn merge() {
        let mut l1 = ConsList::from([2, 3, 5].iter());
        let l2 = ConsList::from([2, 3, 2].iter());
        l1.merge(l2);

        assert_eq!(l1, ConsList::from([2, 3, 5, 2, 3, 2].iter()));
    }
}
