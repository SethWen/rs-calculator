use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::collections::LinkedList;
use std::fmt::{Debug, Display};
use std::ptr::NonNull;
use std::rc::Rc;

struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(data: i32) -> Self {
        Node { data, next: None }
    }

    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

// impl Drop for Node {
//     fn drop(&mut self) {
//         println!("drop {}", self.data);
//     }
// }

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.next {
            Some(node) => {
                let node = node.as_ref();
                write!(f, "Node({}) -> {}", self.data, node)
            }
            None => write!(f, "Node({}) -> null", self.data),
        }
    }
}

struct LinkedQueue {
    head: Option<Box<Node>>,
    tail: *mut Node,
    size: usize,
}

impl LinkedQueue {
    fn new() -> Self {
        Self { head: None, tail: std::ptr::null_mut(), size: 0 }
    }

    fn enqueue(&mut self, data: i32) {
        let mut new_node = Node::new(data).boxed();
        let raw_ptr: *mut _ = &mut *new_node;
        if !self.tail.is_null() {
            unsafe { (*self.tail).next = Some(new_node) };
        } else {
            self.head = Some(new_node);
        }
        self.tail = raw_ptr;
        self.size += 1;
    }

    fn dequeue(&mut self) -> Option<i32> {
        self.head.take().map(|head| {
            // let head = *head;
            self.head = head.next;
            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }
            self.size -= 1;
            head.data
        })
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl Display for LinkedQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            write!(f, "{} -> ", node.data);
            if let Some(next) = &node.next {
                current = Some(next);
            } else {
                current = None;
            }
        }
        write!(f, "None")
    }
}

#[test]
fn test_node() {
    let mut node = Node { data: 2, next: None };
    println!("{node}");

    let mut node = Node { data: 3, next: Some(Box::new(node)) };
    println!("{}", node);

    let node = Node { data: 5, next: Some(Box::new(node)) };
    println!("{}", node);
}

#[test]
fn test_linked_queue() {
    let mut queue = LinkedQueue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    assert_eq!(queue.size(), 3);
    println!("queue is [{}]", queue);

    assert_eq!(queue.dequeue(), Some(1));
    assert_eq!(queue.size(), 2);
    println!("queue is [{}]", queue);

    assert_eq!(queue.dequeue(), Some(2));
    assert_eq!(queue.size(), 1);
    println!("queue is [{}]", queue);

    assert_eq!(queue.dequeue(), Some(3));
    assert_eq!(queue.size(), 0);
    println!("queue is [{}]", queue);

    assert_eq!(queue.dequeue(), None);
    assert_eq!(queue.size(), 0);
    println!("queue is [{}]", queue);
}
