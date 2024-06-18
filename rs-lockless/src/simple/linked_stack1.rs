struct Node {
    data: i32,
    next: Box<Option<Node>>,
}

struct Stack {
    head: Box<Option<Node>>,
    size: usize,
}

impl Stack {
    fn new() -> Self {
        Self { head: Box::new(None), size: 0 }
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn size(&self) -> usize {
        self.size
    }

    fn push(&mut self, data: i32) {
        let head = self.head.take();
        let new_node = Node { data, next: Box::new(head) };
        self.head = Box::new(Some(new_node));
    }

    fn pop(&mut self) -> Option<i32> {
        if let Some(mut node) = self.head.take() {
            if let Some(next) = node.next.take() {
                self.head = Box::new(Some(next));
            } else {
                self.head = Box::new(None);
            }
            Some(node.data)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut list = Stack::new();
        assert!(list.is_empty());
        list.push(1);
        list.push(2);
        list.push(3);
        assert!(!list.is_empty());
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        assert!(list.is_empty());
    }
}
