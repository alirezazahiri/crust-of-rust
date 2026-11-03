pub trait FlattenExt: Iterator + Sized {
    fn into_flat(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator;
}

impl<T> FlattenExt for T
where
    T: Iterator,
{
    fn into_flat(self) -> Flatten<Self>
    where
        Self: Sized,
        Self::Item: IntoIterator,
    {
        flatten(self)
    }
}

pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: IntoIterator,
    O::Item: IntoIterator,
{
    outer: O,
    head: Option<<O::Item as IntoIterator>::IntoIter>,
    tail: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: IntoIterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            head: None,
            tail: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut inner) = self.head {
            if let Some(item) = inner.next() {
                return Some(item);
            }
            self.head = None
        }

        if let Some(next_head) = self.outer.next() {
            self.head = Some(next_head.into_iter());
        } else {
            return self.tail.as_mut()?.next();
        }

        self.next()
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: Iterator + DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(ref mut inner) = self.tail {
            if let Some(item) = inner.next_back() {
                return Some(item);
            }
            self.tail = None;
        }

        if let Some(next_tail) = self.outer.next_back() {
            self.tail = Some(next_tail.into_iter());
        } else {
            return self.head.as_mut()?.next_back();
        }

        self.next_back()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0)
    }
    #[test]
    fn empty_wide() {
        assert_eq!(
            flatten(vec![Vec::<()>::new(), vec![], vec![]].into_iter()).count(),
            0
        )
    }
    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec![0])).count(), 1)
    }
    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec![0, 1])).count(), 2)
    }
    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec![0], vec![0]]).count(), 2)
    }

    #[test]
    fn reverse() {
        assert_eq!(
            flatten(std::iter::once(vec![0, 1]))
                .rev()
                .collect::<Vec<_>>(),
            vec![1, 0]
        )
    }

    #[test]
    fn reverse_wide() {
        assert_eq!(
            flatten(vec![vec![0], vec![1]]).rev().collect::<Vec<_>>(),
            vec![1, 0]
        )
    }

    #[test]
    fn both_ends() {
        let mut iter = flatten(vec![vec![0, 1, 2], vec![3, 4, 5]]);
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next_back(), Some(5));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next_back(), Some(4));
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next_back(), Some(2));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn inf() {
        let mut iter = flatten((0..).map(|i| 0..i));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
    }

    #[test]
    fn ext() {
        assert_eq!(vec![vec![1, 2]].into_iter().into_flat().count(), 2);
    }
}
