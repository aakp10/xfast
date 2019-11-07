use std::ptr::NonNull;
use core::marker::PhantomData;

struct Node<T> {
    data: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

pub struct Deque<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

pub struct Iter<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

pub struct IterMut<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}


impl<T> Node<T> {
    fn new(data: T) -> Box<Self>{
        Box::new(Node{
            data,
            next: None,
            prev: None
        })
    }
} 

impl<T> Deque<T> {
    fn new() -> Self {
        Deque {
            head: None,
            tail: None,
            len: 0
        }
    }

    pub fn push_front(&mut self, data: T) {
        let mut new_node = Node::new(data);
        unsafe {
            new_node.next = self.head;
            let new_node_ptr = Box::into_raw_non_null(new_node);
            match self.head {
                Some(cur_head) => {
                    (*cur_head.as_ptr()).prev = Some(new_node_ptr);
                }
                None => {
                    self.tail = Some(new_node_ptr);
                }
            }
            self.head = Some(new_node_ptr);
            self.len += 1;
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|old_tail| unsafe {
            //as_ptr moves the value
            let old_tail = Box::from_raw(old_tail.as_ptr());
            let new_tail = old_tail.prev;
            let data = old_tail.data;
            self.tail = new_tail;
            match self.tail {
                Some(tail) => (*tail.as_ptr()).next = None,
                None => self.head = None
            }
            self.len -= 1;
            data
        })
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|old_head| unsafe {
            //as_ptr moves the value
            let old_head = Box::from_raw(old_head.as_ptr());
            let new_head = old_head.next;
            let data = old_head.data;
            self.head = new_head;
            match self.head {
                Some(head) => (*head.as_ptr()).prev = None,
                None => self.tail = None
            }
            self.len -= 1;
            data
        })
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            len: self.len,
            marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            head: self.head,
            len: self.len,
            marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|cur_node| unsafe {
                let cur_node = &(*cur_node.as_ptr());
                self.head = cur_node.next;
                self.len -= 1;
                &cur_node.data
            })
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|cur_node| unsafe {
                let cur_node = &mut (*cur_node.as_ptr());
                self.head = cur_node.next;
                self.len -= 1;
                &mut cur_node.data
            })
        }
    }
}

#[cfg(test)]
mod test{
    use super::Deque;
    #[test]
    fn basic_ops() {
        let mut list: Deque<i32> = Deque::new();
        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_back(), Some(1));

        list.push_front(4);
        list.push_front(5);
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_front(), Some(5));

        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_front(), None);     
    }

    #[test]
    fn iter() {
        let mut list: Deque<i32> = Deque::new();
        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut list_iter = list.iter();
        assert_eq!(list_iter.next(), Some(&3));
        assert_eq!(list_iter.next(), Some(&2));
        assert_eq!(list_iter.next(), Some(&1));
        assert_eq!(list_iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list: Deque<i32> = Deque::new();
        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        for  val in list.iter_mut(){
            *val += 1;
        }
        let mut list_iter = list.iter();
        assert_eq!(list_iter.next(), Some(&4));
        assert_eq!(list_iter.next(), Some(&3));
    }
}