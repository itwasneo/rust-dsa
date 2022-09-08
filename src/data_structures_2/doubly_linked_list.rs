use std::fmt::Debug;

#[derive(PartialEq, Debug, Clone)]
struct Link<T>
where T: Clone {
    value: T,
    next: Option<Box<Link<T>>>,
    previous: Option<Box<Link<T>>>,
}

#[derive(PartialEq, Clone)]
struct DoublyLinkedList<T>
where T: Clone {
    head: Option<Box<Link<T>>>,
    tail: Option<Box<Link<T>>>,
}

impl<T> DoublyLinkedList<T>
where T: Debug + Clone{

    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push_back(&mut self, mut link: Box<Link<T>>) {
        link.next = None;
        link.previous = self.tail.clone();

        match &mut self.tail {
            None => self.head = Some(link.clone()),
            Some(tail) => tail.next = Some(link.clone())
        }
        self.tail = Some(link);
    }

    fn push_front(&mut self, mut link: Box<Link<T>>) {
        link.next = self.head.clone();
        link.previous = None;

        match &mut self.head {
            None => self.tail = Some(link.clone()),
            Some(head) => head.previous = Some(link.clone())
        }

        self.head = Some(link);
    }

    fn pop_back(&mut self) {

        match &self.tail {
            None => self.head = None,
            Some(tail) => {
                let link = tail.clone();
                self.tail = link.previous;
            }
        }

        match &mut self.tail {
            None => self.head = None,
            Some(tail) => {
                tail.next = None;
            }
        }
    }

    fn pop_front(&mut self) {
        match &self.head {
            None => self.tail = None,
            Some(head) => {
                let link = head.clone();
                self.head = link.next;
            }
        }

        match &mut self.head {
            None => self.tail = None,
            Some(head) => head.next = None
        }
    }

}


#[cfg(test)]
mod cool_tests {
    use super::{DoublyLinkedList, Link};

    #[test]
    fn new_works() {
        let dll: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert_eq!(None, dll.head);
    }

    #[test]
    fn push_back_works() {
        let mut dll: DoublyLinkedList<i32> = DoublyLinkedList::new();
        dll.push_back(Box::<Link<i32>>::new(
            Link { value: 1, next: None, previous: None }
        ));
        dll.push_back(Box::<Link<i32>>::new(
            Link { value: 5, next: None, previous: None }
        ));
        assert_eq!(1, dll.head.unwrap().value);
        assert_eq!(5, dll.tail.unwrap().value);
    }

    #[test]
    fn  push_font_works() {
        let mut dll: DoublyLinkedList<i32> = DoublyLinkedList::new();
        dll.push_front(Box::<Link<i32>>::new(
            Link { value: 5, next: None, previous: None }
        ));
        dll.push_front(Box::<Link<i32>>::new(
            Link { value: 1, next: None, previous: None }
        ));
        assert_eq!(1, dll.head.unwrap().value);
        assert_eq!(5, dll.tail.unwrap().value);
    }

    #[test]
    fn pop_back_to_empty_works() {
        let mut dll: DoublyLinkedList<i32> = DoublyLinkedList::new();
        dll.push_back(Box::<Link<i32>>::new(
            Link { value: 1, next: None, previous: None }
        ));
        dll.pop_back();
        assert_eq!(None, dll.head);
        assert_eq!(None, dll.tail);
    }

    #[test]
    fn pop_back_works() {
        let mut dll: DoublyLinkedList<i32> = DoublyLinkedList::new();
        dll.push_back(Box::<Link<i32>>::new(
            Link { value: 1, next: None, previous: None }
        ));
        dll.push_back(Box::<Link<i32>>::new(
            Link { value: 2, next: None, previous: None }
        ));
        dll.push_back(Box::<Link<i32>>::new(
            Link { value: 3, next: None, previous: None }
        ));
        dll.pop_back();
        assert_eq!(1, dll.head.unwrap().value);
        assert_eq!(2, dll.tail.unwrap().value);
    }

    #[test]
    fn pop_front_to_empty_works() {
        let mut dll: DoublyLinkedList<i32> = DoublyLinkedList::new();
        dll.push_back(Box::<Link<i32>>::new(
            Link { value: 1, next: None, previous: None }
        ));
        dll.pop_front();
        assert_eq!(None, dll.head);
        assert_eq!(None, dll.tail);
    }

    #[test]
    fn pop_front_works() {
        let mut dll: DoublyLinkedList<i32> = DoublyLinkedList::new();
        dll.push_back(Box::<Link<i32>>::new(
            Link { value: 1, next: None, previous: None }
        ));
        dll.push_back(Box::<Link<i32>>::new(
            Link { value: 2, next: None, previous: None }
        ));
        dll.push_front(Box::<Link<i32>>::new(
            Link { value: 3, next: None, previous: None }
        ));
        dll.pop_front();
        assert_eq!(1, dll.head.unwrap().value);
        assert_eq!(2, dll.tail.unwrap().value);
    }
}