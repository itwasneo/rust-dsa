pub struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {

    fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    fn enqueue(&mut self, item: T) {
        self.items.push(item)
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.items.is_empty() {
            return None
        }
        Some(self.items.remove(0))
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn peek(&self) -> Option<&T> {
        self.items.first()
    }
}

#[cfg(test)]
mod cool_tests {
    use super::Queue;
    #[test]
    fn empty_queue_peek_works() {
        let s: Queue<i32> = Queue::new();
        assert_eq!(None, s.peek());
    }

    #[test]
    fn peek_works() {
        let mut s: Queue<i32> = Queue::new();
        s.enqueue(1);
        assert_eq!(1, *s.peek().unwrap());
    }

    #[test]
    fn empty_queue_dequeue_works() {
        let mut s: Queue<&str> = Queue::new();
        assert_eq!(None, s.dequeue());
    }

    #[test]
    fn dequeue_works() {
        let mut s: Queue<&str> = Queue::new();
        s.enqueue("it");
        assert_eq!("it", s.dequeue().unwrap());
    }

    #[test]
    fn is_empty_works() {
        let mut s: Queue<i32> = Queue::new();
        assert_eq!(true, s.is_empty());
        s.enqueue(1);
        assert_eq!(false, s.is_empty());
    }

    #[test]
    fn enqueue_works() {
        let mut s: Queue<&str> = Queue::new();
        s.enqueue("it");
        s.enqueue("was");
        s.enqueue("neo");
        assert_eq!("it", *s.peek().unwrap());
    }

    #[test]
    fn enqueue_dequeue_works() {
        let mut s: Queue<&str> = Queue::new();
        s.enqueue("it");
        s.enqueue("was");
        s.enqueue("neo");
        s.dequeue();
        s.dequeue();
        assert_eq!("neo", *s.peek().unwrap());
    }

}