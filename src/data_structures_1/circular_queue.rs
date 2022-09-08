use std::collections::VecDeque;

pub struct CircularQueue<T> {
    items: VecDeque<T>
}

impl<T> CircularQueue<T> {

    fn new(capacity: usize) -> Self {
        Self {
            items: VecDeque::<T>::with_capacity(capacity),
        }
    }

    fn enqueue(&mut self, item: T) {
        self.items.push_back(item)
    }

    fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn is_full(&self) -> bool {
        self.items.capacity() - self.items.len() == 1
    }

    fn peak_head(&self) -> Option<&T> {
        self.items.front()
    }

    fn peak_tail(&self) -> Option<&T> {
        self.items.back()
    }

}

#[cfg(test)] 
mod cool_tests {
    use super::CircularQueue;

    #[test]
    fn is_empty_works() {
        let cq: CircularQueue<i32> = CircularQueue::new(5);
        assert_eq!(true, cq.is_empty());
    }

    #[test]
    fn is_full_not_true_works() {
        let mut cq: CircularQueue<i32> = CircularQueue::new(2);
        cq.enqueue(2);
        assert_eq!(false, cq.is_full());
    }

    #[test]
    fn is_full_true_works() {
        let mut cq: CircularQueue<i32> = CircularQueue::new(2);
        cq.enqueue(2);
        cq.enqueue(3);
        assert_eq!(true, cq.is_full());
    }

    #[test]
    fn enqueue_dequeue_works() {
        let mut cq: CircularQueue<i32> = CircularQueue::new(3);
        cq.enqueue(1);
        cq.enqueue(3);
        cq.enqueue(2);
        cq.dequeue();
        cq.enqueue(5);
        assert_eq!(5, *cq.peak_tail().unwrap());
        assert_eq!(3, *cq.peak_head().unwrap());
    }

    #[test]
    fn enqueue_dequeue_works_2() {
        let mut cq: CircularQueue<i32> = CircularQueue::new(3);
        cq.enqueue(1);
        cq.enqueue(3);
        cq.enqueue(2);
        cq.dequeue();
        cq.enqueue(5);
        cq.dequeue();
        assert_eq!(5, *cq.peak_tail().unwrap());
        assert_eq!(2, *cq.peak_head().unwrap());
    }

}