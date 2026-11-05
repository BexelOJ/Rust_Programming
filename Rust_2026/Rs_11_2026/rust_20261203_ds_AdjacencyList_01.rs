//---------------------------------------------------
// AdjacencyList - Rust Data Structures
// Date: 20261203
//---------------------------------------------------

use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::size_of_val;

// ============================================================
// 1. GRAPH STRUCTURE
// ============================================================

#[derive(Debug, Clone)]
struct AdjacencyList {
    adjacency_list: HashMap<i32, Vec<i32>>,
}

// ============================================================
// 2. IMPLEMENTATION
// ============================================================

impl AdjacencyList {

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

    // 6. Get edge count
    // Each undirected edge is stored twice.
    fn edge_count(&self) -> usize {
        self.adjacency_list
            .values()
            .map(|neighbors| neighbors.len())
            .sum::<usize>() / 2
    }

    // 7. Add a vertex
    fn add_vertex(&mut self, vertex: i32) {
        self.adjacency_list.entry(vertex).or_default();
    }

    // 8. Add an undirected edge
    fn add_edge(&mut self, source: i32, destination: i32) {
        self.add_vertex(source);
        self.add_vertex(destination);

        if source == destination {
            if !self.adjacency_list[&source].contains(&destination) {
                self.adjacency_list
                    .get_mut(&source)
                    .unwrap()
                    .push(destination);
            }
            return;
        }

        if !self.adjacency_list[&source].contains(&destination) {
            self.adjacency_list
                .get_mut(&source)
                .unwrap()
                .push(destination);
        }

        if !self.adjacency_list[&destination].contains(&source) {
            self.adjacency_list
                .get_mut(&destination)
                .unwrap()
                .push(source);
        }
    }

    // 9. Check whether vertex exists
    fn contains_vertex(&self, vertex: i32) -> bool {
        self.adjacency_list.contains_key(&vertex)
    }

    // 10. Check whether an edge exists
    fn contains_edge(&self, source: i32, destination: i32) -> bool {
        self.adjacency_list
            .get(&source)
            .map_or(false, |neighbors| neighbors.contains(&destination))
    }

    // 11. Remove an edge
    fn remove_edge(&mut self, source: i32, destination: i32) -> bool {
        if !self.contains_edge(source, destination) {
            return false;
        }

        if let Some(neighbors) = self.adjacency_list.get_mut(&source) {
            neighbors.retain(|&v| v != destination);
        }

        if source != destination {
            if let Some(neighbors) = self.adjacency_list.get_mut(&destination) {
                neighbors.retain(|&v| v != source);
            }
        }

        true
    }

    // 12. Remove a vertex and its connected edges
    fn remove_vertex(&mut self, vertex: i32) -> bool {
        let neighbors = match self.adjacency_list.remove(&vertex) {
            Some(neighbors) => neighbors,
            None => return false,
        };

        for neighbor in neighbors {
            if let Some(adjacent) = self.adjacency_list.get_mut(&neighbor) {
                adjacent.retain(|&v| v != vertex);
            }
        }

        true
    }

    // 13. Get neighbours of a vertex
    fn neighbors(&self, vertex: i32) -> Option<&Vec<i32>> {
        self.adjacency_list.get(&vertex)
    }

    // 14. Calculate vertex degree
    fn degree(&self, vertex: i32) -> Option<usize> {
        self.adjacency_list.get(&vertex).map(|neighbors| {
            neighbors.iter().map(|&v| {
                if v == vertex { 2 } else { 1 }
            }).sum()
        })
    }

    // 15. Get all vertices
    fn vertices(&self) -> Vec<i32> {
        let mut result: Vec<i32> =
            self.adjacency_list.keys().copied().collect();

        result.sort();
        result
    }

    // 16. Display adjacency list
    fn display(&self) {
        println!("Adjacency List:");

        for vertex in self.vertices() {
            print!("{} -> ", vertex);

            if let Some(neighbors) = self.adjacency_list.get(&vertex) {
                println!("{:?}", neighbors);
            }
        }
    }

    // 17. Iterative Depth First Search
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

                if let Some(neighbors) = self.neighbors(vertex) {
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

    // 18. Breadth First Search
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

            if let Some(neighbors) = self.neighbors(vertex) {
                for &neighbor in neighbors {
                    if visited.insert(neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        result
    }

    // 19. Check whether a path exists
    fn has_path(&self, source: i32, destination: i32) -> bool {
        self.bfs(source).contains(&destination)
    }

    // 20. Find connected components
    fn connected_components(&self) -> Vec<Vec<i32>> {
        let mut visited = HashSet::new();
        let mut components = Vec::new();

        for vertex in self.vertices() {
            if !visited.contains(&vertex) {
                let component = self.dfs(vertex);

                for &v in &component {
                    visited.insert(v);
                }

                components.push(component);
            }
        }

        components
    }

    // 21. Check whether graph is connected
    fn is_connected(&self) -> bool {
        if self.is_empty() {
            return true;
        }

        self.connected_components().len() == 1
    }

    // 22. Find isolated vertices
    fn isolated_vertices(&self) -> Vec<i32> {
        self.adjacency_list
            .iter()
            .filter(|(_, neighbors)| neighbors.is_empty())
            .map(|(&vertex, _)| vertex)
            .collect()
    }

    // 23. Check for self-loop
    fn has_self_loop(&self) -> bool {
        self.adjacency_list.iter().any(|(&vertex, neighbors)| {
            neighbors.contains(&vertex)
        })
    }

    // 24. Count self-loops
    fn self_loop_count(&self) -> usize {
        self.adjacency_list.iter().filter(|(&vertex, neighbors)| {
            neighbors.contains(&vertex)
        }).count()
    }

    // 25. Check whether graph is complete
    // A complete simple graph has no self-loops.
    fn is_complete(&self) -> bool {
        let vertices = self.vertices();

        for (i, &source) in vertices.iter().enumerate() {
            if self.contains_edge(source, source) {
                return false;
            }

            for &destination in vertices.iter().skip(i + 1) {
                if !self.contains_edge(source, destination) {
                    return false;
                }
            }
        }

        true
    }

    // 26. Check whether graph is a tree
    fn is_tree(&self) -> bool {
        if self.is_empty() || self.has_self_loop() {
            return false;
        }

        self.is_connected()
            && self.edge_count() == self.vertex_count() - 1
    }

    // 27. Clear graph
    fn clear(&mut self) {
        self.adjacency_list.clear();
    }

    // 28. Clone graph
    fn clone_graph(&self) -> Self {
        self.clone()
    }

    // 29. Estimate adjacency-list element memory
    fn memory_size(&self) -> usize {
        let map_storage = size_of_val(&self.adjacency_list);

        let neighbor_storage: usize = self
            .adjacency_list
            .values()
            .map(|neighbors| {
                size_of_val(neighbors.as_slice())
            })
            .sum();

        map_storage + neighbor_storage
    }
}

// ============================================================
// 30. MAIN FUNCTION
// ============================================================

fn main() {

    println!("======================================");
    println!("   RUST ADJACENCY LIST DATA STRUCTURE");
    println!("======================================");

    // 31. Create graph
    let mut graph = AdjacencyList::new();

    // 32. Add vertices
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_vertex(3);
    graph.add_vertex(4);
    graph.add_vertex(5);
    graph.add_vertex(6);

    // 33. Add edges
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 4);
    graph.add_edge(2, 5);
    graph.add_edge(3, 5);
    graph.add_edge(4, 6);
    graph.add_edge(5, 6);

    // 34. Display graph
    println!("\n31. Graph Structure");
    graph.display();

    // 35. Vertex count
    println!("\n32. Vertex Count");
    println!("Vertices: {}", graph.vertex_count());

    // 36. Edge count
    println!("\n33. Edge Count");
    println!("Edges: {}", graph.edge_count());

    // 37. Check vertex
    println!("\n34. Vertex Search");
    println!("Vertex 3 exists: {}", graph.contains_vertex(3));
    println!("Vertex 9 exists: {}", graph.contains_vertex(9));

    // 38. Check edge
    println!("\n35. Edge Search");
    println!("1 -- 2: {}", graph.contains_edge(1, 2));
    println!("1 -- 6: {}", graph.contains_edge(1, 6));

    // 39. Neighbours
    println!("\n36. Neighbours");
    println!("Neighbours of 2: {:?}", graph.neighbors(2));

    // 40. Degree
    println!("\n37. Vertex Degree");
    println!("Degree of vertex 2: {:?}", graph.degree(2));

    // 41. DFS
    println!("\n38. Depth First Search");
    println!("DFS: {:?}", graph.dfs(1));

    // 42. BFS
    println!("\n39. Breadth First Search");
    println!("BFS: {:?}", graph.bfs(1));

    // 43. Path
    println!("\n40. Path Checking");
    println!("1 to 6: {}", graph.has_path(1, 6));
    println!("1 to 9: {}", graph.has_path(1, 9));

    // 44. Connected components
    println!("\n41. Connected Components");
    println!("{:?}", graph.connected_components());

    // 45. Connectivity
    println!("\n42. Is Connected");
    println!("{}", graph.is_connected());

    // 46. Isolated vertex
    println!("\n43. Isolated Vertices");
    graph.add_vertex(10);
    println!("{:?}", graph.isolated_vertices());

    // 47. Self-loop
    println!("\n44. Self Loop");
    graph.add_edge(10, 10);
    println!("Has self-loop: {}", graph.has_self_loop());
    println!("Self-loop count: {}", graph.self_loop_count());

    // 48. Complete graph
    println!("\n45. Complete Graph");

    let mut complete = AdjacencyList::new();

    complete.add_edge(1, 2);
    complete.add_edge(1, 3);
    complete.add_edge(2, 3);

    println!("Is complete: {}", complete.is_complete());

    // 49. Tree graph
    println!("\n46. Tree Graph");

    let mut tree = AdjacencyList::new();

    tree.add_edge(1, 2);
    tree.add_edge(1, 3);
    tree.add_edge(2, 4);

    println!("Is tree: {}", tree.is_tree());

    // 50. Remove edge
    println!("\n47. Remove Edge");
    println!("Removed: {}", graph.remove_edge(1, 2));
    graph.display();

    // 51. Remove vertex
    println!("\n48. Remove Vertex");
    println!("Removed: {}", graph.remove_vertex(3));
    graph.display();

    // 52. Clone graph
    println!("\n49. Clone Graph");
    let cloned_graph = graph.clone_graph();
    cloned_graph.display();

    // 53. Memory estimation
    println!("\n50. Memory Estimation");
    println!("Estimated bytes: {}", graph.memory_size());

    // 54. Clear graph
    println!("\n51. Clear Graph");
    graph.clear();

    println!("Graph empty: {}", graph.is_empty());
    println!("Vertex count: {}", graph.vertex_count());

    println!("\n======================================");
    println!("           PROGRAM COMPLETE");
    println!("======================================");
}

/*
//---------------------------------------------------
An adjacency list stores each vertex along with the vertices directly connected to it. We'll use:
- HashMap<i32, Vec<i32>> to store vertices and their neighbours.
- Vec to maintain a simple neighbour collection.
- DFS and BFS for graph traversal.

//---------------------------------------------------
Important concepts:

Operation - Average time complexity

Add vertex - O(1)
Add edge - O(degree)
Search edge - O(degree)
Remove edge - O(degree)
Remove vertex - O(V + E)
DFS - O(V + E)
BFS - O(V + E)
Display graph - O(V + E)

//---------------------------------------------------
Adjacency Matrix vs Adjacency List:

Feature - Matrix - List

Storage - O(V²) - O(V + E)
Edge lookup - O(1) - O(degree)
Iterate neighbours - O(V) - O(degree)
Best suited for - Dense graphs - Sparse graphs

//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261203_ds_AdjacencyList_01.rs -o adjacency_list_example

./adjacency_list_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

======================================
   RUST ADJACENCY LIST DATA STRUCTURE
======================================

31. Graph Structure
Adjacency List:
1 -> [2, 3]
2 -> [1, 4, 5]
3 -> [1, 5]
4 -> [2, 6]
5 -> [2, 3, 6]
6 -> [4, 5]

32. Vertex Count
Vertices: 6

33. Edge Count
Edges: 7

34. Vertex Search
Vertex 3 exists: true
Vertex 9 exists: false

35. Edge Search
1 -- 2: true
1 -- 6: false

36. Neighbours
Neighbours of 2: Some([1, 4, 5])

37. Vertex Degree
Degree of vertex 2: Some(3)

38. Depth First Search
DFS: [1, 2, 4, 6, 5, 3]

39. Breadth First Search
BFS: [1, 2, 3, 4, 5, 6]

40. Path Checking
1 to 6: true
1 to 9: false

41. Connected Components
[[1, 2, 4, 6, 5, 3]]

42. Is Connected
true

43. Isolated Vertices
[10]

44. Self Loop
Has self-loop: true
Self-loop count: 1

45. Complete Graph
Is complete: true

46. Tree Graph
Is tree: true

47. Remove Edge
Removed: true
Adjacency List:
1 -> [3]
2 -> [4, 5]
3 -> [1, 5]
4 -> [2, 6]
5 -> [2, 3, 6]
6 -> [4, 5]
10 -> [10]

48. Remove Vertex
Removed: true
Adjacency List:
1 -> []
2 -> [4, 5]
4 -> [2, 6]
5 -> [2, 6]
6 -> [4, 5]
10 -> [10]

49. Clone Graph
Adjacency List:
1 -> []
2 -> [4, 5]
4 -> [2, 6]
5 -> [2, 6]
6 -> [4, 5]
10 -> [10]

50. Memory Estimation
Estimated bytes: 84

51. Clear Graph
Graph empty: true
Vertex count: 0

======================================
           PROGRAM COMPLETE
======================================

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/


