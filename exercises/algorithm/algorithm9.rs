/*
	heap
	This question requires you to implement a binary heap function
*/


// use std::cmp::Ord; // 已不需要，移除
use std::default::Default;

pub struct Heap<T> {
    items: Vec<T>,
    count: usize,
    comparator: Box<dyn Fn(&T, &T) -> bool>,
}

impl<T> Heap<T>
where
    T: Default + Clone,
{
    pub fn new<F>(cmp: F) -> Self
    where
        F: 'static + Fn(&T, &T) -> bool,
    {
        let mut items = Vec::new();
        items.push(T::default()); // 占位，便于下标从1开始
        Heap {
            items,
            count: 0,
            comparator: Box::new(cmp),
        }
    }
    
    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        let mut idx = self.count;
        // 上浮
        while idx > 1 {
            let parent = self.parent_idx(idx);
            if !(self.comparator)(&self.items[idx], &self.items[parent]) {
                break;
            }
            self.items.swap(idx, parent);
            idx = parent;
        }
    }
    
    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        if right <= self.count && (self.comparator)(&self.items[right], &self.items[left]) {
            right
        } else if left <= self.count {
            left
        } else {
            0
        }
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        // 堆顶元素
        let result = self.items[1].clone();
        // 最后一个元素移到堆顶
        self.items[1] = self.items[self.count].clone();
        self.items.pop();
        self.count -= 1;
        // 下沉
        let mut idx = 1;
        while self.children_present(idx) {
            let child = self.smallest_child_idx(idx);
            if child == 0 || !(self.comparator)(&self.items[child], &self.items[idx]) {
                break;
            }
            self.items.swap(idx, child);
            idx = child;
        }
        Some(result)
    }
}

pub trait Graph {
    fn add_node(&mut self, node: &str) -> bool;
    fn add_edge(&mut self, edge: (&str, &str, i32));
}

pub struct MyGraph {
    pub adjacency_table: std::collections::HashMap<String, Vec<(String, i32)>>,
}

impl MyGraph {
    pub fn new() -> Self {
        MyGraph {
            adjacency_table: std::collections::HashMap::new(),
        }
    }
    pub fn adjacency_table_mutable(&mut self) -> &mut std::collections::HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
}

impl Graph for MyGraph {
    fn add_node(&mut self, node: &str) -> bool {
        let table = self.adjacency_table_mutable();
        if !table.contains_key(node) {
            table.insert(node.to_string(), Vec::new());
            true
        } else {
            false
        }
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (from, to, weight) = edge;
        self.add_node(from);
        self.add_node(to);
        if let Some(neighbors) = self.adjacency_table.get_mut(from) {
            neighbors.push((to.to_string(), weight));
        }
      
    }
}

