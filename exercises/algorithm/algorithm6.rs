/*
	dfs
	This problem requires you to implement a basic DFS traversal
*/
use std::collections::{HashSet, VecDeque};

struct Graph {
    adj: Vec<Vec<usize>>, 
}

struct StackWrapper {
    value: VecDeque<usize>,
}

impl StackWrapper {
    fn new() -> Self {
        Self { value: VecDeque::new() }
    }

    fn push(&mut self, value: usize) {
        self.value.push_back(value);
    }

    fn pop(&mut self) -> Option<usize> {
        self.value.pop_back()
    }
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

    // Perform a depth-first search on the graph using a stack, return the order of visited nodes
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut visit_order = Vec::new();
        let mut stack = StackWrapper::new();
        
        // 将起始节点压入栈
        stack.push(start);
        
        while let Some(v) = stack.pop() {
            // 如果节点已经被访问过，跳过
            if visited.contains(&v) {
                continue;
            }
            
            // 标记为已访问并记录访问顺序
            visited.insert(v);
            visit_order.push(v);
            
            // 将当前节点的所有未访问邻居压入栈
            // 注意：为了保持与递归版本相同的访问顺序，我们需要反向遍历邻居
            for &neighbor in self.adj[v].iter().rev() {
                if !visited.contains(&neighbor) {
                    stack.push(neighbor);
                }
            }
        }
        
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

