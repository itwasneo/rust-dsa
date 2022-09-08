use std::{fmt::Display};

struct Heap<T> {
    nodes: Vec<T>,
}

impl<T> Heap<T> 
where T: PartialOrd + Display {

    fn new() -> Self {
        Self {
            nodes: Vec::<T>::new()
        }
    }

    fn from_vec(nodes: Vec<T>) -> Self {
        Self {
            nodes
        }
    }

    fn to_max_heap(&mut self) {
        for i in (0..self.nodes.len()/2).rev() {
            let mut position = i;
            while let Some(max_child) = self.find_max_child(position) {
                if position < self.nodes.len() && self.nodes[max_child] > self.nodes[position] {
                    self.nodes.swap(max_child, position);
                    position = position * 2 + 1;
                } else {
                    break;
                }
            }
        }
    }

    fn to_min_heap(&mut self) {
        for i in (0..self.nodes.len()/2).rev() {
            let mut position = i;
            while let Some(min_child) = self.find_min_child(position) {
                if position < self.nodes.len() && self.nodes[min_child] < self.nodes[position] {
                    self.nodes.swap(min_child, position);
                    position = position * 2 + 1;
                } else {
                    break;
                }
            }
        }
    }

    fn find_max_child(&self, index: usize) -> Option<usize> {
        let left = 2 * index + 1;
        let right = 2 * index + 2;
        if right < self.nodes.len() {
            if self.nodes[right] > self.nodes[left] {
                return Some(right);
            } else {
                return Some(left);
            }
        } else if left < self.nodes.len() {
            return Some(left);
        }
        None
    }

    fn find_min_child(&self, index: usize) -> Option<usize> {
        let left = 2 * index + 1;
        let right = 2 * index + 2;
        if right < self.nodes.len() {
            if self.nodes[right] < self.nodes[left] {
                return Some(right);
            } else {
                return Some(left);
            }
        } else if left < self.nodes.len(){
            return Some(left);
        }
        None
    }

    fn insert_to_max_heap(&mut self, data: T) {
        if self.nodes.is_empty() {
            self.nodes.push(data);
        } else {
            self.nodes.push(data);
            self.to_max_heap();
        }
    }

    fn insert_to_min_heap(&mut self, data: T) {
        if self.nodes.is_empty() {
            self.nodes.push(data);
        } else {
            self.nodes.push(data);
            self.to_min_heap();
        }
    }

    fn delete_from_max_heap(&mut self, index: usize) -> bool{
        if let Some(_) = self.nodes.get(index) {
            let last = self.nodes.len() - 1;
            self.nodes.swap(index, last);
            self.nodes.remove(last);
            self.to_max_heap();
            return true;
        }
        false
    }

    fn delete_from_min_heap(&mut self, index: usize) -> bool {
        if let Some(_) = self.nodes.get(index) {
            let last = self.nodes.len() - 1;
            self.nodes.swap(index, last);
            self.nodes.remove(last);
            self.to_min_heap();
            return true;
        }
        false
    }

    fn peek(&self) -> Option<&T> {
        self.nodes.get(0)
    }

}

#[cfg(test)]
mod cool_tests {
    use super::Heap;

    #[test]
    fn new_works() {
        let h: Heap<i32> = Heap::new();
        assert_eq!(0, h.nodes.len());
    }

    #[test]
    fn from_vec_works() {
        let h: Heap<i32> = Heap::from_vec(vec![3, 9, 2, 1, 4, 5]);
        assert_eq!(2, *h.nodes.get(2).unwrap());
    }

    #[test]
    fn to_max_heap_works() {
        let mut h: Heap<i32> = Heap::from_vec(vec![3,9,2,1,4,5]);
        h.to_max_heap();
        println!("{:?}", h.nodes);
    }

    #[test]
    fn to_min_help_works() {
        let mut h: Heap<i32> = Heap::from_vec(vec![3,9,2,1,4,5]);
        h.to_min_heap();
        println!("{:?}", h.nodes);
    }

    #[test]
    fn peek_works() {
        let mut h: Heap<i32> = Heap::from_vec(vec![3,9,2,1,4,5]);
        h.to_max_heap();
        assert_eq!(9, *h.peek().unwrap());
    }

    #[test]
    fn insert_to_max_heap_works() {
        let mut h: Heap<i32> = Heap::new();
        h.insert_to_max_heap(3);
        h.insert_to_max_heap(5);
        assert_eq!(5, *h.peek().unwrap())
    }

    #[test]
    fn insert_to_min_heap_works() {
        let mut h: Heap<i32> = Heap::from_vec(vec![3,9,2,4,5,10]);
        h.to_min_heap();
        h.insert_to_min_heap(1);
        h.insert_to_min_heap(11);
        assert_eq!(1, *h.peek().unwrap());
    }

    #[test]
    fn delete_from_max_heap_invalid_index_works() {
        let mut h: Heap<i32> = Heap::from_vec(vec![3,9,2,4,5,10]);
        h.to_max_heap();
        assert_eq!(false, h.delete_from_max_heap(10));
        assert_eq!(true, h.delete_from_max_heap(3));
    }

    #[test]
    fn delete_from_max_heap_works() {
        let mut h: Heap<i32> = Heap::from_vec(vec![3,9,2,4,5]);
        h.to_max_heap();
        h.delete_from_max_heap(0);
        assert_eq!(5, *h.peek().unwrap());
    }

    #[test]
    fn delete_from_min_heap_invalid_index_works() {
        let mut h: Heap<i32> = Heap::from_vec(vec![3,9,2,4,5,10]);
        h.to_min_heap();
        assert_eq!(false, h.delete_from_min_heap(10));
        assert_eq!(true, h.delete_from_min_heap(3));
    }

    #[test]
    fn delete_from_min_heap_works() {
        let mut h: Heap<i32> = Heap::from_vec(vec![3,9,2,3,5,5,10]);
        h.to_min_heap();
        h.delete_from_min_heap(0);
        assert_eq!(3, *h.peek().unwrap());
        println!("{:?}", h.nodes);
    }

}