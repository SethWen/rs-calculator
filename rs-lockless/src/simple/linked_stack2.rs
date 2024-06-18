struct Node<T: Copy> {
    data: T,
    next: *mut Node<T>,
}

/// 通过裸指针实现链表
struct LinkedStack<T: Copy> {
    head: *mut Node<T>,
    size: usize,
}

impl<T> LinkedStack<T>
where
    T: Copy,
{
    fn new() -> Self {
        Self { head: std::ptr::null_mut(), size: 0 }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn push(&mut self, data: T) {
        let new_node = Box::new(Node { data, next: self.head });
        self.head = Box::into_raw(new_node);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }
        // let data = unsafe { (*self.head).data };
        // let new_head = unsafe { (*self.head).next };

        let data = unsafe { Box::from_raw(self.head) };
        let new_head = data.next;

        self.head = new_head;
        self.size -= 1;
        Some(data.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut list = LinkedStack::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    fn check_braces(content: &str) -> bool {
        let mut list = LinkedStack::new();
        for c1 in content.chars() {
            match c1 {
                '{' | '[' | '(' => list.push(c1),
                '}' | ']' | ')' => match list.pop() {
                    Some(c2) if (c2, c1) == ('{', '}') || (c2, c1) == ('[', ']') || (c2, c1) == ('(', ')') => {}
                    _ => return false,
                },
                _ => {}
            }
        }
        list.len() == 0
    }

    #[test]
    fn test_check_braces() {
        let content = "({}()(){()([])})";
        assert!(check_braces(content));
    }
}
