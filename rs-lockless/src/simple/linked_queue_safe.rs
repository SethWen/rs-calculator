use std::cell::RefCell;
use std::rc::Rc;

type NodeRef = Option<Rc<RefCell<Node>>>;

struct Node {
    data: i32,
    next: NodeRef,
}

struct LinkedQueue {
    head: NodeRef,
    tail: NodeRef,
    size: usize,
}

impl LinkedQueue {
    fn new() -> Self {
        Self { head: None, tail: None, size: 0 }
    }

    // fn enqueue(&mut self, data: i32) {
    //     let next_node = self.head.take();
    //     let new_node = Node { data, next: Box::new(next_node) };
    //     self.head = Box::new(Some(Node { data: 0, next: Box::new(Some(new_node)) }));
    //     self.size += 1;
    // }

    // fn dequeue(&mut self) -> Option<i32> {
    //     self.size -= 1;
    //     None
    // }
}
