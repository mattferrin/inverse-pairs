use std::collections::VecDeque;

pub struct FixedCircularBuffer<T> {
    buffer: VecDeque<T>,
    capacity: usize,
}

impl<T> FixedCircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        FixedCircularBuffer {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push_front(&mut self, item: T) {
        if self.capacity > 0 {
            if self.buffer.len() == self.capacity {
                if !self.buffer.is_empty() {
                    let _ = self.buffer.pop_back(); // discard the oldest item
                }
            }
            self.buffer.push_front(item);
        }
    }

    pub fn front(&self) -> Option<&T> {
        self.buffer.front()
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn back(&self) -> Option<&T> {
        self.buffer.back()
    }
}

impl<T> IntoIterator for FixedCircularBuffer<T> {
    type Item = T;
    type IntoIter = std::collections::vec_deque::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a FixedCircularBuffer<T> {
    type Item = &'a T;
    type IntoIter = std::collections::vec_deque::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut FixedCircularBuffer<T> {
    type Item = &'a mut T;
    type IntoIter = std::collections::vec_deque::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_front() {
        let mut buffer = FixedCircularBuffer::new(3);

        buffer.push_front(1);
        assert_eq!(buffer.buffer, vec![1]);

        buffer.push_front(2);
        assert_eq!(buffer.buffer, vec![2, 1]);

        buffer.push_front(3);
        assert_eq!(buffer.buffer, vec![3, 2, 1]);

        buffer.push_front(4);
        assert_eq!(buffer.buffer, vec![4, 3, 2]);

        buffer.push_front(5);
        assert_eq!(buffer.buffer, vec![5, 4, 3]);

        buffer.push_front(6);
        assert_eq!(buffer.buffer, vec![6, 5, 4]);
    }

    #[test]
    fn test_push_front_with_zero_capacity() {
        let mut buffer = FixedCircularBuffer::new(0);
        buffer.push_front(1); // should do nothing
        assert_eq!(buffer.buffer, VecDeque::new());
    }

    #[test]
    fn test_push_front_with_one_capacity() {
        let mut buffer = FixedCircularBuffer::new(1);

        buffer.push_front(1);
        assert_eq!(buffer.buffer, vec![1]);

        buffer.push_front(2);
        assert_eq!(buffer.buffer, vec![2]);
    }

    #[test]
    fn test_push_front_with_large_capacity() {
        let mut buffer = FixedCircularBuffer::new(100);

        for i in 1..=100 {
            buffer.push_front(i);
            assert_eq!(buffer.buffer.len(), i);
        }
        assert_eq!(buffer.buffer, (1..=100).rev().collect::<Vec<_>>());
    }

    #[test]
    fn test_pop_back() {
        let mut buffer = FixedCircularBuffer::new(3);

        buffer.push_front(1);
        buffer.push_front(2);
        buffer.push_front(3);
        assert_eq!(buffer.buffer, vec![3, 2, 1]);

        let popped = buffer.buffer.pop_back();
        assert_eq!(popped, Some(1));
        assert_eq!(buffer.buffer, vec![3, 2]);

        let popped = buffer.buffer.pop_back();
        assert_eq!(popped, Some(2));
        assert_eq!(buffer.buffer, vec![3]);

        let popped = buffer.buffer.pop_back();
        assert_eq!(popped, Some(3));
        assert_eq!(buffer.buffer, vec![]);

        let popped = buffer.buffer.pop_back();
        assert_eq!(popped, None);
        assert_eq!(buffer.buffer, vec![]);
    }

    #[test]
    fn test_front() {
        let mut buffer = FixedCircularBuffer::new(3);

        buffer.push_front(1);
        buffer.push_front(2);
        buffer.push_front(3);

        let front = buffer.front();
        assert_eq!(front, Some(&3));

        buffer.buffer.clear();

        let front = buffer.front();
        assert_eq!(front, None);
    }

    #[test]
    fn test_back() {
        let mut buffer = FixedCircularBuffer::new(3);

        buffer.push_front(1);
        buffer.push_front(2);
        buffer.push_front(3);

        let back = buffer.back();
        assert_eq!(back, Some(&1));

        buffer.buffer.clear();

        let back = buffer.back();
        assert_eq!(back, None);
    }

    #[test]
    fn test_len() {
        let mut buffer = FixedCircularBuffer::new(3);

        assert_eq!(buffer.len(), 0);

        buffer.push_front(1);
        assert_eq!(buffer.len(), 1);

        buffer.push_front(2);
        assert_eq!(buffer.len(), 2);

        buffer.push_front(3);
        assert_eq!(buffer.len(), 3);

        buffer.push_front(4);
        assert_eq!(buffer.len(), 3);

        buffer.buffer.clear();
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn test_iteration() {
        let mut buffer = FixedCircularBuffer::new(3);

        buffer.push_front(1);
        buffer.push_front(2);
        buffer.push_front(3);

        let mut iter = buffer.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}
