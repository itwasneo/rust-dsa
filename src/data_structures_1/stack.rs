pub struct Stack<T> {
    items: Vec<T>
}

impl<T> Stack<T> {

    fn new() -> Self {
        Self {
            items: Vec::<T>::new()
        }
    }

    fn push(&mut self, item: T) {
        self.items.push(item)
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

}

#[cfg(test)]
mod cool_tests {
    use super::Stack;

    #[test]
    fn empty_stack_peek_works() {
       let s: Stack<i32> = Stack::new();
       assert_eq!(None, s.peek())
    }

    #[test]
    fn empty_stack_pop_works() {
        let mut s: Stack<&str> = Stack::new();
        assert_eq!(None, s.pop());
    }

    #[test]
    fn push_pop_works() {
        let mut s: Stack<&str> = Stack::new();
        s.push("it");
        s.push("was");
        s.push("neo");
        s.pop();
        assert_eq!("was", *s.peek().unwrap())
    }

    #[test]
    fn is_empty_works() {
        let mut s: Stack<&str> = Stack::new();
        s.push("neo");
        s.pop();
        assert_eq!(true, s.is_empty());
    }

}