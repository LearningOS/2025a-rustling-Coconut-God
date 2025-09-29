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
    if self.is_empty() {
        return Err("Stack is empty");
    }

    let use_q1 = self.use_q1_as_main;
    let main_size = if use_q1 { self.q1.size() } else { self.q2.size() };

    // 临时转移元素
    for _ in 0..main_size.saturating_sub(1) {
        let val = if use_q1 {
            self.q1.dequeue()?
        } else {
            self.q2.dequeue()?
        };
        if use_q1 {
            self.q2.enqueue(val);
        } else {
            self.q1.enqueue(val);
        }
    }

    // 取出最后一个元素
    let result = if use_q1 {
        self.q1.dequeue()
    } else {
        self.q2.dequeue()
    }?;

    // 切换主队列标志
    self.use_q1_as_main = !self.use_q1_as_main;

    Ok(result)
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