use std::borrow::Borrow;
use std::collections::LinkedList;
use std::fmt::Display;
use std::mem;
use std::ptr::NonNull;

struct Node {
    data: i32,
    next: Option<NonNull<Node>>,
}

impl Node {
    fn new(data: i32) -> Self {
        Self { data, next: None }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("drop {}", self.data);
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.next.borrow() {
            Some(node) => write!(f, "{} -> {}", self.data, unsafe { node.as_ref() }),
            None => write!(f, "{} -> None", self.data),
        }
    }
}

struct LinkedQueue {
    head: Option<NonNull<Node>>,
    tail: Option<NonNull<Node>>,
    size: usize,
}

impl LinkedQueue {
    pub fn new() -> Self {
        Self { head: None, tail: None, size: 0 }
    }

    pub fn enqueue(&mut self, data: i32) {
        let mut node = Node::new(data);
        // 这里故意 leak 数据，enqueue 后便会被 drop 掉
        let node = NonNull::from(Box::leak(Box::new(node)));

        match self.tail {
            None => self.head = Some(node),
            Some(tail) => unsafe { (*tail.as_ptr()).next = Some(node) },
        }

        self.tail = Some(node);
        self.size += 1;
    }

    fn dequeue(&mut self) -> Option<i32> {
        if let Some(head_ptr) = self.head {
            let head = unsafe { *Box::from_raw(head_ptr.as_ptr()) };
            self.head = head.next;

            if self.head.is_none() {
                self.tail = None;
            }
            self.size -= 1;

            Some(head.data)
        } else {
            None
        }
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl Display for LinkedQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            let node_ref = unsafe { node.as_ref() };
            write!(f, "{} -> ", node_ref.data);
            if let Some(next) = &node_ref.next {
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
    println!("[{}]", node);

    let mut node = Node { data: 3, next: NonNull::new(&mut node as *mut _) };
    println!("[{}]", node);

    let node = Node { data: 5, next: NonNull::new(&mut node as *mut _) };
    println!("[{}]", node);
}

#[test]
fn test_linked_queue() {
    let mut queue = LinkedQueue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    assert_eq!(queue.size(), 3);
    println!("queue is [{}]", queue);
    println!("=================================");

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
