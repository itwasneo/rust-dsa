use std::{ptr::NonNull, fmt::Debug};
struct Link<T> {
    value: T,
    next: Option<NonNull<Link<T>>>,
    previous: Option<NonNull<Link<T>>>,
}

pub struct AppendOnlyLinkedList<T> {
    head: Option<NonNull<Link<T>>>,
    tail: Option<NonNull<Link<T>>>,
}

impl<T> AppendOnlyLinkedList<T> 
where T: Debug {

    fn push_back(&mut self, mut link: NonNull<Link<T>>) {
        unsafe {
            link.as_mut().next = None;
            link.as_mut().previous = self.tail;
            let link = Some(link);
    
            match self.tail {
                None => self.head = link,
                Some(mut tail) => {
                        tail.as_mut().next = link;
                }
            }
            self.tail = link;
            println!("{:?}", self.head.unwrap().as_mut().value);
        }
    }

    fn pop_back(&mut self) -> &mut Link<T>{
        self.tail.map(|mut link| unsafe {
            let link = link.as_mut();
            self.tail = link.previous;
            match self.tail {
                None => self.head = None,
                Some(mut tail) => {
                    tail.as_mut().next = None;
                }
            }
            link
        }).unwrap()
    }

    fn new() -> Self {
        Self {
            head: None,
            tail: None
        }
    }
}

#[cfg(test)]
mod cool_tests {
    use std::ptr::NonNull;

    use super::{Link, AppendOnlyLinkedList};

    #[test]
    fn new_works() {
        let  ll: AppendOnlyLinkedList<i32> = AppendOnlyLinkedList::new();
        assert_eq!(None, ll.head);
        assert_eq!(None, ll.tail);
    }

    #[test]
    fn push_back_works() {
        let mut ll: AppendOnlyLinkedList<i32> = AppendOnlyLinkedList::new();
        ll.push_back(NonNull::<Link<i32>>::new(&mut Link {
            value: 1,
            next: None,
            previous: None
        } as *mut _).unwrap());
        ll.push_back(NonNull::<Link<i32>>::new(&mut Link {
            value: 2,
            next: None,
            previous: None
        } as *mut _).unwrap());
        ll.push_back(NonNull::<Link<i32>>::new(&mut Link {
            value: 3,
            next: None,
            previous: None
        } as *mut _).unwrap());
        unsafe {
            assert_eq!(1, ll.head.unwrap().as_ref().value);
            assert_eq!(3, ll.tail.unwrap().as_ref().value);
        }
    }

    #[test]
    fn pop_back_works() {
        let mut ll: AppendOnlyLinkedList<i32> = AppendOnlyLinkedList::new();
        ll.push_back(NonNull::<Link<i32>>::new(&mut Link {
            value: 1,
            next: None,
            previous: None
        } as *mut _).unwrap());
        ll.push_back(NonNull::<Link<i32>>::new(&mut Link {
            value: 2,
            next: None,
            previous: None
        } as *mut _).unwrap());
        ll.push_back(NonNull::<Link<i32>>::new(&mut Link {
            value: 3,
            next: None,
            previous: None
        } as *mut _).unwrap());
        
        unsafe {
            assert_eq!(3, ll.pop_back().value);
            assert_eq!(1, ll.head.unwrap().as_ref().value);
            assert_eq!(2, ll.tail.unwrap().as_ref().value);
        }
    }
}