use std::{mem, fmt::{Debug, Display}};

/// This module describes a list which can be pushed to and iterated through
/// which is stored entirely on the stack. A user must specify maximum capacity
/// and contained type at compile time. An error is returned if attempting to push 
/// past the maximum capacity.
/// 
/// Used in-engine to store moves.

#[derive(Debug)]
pub enum ListError {
    /// Raised when attempting to push an item to the list which would overflow
    ListFull,
}

impl Display for ListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ListFull => write!(f, "The list is full.")
        }
    }
}

/// A list stored on the stack of type T up to a maximum number of items S
pub struct StackList<T: Sized, const S: usize> {
    data: [mem::MaybeUninit<T>; S],
    writer_index: usize
}

impl<T: Sized, const S: usize> StackList<T, S> {
    /// Push an item to the end of the list
    pub fn push(&mut self, item: T) -> Result<(), ListError> {
        if self.is_full() {
            return Err(ListError::ListFull)
        }

        self.data[self.writer_index].write(item);
        self.writer_index += 1;

        Ok(())
    }

    /// Return the item at the top of the stack. None if empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.writer_index -= 1;
            Some(unsafe { 
                mem::replace(
                    &mut self.data[self.writer_index],
                    mem::MaybeUninit::uninit()
                ).assume_init()
            })
        }
    }

    /// Returns true if the list is full.
    pub fn is_full(&self) -> bool {
        self.writer_index == S
    }

    /// Returns true if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.writer_index == 0
    }

    /// Get an iterator over the items in the stack.
    /// Iterates FIFO.
    pub fn iter(&self) -> StackListIter<'_, T, S> {
        StackListIter {
            list: self,
            reader_index: 0
        }
    }

    /// Initialize an empty list on the stack
    pub fn new() -> Self {
        StackList {
            data: unsafe { mem::MaybeUninit::uninit().assume_init() }, // actually an initialised list of MaybeUninit<T>s
            writer_index: 0
        }
    }
}

/// An iterator over the items in the list (Iterator)
pub struct StackListIter<'a, T: Sized, const S: usize> {
    list: &'a StackList<T, S>,
    reader_index: usize
}

impl<'a, T: Sized, const S: usize> Iterator for StackListIter<'a, T, S> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reader_index == self.list.writer_index {
            // End of list reached.
            None
        } else {
            let ret = &self.list.data[self.reader_index];

            self.reader_index += 1;

            Some(unsafe { ret.assume_init_ref() })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.list.writer_index - self.reader_index) + 1;
        (remaining, Some(remaining))
    }
}


// IntoIterator
pub struct IntoIter<T: Sized, const S: usize>(StackList<T, S>);

impl<T: Sized, const S: usize> Iterator for IntoIter<T, S> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T: Sized, const S: usize> IntoIterator for StackList<T, S> {
    type Item = T;
    type IntoIter = IntoIter<T, S>;

    /// Consume the list and produce an iterator.
    /// Iterates LIFO
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut list: StackList<i32, 3> = StackList::new();

        assert!(list.push(1).is_ok());
        assert!(list.push(2).is_ok());
        assert!(list.push(3).is_ok());
        assert!(matches!(list.push(4).unwrap_err(), ListError::ListFull)); // Uh oh! Maximum capacity reached!

        assert!(list.pop() == Some(3));
        assert!(list.pop() == Some(2));
        assert!(list.pop() == Some(1));
        assert!(list.pop().is_none()); // The list is now empty
        assert!(list.pop().is_none());
    }

    #[test]
    fn test_iter() {
        let mut list: StackList<i32, 3> = StackList::new();

        list.push(1).expect("Couldn't push to list.");
        list.push(2).expect("Couldn't push to list.");
        list.push(3).expect("Couldn't push to list.");

        let mut expected = 0;

        for i in list.iter() {
            expected += 1;
            assert!(*i == expected);
        }
    }

    #[test]
    fn test_into_iter() {
        let mut list: StackList<i32, 3> = StackList::new();

        list.push(1).expect("Couldn't push to list.");
        list.push(2).expect("Couldn't push to list.");
        list.push(3).expect("Couldn't push to list.");

        let mut expected = 4;

        for i in list.into_iter() {
            expected -= 1;
            assert!(i == expected);
        }
    }

    #[test]
    fn test_is_empty() {
        let mut list: StackList<i32, 3> = StackList::new();

        assert!(list.is_empty());
        list.push(1).expect("Couldn't push to list.");
        assert!(!list.is_empty());
        list.pop();
        assert!(list.is_empty());
    }

    #[test]
    fn test_is_full() {
        let mut list: StackList<i32, 2> = StackList::new();

        assert!(!list.is_full());
        list.push(1).expect("Couldn't push to list.");
        assert!(!list.is_full());
        list.push(1).expect("Couldn't push to list.");
        assert!(list.is_full());
        list.pop();
        assert!(!list.is_full());
    }
}