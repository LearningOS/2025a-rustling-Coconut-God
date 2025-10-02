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
    q1: Queue<T>,
    q2: Queue<T>,
    use_q1_as_main: bool,
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::new(),
            q2: Queue::new(),
            use_q1_as_main: true,
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.use_q1_as_main {
            self.q1.enqueue(elem);
        } else {
            self.q2.enqueue(elem);
        }
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        if self.is_empty() {
            return Err("Stack is empty");
        }

        let use_q1 = self.use_q1_as_main;
        let main_size = if use_q1 { self.q1.size() } else { self.q2.size() };

        // 先把所有要转移的元素出队到临时 Vec
        let mut temp = Vec::with_capacity(main_size);
        if use_q1 {
            let mut q1_vals = std::mem::take(&mut self.q1.elements);
            for _ in 0..main_size.saturating_sub(1) {
                temp.push(q1_vals.remove(0));
            }
            let result = q1_vals.remove(0);
            self.q1.elements = Vec::new();
            self.q2.elements.extend(temp);
            self.use_q1_as_main = false;
            Ok(result)
        } else {
            let mut q2_vals = std::mem::take(&mut self.q2.elements);
            for _ in 0..main_size.saturating_sub(1) {
                temp.push(q2_vals.remove(0));
            }
            let result = q2_vals.remove(0);
            self.q2.elements = Vec::new();
            self.q1.elements.extend(temp);
            self.use_q1_as_main = true;
            Ok(result)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
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