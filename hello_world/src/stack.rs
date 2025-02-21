pub(crate) mod stack_mod {
    pub struct Stack<T> {
        top: Option<Box<Node<T>>>,
        length: usize,
    }
    struct Node<T> {
        value: T,
        next: Option<Box<Node<T>>>,
    }
    impl<T> Stack<T> {
        pub fn new() -> Self {
            Stack {
                top: None,
                length: 0,
            }
        }
        pub fn push(&mut self, x: T) {
            let node = Box::new(Node {
                value: x,
                next: self.top.take(),
            });
            self.top = Some(node);
            self.length += 1;
        }
        pub fn pop(&mut self) -> Option<T> {
            match self.top.take() {
                Some(node) => {
                    self.top = node.next;
                    self.length -= 1;
                    Some(node.value)
                }
                None => None, // 更安全的错误处理
            }
        }
        pub fn is_empty(&self) -> bool {
            self.length == 0
        }
        pub fn peek(&self) -> Option<&T> {
            self.top.as_ref().map(|node| &node.value)
        }
        pub fn len(&self) -> usize {
            self.length
        }
        pub fn clear(&mut self) {
            while let Some(_) = self.top.take() {
                self.top = self.top.take().and_then(|mut node| node.next.take());
                self.length -= 1;
            }
            self.length = 0;
        }
        pub fn into_iter(self) -> impl Iterator<Item = T> {
            let mut current = self.top;
            std::iter::from_fn(move || {
                current.take().map(|mut node| {
                    current = node.next.take();
                    node.value
                })
            })
        }
        pub fn iter(&self) -> impl Iterator<Item = &T> {
            let mut current = self.top.as_ref();
            std::iter::from_fn(move || {
                current.take().map(|node| {
                    current = node.next.as_ref();
                    &node.value
                })
            })
        }
    }
    impl<T> IntoIterator for Stack<T> {
        type Item = T;
        type IntoIter = std::vec::IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter {
            let mut items = Vec::new();
            let mut current = self.top;
            while let Some(node) = current {
                items.push(node.value);
                current = node.next;
            }
            items.into_iter()
        }
    }

}

pub use stack_mod::Stack;