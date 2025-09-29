/*
	dfs
	This problem requires you to implement a basic DFS traversal
*/


use std::collections::HashSet;

struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src); 
    }

    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        if visited.contains(&v) {
            return;
        }
        visited.insert(v);
        visit_order.push(v);
        for &neighbor in &self.adj[v] {
            if !visited.contains(&neighbor) {
                self.dfs_util(neighbor, visited, visit_order);
            }
        }
    }

    // Perform a depth-first search on the graph, return the order of visited nodes
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut visit_order = Vec::new(); 
        self.dfs_util(start, &mut visited, &mut visit_order);
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_with_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]); 
        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]); 
    }
}

struct Stack<T> {
    data: Vec<T>,
    size: usize,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            data: Vec::new(),
            size: 0,
        }
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            self.size -= 1;
            self.data.pop()
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

fn bracket_match(bracket: &str) -> bool {
    let mut stack = Stack::new();
    for c in bracket.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' => {
                if let Some(top) = stack.pop() {
                    if top != '(' { return false; }
                } else { return false; }
            }
            '}' => {
                if let Some(top) = stack.pop() {
                    if top != '{' { return false; }
                } else { return false; }
            }
            ']' => {
                if let Some(top) = stack.pop() {
                    if top != '[' { return false; }
                } else { return false; }
            }
            _ => {}
        }
    }
    stack.is_empty()
}

