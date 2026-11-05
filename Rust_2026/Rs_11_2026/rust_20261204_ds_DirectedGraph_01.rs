//---------------------------------------------------
// DirectedGraph - Rust Data Structures
// Date: 20261204
//---------------------------------------------------

use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::size_of_val;

// ============================================================
// 1. DIRECTED GRAPH STRUCTURE
// ============================================================

#[derive(Debug, Clone)]
struct DirectedGraph {
    adjacency_list: HashMap<i32, Vec<i32>>,
}

// ============================================================
// 2. IMPLEMENTATION
// ============================================================

impl DirectedGraph {

    // 3. Create an empty graph
    fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    // 4. Check whether graph is empty
    fn is_empty(&self) -> bool {
        self.adjacency_list.is_empty()
    }

    // 5. Get vertex count
    fn vertex_count(&self) -> usize {
        self.adjacency_list.len()
    }

    // 6. Get directed edge count
    fn edge_count(&self) -> usize {
        self.adjacency_list
            .values()
            .map(|neighbors| neighbors.len())
            .sum()
    }

    // 7. Add vertex
    fn add_vertex(&mut self, vertex: i32) {
        self.adjacency_list.entry(vertex).or_default();
    }

    // 8. Add directed edge: source -> destination
    fn add_edge(&mut self, source: i32, destination: i32) {
        self.add_vertex(source);
        self.add_vertex(destination);

        let neighbors = self.adjacency_list.get_mut(&source).unwrap();

        if !neighbors.contains(&destination) {
            neighbors.push(destination);
        }
    }

    // 9. Check whether vertex exists
    fn contains_vertex(&self, vertex: i32) -> bool {
        self.adjacency_list.contains_key(&vertex)
    }

    // 10. Check whether directed edge exists
    fn contains_edge(&self, source: i32, destination: i32) -> bool {
        self.adjacency_list
            .get(&source)
            .map_or(false, |neighbors| neighbors.contains(&destination))
    }

    // 11. Remove directed edge
    fn remove_edge(&mut self, source: i32, destination: i32) -> bool {
        if let Some(neighbors) = self.adjacency_list.get_mut(&source) {
            let old_len = neighbors.len();
            neighbors.retain(|&v| v != destination);
            return neighbors.len() != old_len;
        }

        false
    }

    // 12. Remove vertex and all incoming/outgoing edges
    fn remove_vertex(&mut self, vertex: i32) -> bool {
        if self.adjacency_list.remove(&vertex).is_none() {
            return false;
        }

        for neighbors in self.adjacency_list.values_mut() {
            neighbors.retain(|&v| v != vertex);
        }

        true
    }

    // 13. Get outgoing neighbours
    fn outgoing_neighbors(&self, vertex: i32) -> Option<&Vec<i32>> {
        self.adjacency_list.get(&vertex)
    }

    // 14. Get incoming neighbours
    fn incoming_neighbors(&self, vertex: i32) -> Option<Vec<i32>> {
        if !self.contains_vertex(vertex) {
            return None;
        }

        let mut result = Vec::new();

        for (&source, neighbors) in &self.adjacency_list {
            if neighbors.contains(&vertex) {
                result.push(source);
            }
        }

        result.sort();
        Some(result)
    }

    // 15. Calculate out-degree
    fn out_degree(&self, vertex: i32) -> Option<usize> {
        self.adjacency_list
            .get(&vertex)
            .map(|neighbors| neighbors.len())
    }

    // 16. Calculate in-degree
    fn in_degree(&self, vertex: i32) -> Option<usize> {
        if !self.contains_vertex(vertex) {
            return None;
        }

        Some(
            self.adjacency_list
                .values()
                .map(|neighbors| {
                    neighbors.iter().filter(|&&v| v == vertex).count()
                })
                .sum()
        )
    }

    // 17. Get total degree
    fn total_degree(&self, vertex: i32) -> Option<usize> {
        Some(self.in_degree(vertex)? + self.out_degree(vertex)?)
    }

    // 18. Get all vertices
    fn vertices(&self) -> Vec<i32> {
        let mut result: Vec<i32> =
            self.adjacency_list.keys().copied().collect();

        result.sort();
        result
    }

    // 19. Display directed graph
    fn display(&self) {
        println!("Directed Adjacency List:");

        for vertex in self.vertices() {
            println!(
                "{} -> {:?}",
                vertex,
                self.adjacency_list.get(&vertex).unwrap()
            );
        }
    }

    // 20. Depth First Search
    fn dfs(&self, start: i32) -> Vec<i32> {
        if !self.contains_vertex(start) {
            return Vec::new();
        }

        let mut visited = HashSet::new();
        let mut stack = vec![start];
        let mut result = Vec::new();

        while let Some(vertex) = stack.pop() {
            if visited.insert(vertex) {
                result.push(vertex);

                if let Some(neighbors) = self.outgoing_neighbors(vertex) {
                    for &neighbor in neighbors.iter().rev() {
                        if !visited.contains(&neighbor) {
                            stack.push(neighbor);
                        }
                    }
                }
            }
        }

        result
    }

    // 21. Breadth First Search
    fn bfs(&self, start: i32) -> Vec<i32> {
        if !self.contains_vertex(start) {
            return Vec::new();
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        visited.insert(start);
        queue.push_back(start);

        while let Some(vertex) = queue.pop_front() {
            result.push(vertex);

            if let Some(neighbors) = self.outgoing_neighbors(vertex) {
                for &neighbor in neighbors {
                    if visited.insert(neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        result
    }

    // 22. Check directed path: source -> destination
    fn has_path(&self, source: i32, destination: i32) -> bool {
        self.bfs(source).contains(&destination)
    }

    // 23. Find vertices reachable from a source
    fn reachable_vertices(&self, source: i32) -> Vec<i32> {
        self.bfs(source)
    }

    // 24. Check whether graph has a self-loop
    fn has_self_loop(&self) -> bool {
        self.adjacency_list.iter().any(|(&vertex, neighbors)| {
            neighbors.contains(&vertex)
        })
    }

    // 25. Count self-loops
    fn self_loop_count(&self) -> usize {
        self.adjacency_list.iter().filter(|(&vertex, neighbors)| {
            neighbors.contains(&vertex)
        }).count()
    }

    // 26. Detect cycle using DFS coloring
    // 0 = unvisited, 1 = currently visiting, 2 = completed
    fn has_cycle(&self) -> bool {
        fn visit(
            graph: &DirectedGraph,
            vertex: i32,
            colors: &mut HashMap<i32, u8>,
        ) -> bool {
            colors.insert(vertex, 1);

            if let Some(neighbors) = graph.outgoing_neighbors(vertex) {
                for &neighbor in neighbors {
                    match colors.get(&neighbor).copied().unwrap_or(0) {
                        1 => return true,
                        0 => {
                            if visit(graph, neighbor, colors) {
                                return true;
                            }
                        }
                        _ => {}
                    }
                }
            }

            colors.insert(vertex, 2);
            false
        }

        let mut colors = HashMap::new();

        for vertex in self.vertices() {
            if colors.get(&vertex).copied().unwrap_or(0) == 0 {
                if visit(self, vertex, &mut colors) {
                    return true;
                }
            }
        }

        false
    }

    // 27. Topological sorting using Kahn's algorithm
    // Returns None if the graph contains a cycle.
    fn topological_sort(&self) -> Option<Vec<i32>> {
        let mut in_degree_map: HashMap<i32, usize> =
            HashMap::new();

        for &vertex in self.adjacency_list.keys() {
            in_degree_map.insert(vertex, 0);
        }

        for neighbors in self.adjacency_list.values() {
            for &neighbor in neighbors {
                *in_degree_map.get_mut(&neighbor).unwrap() += 1;
            }
        }

        let mut zero_degree: Vec<i32> = in_degree_map
            .iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(&vertex, _)| vertex)
            .collect();

        zero_degree.sort();

        let mut queue: VecDeque<i32> = zero_degree.into();
        let mut result = Vec::new();

        while let Some(vertex) = queue.pop_front() {
            result.push(vertex);

            if let Some(neighbors) = self.outgoing_neighbors(vertex) {
                for &neighbor in neighbors {
                    let degree = in_degree_map.get_mut(&neighbor).unwrap();
                    *degree -= 1;

                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        if result.len() == self.vertex_count() {
            Some(result)
        } else {
            None
        }
    }

    // 28. Reverse all directed edges
    fn reverse_graph(&self) -> Self {
        let mut reversed = Self::new();

        for vertex in self.vertices() {
            reversed.add_vertex(vertex);
        }

        for (&source, neighbors) in &self.adjacency_list {
            for &destination in neighbors {
                reversed.add_edge(destination, source);
            }
        }

        reversed
    }

    // 29. Check whether graph is strongly connected
    // Every vertex must be reachable from every other vertex.
    fn is_strongly_connected(&self) -> bool {
        if self.is_empty() {
            return true;
        }

        let start = self.vertices()[0];

        let forward = self.dfs(start);

        if forward.len() != self.vertex_count() {
            return false;
        }

        let reversed = self.reverse_graph();
        let backward = reversed.dfs(start);

        backward.len() == self.vertex_count()
    }

    // 30. Find strongly connected components using Kosaraju's algorithm
    fn strongly_connected_components(&self) -> Vec<Vec<i32>> {
        fn fill_order(
            graph: &DirectedGraph,
            vertex: i32,
            visited: &mut HashSet<i32>,
            order: &mut Vec<i32>,
        ) {
            visited.insert(vertex);

            if let Some(neighbors) = graph.outgoing_neighbors(vertex) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        fill_order(graph, neighbor, visited, order);
                    }
                }
            }

            order.push(vertex);
        }

        fn collect_component(
            graph: &DirectedGraph,
            vertex: i32,
            visited: &mut HashSet<i32>,
            component: &mut Vec<i32>,
        ) {
            visited.insert(vertex);
            component.push(vertex);

            if let Some(neighbors) = graph.outgoing_neighbors(vertex) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        collect_component(
                            graph,
                            neighbor,
                            visited,
                            component,
                        );
                    }
                }
            }
        }

        let mut visited = HashSet::new();
        let mut order = Vec::new();

        for vertex in self.vertices() {
            if !visited.contains(&vertex) {
                fill_order(self, vertex, &mut visited, &mut order);
            }
        }

        let reversed = self.reverse_graph();
        visited.clear();

        let mut components = Vec::new();

        while let Some(vertex) = order.pop() {
            if !visited.contains(&vertex) {
                let mut component = Vec::new();

                collect_component(
                    &reversed,
                    vertex,
                    &mut visited,
                    &mut component,
                );

                components.push(component);
            }
        }

        components
    }

    // 31. Find source vertices (in-degree = 0)
    fn source_vertices(&self) -> Vec<i32> {
        self.vertices()
            .into_iter()
            .filter(|&vertex| self.in_degree(vertex) == Some(0))
            .collect()
    }

    // 32. Find sink vertices (out-degree = 0)
    fn sink_vertices(&self) -> Vec<i32> {
        self.vertices()
            .into_iter()
            .filter(|&vertex| self.out_degree(vertex) == Some(0))
            .collect()
    }

    // 33. Check whether graph is a DAG
    // DAG = Directed Acyclic Graph
    fn is_dag(&self) -> bool {
        !self.has_cycle()
    }

    // 34. Clear graph
    fn clear(&mut self) {
        self.adjacency_list.clear();
    }

    // 35. Clone graph
    fn clone_graph(&self) -> Self {
        self.clone()
    }

    // 36. Estimate adjacency-list storage
    fn memory_size(&self) -> usize {
        let map_storage = size_of_val(&self.adjacency_list);

        let neighbor_storage: usize = self
            .adjacency_list
            .values()
            .map(|neighbors| size_of_val(neighbors.as_slice()))
            .sum();

        map_storage + neighbor_storage
    }
}

// ============================================================
// 37. MAIN FUNCTION
// ============================================================

fn main() {

    println!("======================================");
    println!("       RUST DIRECTED GRAPH");
    println!("======================================");

    // 38. Create graph
    let mut graph = DirectedGraph::new();

    // 39. Add vertices
    for vertex in 1..=6 {
        graph.add_vertex(vertex);
    }

    // 40. Add directed edges
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 4);
    graph.add_edge(3, 4);
    graph.add_edge(4, 5);
    graph.add_edge(5, 6);

    // 41. Display graph
    println!("\n1. Directed Graph");
    graph.display();

    // 42. Vertex and edge count
    println!("\n2. Graph Size");
    println!("Vertices: {}", graph.vertex_count());
    println!("Edges: {}", graph.edge_count());

    // 43. Vertex and edge search
    println!("\n3. Search");
    println!("Vertex 3: {}", graph.contains_vertex(3));
    println!("Edge 1 -> 2: {}", graph.contains_edge(1, 2));
    println!("Edge 2 -> 1: {}", graph.contains_edge(2, 1));

    // 44. Outgoing neighbours
    println!("\n4. Outgoing Neighbours");
    println!("{:?}", graph.outgoing_neighbors(1));

    // 45. Incoming neighbours
    println!("\n5. Incoming Neighbours");
    println!("{:?}", graph.incoming_neighbors(4));

    // 46. In-degree and out-degree
    println!("\n6. Degree");
    println!("In-degree of 4: {:?}", graph.in_degree(4));
    println!("Out-degree of 4: {:?}", graph.out_degree(4));
    println!("Total degree of 4: {:?}", graph.total_degree(4));

    // 47. DFS
    println!("\n7. DFS");
    println!("{:?}", graph.dfs(1));

    // 48. BFS
    println!("\n8. BFS");
    println!("{:?}", graph.bfs(1));

    // 49. Reachability
    println!("\n9. Reachability");
    println!("1 -> 6: {}", graph.has_path(1, 6));
    println!("6 -> 1: {}", graph.has_path(6, 1));

    // 50. Reachable vertices
    println!("\n10. Reachable Vertices");
    println!("{:?}", graph.reachable_vertices(1));

    // 51. Source vertices
    println!("\n11. Source Vertices");
    println!("{:?}", graph.source_vertices());

    // 52. Sink vertices
    println!("\n12. Sink Vertices");
    println!("{:?}", graph.sink_vertices());

    // 53. Cycle detection
    println!("\n13. Cycle Detection");
    println!("Has cycle: {}", graph.has_cycle());

    // 54. DAG check
    println!("\n14. DAG Check");
    println!("Is DAG: {}", graph.is_dag());

    // 55. Topological sort
    println!("\n15. Topological Sorting");
    println!("{:?}", graph.topological_sort());

    // 56. Reverse graph
    println!("\n16. Reverse Graph");
    let reversed = graph.reverse_graph();
    reversed.display();

    // 57. Strong connectivity
    println!("\n17. Strong Connectivity");
    println!("{}", graph.is_strongly_connected());

    // 58. Strongly connected components
    println!("\n18. Strongly Connected Components");
    println!("{:?}", graph.strongly_connected_components());

    // 59. Self-loop
    println!("\n19. Self Loop");
    graph.add_edge(6, 6);
    println!("Has self-loop: {}", graph.has_self_loop());
    println!("Self-loop count: {}", graph.self_loop_count());

    // 60. Cycle after adding self-loop
    println!("\n20. Cycle After Self Loop");
    println!("Has cycle: {}", graph.has_cycle());

    // 61. Remove edge
    println!("\n21. Remove Edge");
    println!("Removed: {}", graph.remove_edge(1, 3));
    graph.display();

    // 62. Remove vertex
    println!("\n22. Remove Vertex");
    println!("Removed: {}", graph.remove_vertex(5));
    graph.display();

    // 63. Clone graph
    println!("\n23. Clone Graph");
    let cloned = graph.clone_graph();
    cloned.display();

    // 64. Memory estimation
    println!("\n24. Memory Estimation");
    println!("Estimated bytes: {}", graph.memory_size());

    // 65. Clear graph
    println!("\n25. Clear Graph");
    graph.clear();
    println!("Is empty: {}", graph.is_empty());
    println!("Vertex count: {}", graph.vertex_count());

    println!("\n======================================");
    println!("          PROGRAM COMPLETE");
    println!("======================================");
}


/*
//---------------------------------------------------
We'll implemented:

- Directed vertex and edge operations.
- Incoming and outgoing neighbours.
- In-degree and out-degree.
- DFS and BFS.
- Path detection and reachability.
- Cycle detection.
- Topological sorting.
- Strongly connected components.
- Graph reversal.

//---------------------------------------------------
Important concepts:

Operation - Time complexity

Add vertex      - O(1) average
Add edge        - O(degree)
Search edge     - O(degree)
In-degree       - O(V + E)
Out-degree      - O(1) average
DFS / BFS                       - O(V + E)
Cycle detection                 - O(V + E)
Topological sorting             - O(V + E)
Strongly connected components   - O(V + E)

Remember these three concepts:
1. In-degree: Number of incoming edges.
2. Out-degree: Number of outgoing edges.
3. DAG: Directed Acyclic Graph — a directed graph with no cycles.

//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261204_ds_DirectedGraph_01.rs -o directed_graph_example

./directed_graph_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

======================================
       RUST DIRECTED GRAPH
======================================

1. Directed Graph
Directed Adjacency List:
1 -> [2, 3]
2 -> [4]
3 -> [4]
4 -> [5]
5 -> [6]
6 -> []

2. Graph Size
Vertices: 6
Edges: 6

3. Search
Vertex 3: true
Edge 1 -> 2: true
Edge 2 -> 1: false

4. Outgoing Neighbours
Some([2, 3])

5. Incoming Neighbours
Some([2, 3])

6. Degree
In-degree of 4: Some(2)
Out-degree of 4: Some(1)
Total degree of 4: Some(3)

7. DFS
[1, 2, 4, 5, 6, 3]

8. BFS
[1, 2, 3, 4, 5, 6]

9. Reachability
1 -> 6: true
6 -> 1: false

10. Reachable Vertices
[1, 2, 3, 4, 5, 6]

11. Source Vertices
[1]

12. Sink Vertices
[6]

13. Cycle Detection
Has cycle: false

14. DAG Check
Is DAG: true

15. Topological Sorting
Some([1, 2, 3, 4, 5, 6])

16. Reverse Graph
Directed Adjacency List:
1 -> []
2 -> [1]
3 -> [1]
4 -> [2, 3]
5 -> [4]
6 -> [5]

17. Strong Connectivity
false

18. Strongly Connected Components
[[1], [3], [2], [4], [5], [6]]

19. Self Loop
Has self-loop: true
Self-loop count: 1

20. Cycle After Self Loop
Has cycle: true

21. Remove Edge
Removed: true
Directed Adjacency List:
1 -> [2]
2 -> [4]
3 -> [4]
4 -> [5]
5 -> [6]
6 -> [6]

22. Remove Vertex
Removed: true
Directed Adjacency List:
1 -> [2]
2 -> [4]
3 -> [4]
4 -> []
6 -> [6]

23. Clone Graph
Directed Adjacency List:
1 -> [2]
2 -> [4]
3 -> [4]
4 -> []
6 -> [6]

24. Memory Estimation
Estimated bytes: 64

25. Clear Graph
Is empty: true
Vertex count: 0

======================================
          PROGRAM COMPLETE
======================================

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/


