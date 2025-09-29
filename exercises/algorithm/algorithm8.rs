#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        self.elements.first().ok_or("Queue is empty")
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue::new()
    }
}

pub struct MyStack<T> {
    q: Queue<T>,
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            q: Queue::new(),
        }
    }

    pub fn push(&mut self, elem: T) {
        self.q.enqueue(elem);
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        if self.q.is_empty() {
            return Err("Stack is empty");
        }

        // 使用单个队列实现栈
        let mut n = self.q.size();
        
        // 将前 n-1 个元素出队并重新入队
        while n > 1 {
            let val = self.q.dequeue().unwrap();
            self.q.enqueue(val);
            n -= 1;
        }
        
        // 返回最后一个元素（栈顶）
        self.q.dequeue()
    }

    pub fn is_empty(&self) -> bool {
        self.q.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_with_queues() {
        let mut stack = MyStack::<i32>::new();
        
        assert_eq!(stack.pop(), Err("Stack is empty"));
        
        stack.push(1);
        stack.push(2);
        stack.push(3);
        
        assert_eq!(stack.pop(), Ok(3));
        assert_eq!(stack.pop(), Ok(2));
        
        stack.push(4);
        stack.push(5);
        
        assert!(!stack.is_empty());
        assert_eq!(stack.pop(), Ok(5));
        assert_eq!(stack.pop(), Ok(4));
        assert_eq!(stack.pop(), Ok(1));
        
        assert_eq!(stack.pop(), Err("Stack is empty"));
        assert!(stack.is_empty());
    }
}