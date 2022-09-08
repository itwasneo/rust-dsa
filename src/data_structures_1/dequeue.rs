use std::collections::VecDeque;

pub struct Dequeue<T> {
    items: VecDeque<T>
}

impl<T> Dequeue<T> {

    fn new(capacity: usize) -> Self {
        Self {
            items: VecDeque::<T>::with_capacity(capacity)
        }
    }

    fn push_front(&mut self, item: T) {
        self.items.push_front(item)
    }

    fn pop_front(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    fn push_back(&mut self, item: T) {
        self.items.push_back(item)
    }

    fn pop_back(&mut self) -> Option<T> {
        self.items.pop_back()
    }

    fn peak_head(&self) -> Option<&T> {
        self.items.front()
    }

    fn peak_tail(&self) -> Option<&T> {
        self.items.back()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn is_full(&self) -> bool {
        self.items.capacity() - self.items.len() == 1
    }
}

#[cfg(test)] 
mod cool_tests {
    use super::Dequeue;

    #[test]
    fn is_empty_works() {
        let d: Dequeue<i32> = Dequeue::new(3);
        assert_eq!(true, d.is_empty());
    }

    #[test]
    fn is_full_for_empty_works() {
        let d: Dequeue<i32> = Dequeue::new(2);
        assert_eq!(false, d.is_full());
    }

    #[test]
    fn is_full_works() {
        let mut d: Dequeue<i32> = Dequeue::new(2);
        d.push_back(3);
        d.push_back(2);
        assert_eq!(true, d.is_full());
    }

    #[test]
    fn push_front_works() {
        let mut d: Dequeue<i32> = Dequeue::new(3);
        d.push_front(3);
        d.push_front(2);
        assert_eq!(2, *d.peak_head().unwrap());
    }

    #[test]
    fn pop_front_works() {
        let mut d: Dequeue<i32> = Dequeue::new(3);
        d.push_front(3);
        d.push_front(2);
        d.push_front(1);
        d.pop_front();
        assert_eq!(2, *d.peak_head().unwrap());
    }

    #[test]
    fn push_back_works() {
        let mut d: Dequeue<i32> = Dequeue::new(3);
        d.push_front(3);
        d.push_back(2);
        d.push_front(1);
        d.pop_back();
        assert_eq!(3, *d.peak_tail().unwrap());
    }

    #[test]
    fn pop_back_works() {
        let mut d: Dequeue<i32> = Dequeue::new(3);
        d.push_front(3);
        d.push_front(2);
        d.push_front(1);
        d.pop_back();
        assert_eq!(2, *d.peak_tail().unwrap());
    }

}
