use std::borrow::Borrow;
use std::fmt::Display;
use std::ptr::NonNull;

struct Node {
    data: i32,
    next: Option<NonNull<Node>>,
}

impl Node {
    pub fn new(data: i32) -> Self {
        Self { data, next: None }
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

impl Drop for Node {
    fn drop(&mut self) {
        println!("drop - {}", self.data);
    }
}

struct LinkedStack {
    head: Option<NonNull<Node>>,
    size: usize,
}

impl LinkedStack {
    pub fn new() -> Self {
        Self { head: None, size: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, data: i32) {
        let node = Node::new(data);
        let node = NonNull::from(Box::leak(Box::new(node)));

        if let Some(head) = self.head {
            unsafe {
                (*node.as_ptr()).next = Some(head);
            }
        }

        self.head = Some(node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            let node = unsafe { Box::from_raw(node.as_ptr()) };
            self.head = node.next;
            self.size -= 1;
            node.data
        })
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
fn test_linked_stack() {
    let mut list = LinkedStack::new();
    assert!(list.is_empty());
    list.push(1);
    list.push(2);
    list.push(3);
    assert!(!list.is_empty());
    println!("size is {}", list.size);

    println!("================");
    assert_eq!(list.pop(), Some(3));
    println!("pop ===================");
    assert_eq!(list.pop(), Some(2));
    println!("pop ===================");
    assert_eq!(list.pop(), Some(1));
    println!("pop ===================");
    assert_eq!(list.pop(), None);
    assert!(list.is_empty());
}
