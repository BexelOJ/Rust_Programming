//---------------------------------------------------
// WeightedGraph - Rust Data Structures
// Date: 20261206
//---------------------------------------------------


use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::mem::size_of_val;

// ============================================================
// 1. WEIGHTED GRAPH STRUCTURE
// ============================================================

#[derive(Debug, Clone)]
struct WeightedGraph {
    adjacency_list: HashMap<i32, Vec<(i32, i32)>>,
}

// ============================================================
// 2. IMPLEMENTATION
// ============================================================

impl WeightedGraph {

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

    // 6. Get undirected edge count
    fn edge_count(&self) -> usize {
        self.adjacency_list
            .values()
            .map(|neighbors| neighbors.len())
            .sum::<usize>() / 2
    }

    // 7. Add vertex
    fn add_vertex(&mut self, vertex: i32) {
        self.adjacency_list.entry(vertex).or_default();
    }

    // 8. Add weighted undirected edge
    // Negative weights are allowed for storage,
    // but Dijkstra's algorithm requires non-negative weights.
    fn add_edge(&mut self, source: i32, destination: i32, weight: i32) {
        self.add_vertex(source);
        self.add_vertex(destination);

        if source == destination {
            let neighbors = self.adjacency_list.get_mut(&source).unwrap();

            if let Some(edge) = neighbors.iter_mut().find(|edge| edge.0 == destination) {
                edge.1 = weight;
            } else {
                neighbors.push((destination, weight));
            }

            return;
        }

        let source_neighbors = self.adjacency_list.get_mut(&source).unwrap();

        if let Some(edge) = source_neighbors.iter_mut()
            .find(|edge| edge.0 == destination)
        {
            edge.1 = weight;
        } else {
            source_neighbors.push((destination, weight));
        }

        let destination_neighbors =
            self.adjacency_list.get_mut(&destination).unwrap();

        if let Some(edge) = destination_neighbors.iter_mut()
            .find(|edge| edge.0 == source)
        {
            edge.1 = weight;
        } else {
            destination_neighbors.push((source, weight));
        }
    }

    // 9. Check vertex
    fn contains_vertex(&self, vertex: i32) -> bool {
        self.adjacency_list.contains_key(&vertex)
    }

    // 10. Check edge
    fn contains_edge(&self, source: i32, destination: i32) -> bool {
        self.adjacency_list
            .get(&source)
            .map_or(false, |neighbors| {
                neighbors.iter().any(|edge| edge.0 == destination)
            })
    }

    // 11. Get edge weight
    fn get_weight(&self, source: i32, destination: i32) -> Option<i32> {
        self.adjacency_list.get(&source)?.iter()
            .find(|edge| edge.0 == destination)
            .map(|edge| edge.1)
    }

    // 12. Update edge weight
    fn update_weight(
        &mut self,
        source: i32,
        destination: i32,
        new_weight: i32,
    ) -> bool {
        if !self.contains_edge(source, destination) {
            return false;
        }

        self.add_edge(source, destination, new_weight);
        true
    }

    // 13. Remove edge
    fn remove_edge(&mut self, source: i32, destination: i32) -> bool {
        if !self.contains_edge(source, destination) {
            return false;
        }

        if let Some(neighbors) = self.adjacency_list.get_mut(&source) {
            neighbors.retain(|edge| edge.0 != destination);
        }

        if source != destination {
            if let Some(neighbors) = self.adjacency_list.get_mut(&destination) {
                neighbors.retain(|edge| edge.0 != source);
            }
        }

        true
    }

    // 14. Remove vertex and its edges
    fn remove_vertex(&mut self, vertex: i32) -> bool {
        let neighbors = match self.adjacency_list.remove(&vertex) {
            Some(neighbors) => neighbors,
            None => return false,
        };

        for (neighbor, _) in neighbors {
            if let Some(adjacent) = self.adjacency_list.get_mut(&neighbor) {
                adjacent.retain(|edge| edge.0 != vertex);
            }
        }

        true
    }

    // 15. Get neighbours and weights
    fn neighbors(&self, vertex: i32) -> Option<&Vec<(i32, i32)>> {
        self.adjacency_list.get(&vertex)
    }

    // 16. Calculate degree
    // A self-loop contributes 2 to undirected degree.
    fn degree(&self, vertex: i32) -> Option<usize> {
        self.adjacency_list.get(&vertex).map(|neighbors| {
            neighbors.iter()
                .map(|&(neighbor, _)| {
                    if neighbor == vertex { 2 } else { 1 }
                })
                .sum()
        })
    }

    // 17. Calculate weighted degree (sum of incident edge weights)
    // A self-loop contributes twice.
    fn weighted_degree(&self, vertex: i32) -> Option<i32> {
        self.adjacency_list.get(&vertex).map(|neighbors| {
            neighbors.iter()
                .map(|&(neighbor, weight)| {
                    if neighbor == vertex {
                        weight.saturating_mul(2)
                    } else {
                        weight
                    }
                })
                .fold(0i32, i32::saturating_add)
        })
    }

    // 18. Get all vertices
    fn vertices(&self) -> Vec<i32> {
        let mut result: Vec<i32> =
            self.adjacency_list.keys().copied().collect();

        result.sort();
        result
    }

    // 19. Display weighted adjacency list
    fn display(&self) {
        println!("Weighted Adjacency List:");

        for vertex in self.vertices() {
            println!(
                "{} -> {:?}",
                vertex,
                self.adjacency_list.get(&vertex).unwrap()
            );
        }
    }

    // 20. Calculate total weight of undirected edges
    // Each edge is counted once.
    fn total_weight(&self) -> i64 {
        let mut total = 0i64;

        for (&source, neighbors) in &self.adjacency_list {
            for &(destination, weight) in neighbors {
                if source <= destination {
                    total += i64::from(weight);
                }
            }
        }

        total
    }

    // 21. Find minimum edge weight
    fn minimum_edge_weight(&self) -> Option<i32> {
        self.adjacency_list.values()
            .flat_map(|neighbors| neighbors.iter().map(|edge| edge.1))
            .min()
    }

    // 22. Find maximum edge weight
    fn maximum_edge_weight(&self) -> Option<i32> {
        self.adjacency_list.values()
            .flat_map(|neighbors| neighbors.iter().map(|edge| edge.1))
            .max()
    }

    // 23. DFS traversal
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
                    for &(neighbor, _) in neighbors.iter().rev() {
                        if !visited.contains(&neighbor) {
                            stack.push(neighbor);
                        }
                    }
                }
            }
        }

        result
    }

    // 24. BFS traversal
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
                for &(neighbor, _) in neighbors {
                    if visited.insert(neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        result
    }

    // 25. Calculate weight of a specified path
    fn path_weight(&self, path: &[i32]) -> Option<i64> {
        let mut total = 0i64;

        for pair in path.windows(2) {
            let weight = self.get_weight(pair[0], pair[1])?;
            total += i64::from(weight);
        }

        Some(total)
    }

    // 26. Dijkstra's shortest path
    // Returns (distance, path).
    // Negative weights are rejected.
    fn dijkstra(&self, start: i32, destination: i32) -> Option<(i64, Vec<i32>)> {
        if !self.contains_vertex(start) || !self.contains_vertex(destination) {
            return None;
        }

        if self.adjacency_list.values().any(|neighbors| {
            neighbors.iter().any(|&(_, weight)| weight < 0)
        }) {
            return None;
        }

        let mut distances: HashMap<i32, i64> = HashMap::new();
        let mut previous: HashMap<i32, i32> = HashMap::new();

        for vertex in self.vertices() {
            distances.insert(vertex, i64::MAX);
        }

        distances.insert(start, 0);

        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0i64, start)));

        while let Some(Reverse((current_distance, vertex))) = heap.pop() {
            if current_distance > distances[&vertex] {
                continue;
            }

            if vertex == destination {
                break;
            }

            if let Some(neighbors) = self.neighbors(vertex) {
                for &(neighbor, weight) in neighbors {
                    let new_distance =
                        current_distance + i64::from(weight);

                    if new_distance < distances[&neighbor] {
                        distances.insert(neighbor, new_distance);
                        previous.insert(neighbor, vertex);

                        heap.push(Reverse((new_distance, neighbor)));
                    }
                }
            }
        }

        let distance = *distances.get(&destination)?;

        if distance == i64::MAX {
            return None;
        }

        let mut path = vec![destination];
        let mut current = destination;

        while current != start {
            current = *previous.get(&current)?;
            path.push(current);
        }

        path.reverse();

        Some((distance, path))
    }

    // 27. Prim's Minimum Spanning Tree
    // Returns (selected edges, total weight).
    // Requires a connected, non-empty graph and non-negative weights.
    fn prim_mst(&self, start: i32) -> Option<(Vec<(i32, i32, i32)>, i64)> {
        if !self.contains_vertex(start) {
            return None;
        }

        if self.adjacency_list.values().any(|neighbors| {
            neighbors.iter().any(|&(_, weight)| weight < 0)
        }) {
            return None;
        }

        let mut visited = HashSet::new();
        let mut heap = BinaryHeap::new();
        let mut mst_edges = Vec::new();
        let mut total_weight = 0i64;

        visited.insert(start);

        if let Some(neighbors) = self.neighbors(start) {
            for &(neighbor, weight) in neighbors {
                heap.push(Reverse((weight, start, neighbor)));
            }
        }

        while let Some(Reverse((weight, source, destination))) = heap.pop() {
            if visited.contains(&destination) {
                continue;
            }

            visited.insert(destination);
            mst_edges.push((source, destination, weight));
            total_weight += i64::from(weight);

            if let Some(neighbors) = self.neighbors(destination) {
                for &(neighbor, next_weight) in neighbors {
                    if !visited.contains(&neighbor) {
                        heap.push(Reverse((
                            next_weight,
                            destination,
                            neighbor,
                        )));
                    }
                }
            }
        }

        if visited.len() != self.vertex_count() {
            return None;
        }

        Some((mst_edges, total_weight))
    }

    // 28. Reverse graph
    // For an undirected graph, reversing edges produces the same graph.
    fn reverse_graph(&self) -> Self {
        let mut reversed = Self::new();

        for vertex in self.vertices() {
            reversed.add_vertex(vertex);
        }

        for (&source, neighbors) in &self.adjacency_list {
            for &(destination, weight) in neighbors {
                if source <= destination {
                    reversed.add_edge(destination, source, weight);
                }
            }
        }

        reversed
    }

    // 29. Check whether graph has a self-loop
    fn has_self_loop(&self) -> bool {
        self.adjacency_list.iter().any(|(&vertex, neighbors)| {
            neighbors.iter().any(|&(neighbor, _)| neighbor == vertex)
        })
    }

    // 30. Check whether graph is connected
    fn is_connected(&self) -> bool {
        if self.is_empty() {
            return true;
        }

        self.bfs(self.vertices()[0]).len() == self.vertex_count()
    }

    // 31. Clear graph
    fn clear(&mut self) {
        self.adjacency_list.clear();
    }

    // 32. Clone graph
    fn clone_graph(&self) -> Self {
        self.clone()
    }

    // 33. Estimate adjacency-list element storage
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
// 34. MAIN FUNCTION
// ============================================================

fn main() {

    println!("======================================");
    println!("       RUST WEIGHTED GRAPH");
    println!("======================================");

    // 35. Create graph
    let mut graph = WeightedGraph::new();

    // 36. Add vertices
    for vertex in 1..=6 {
        graph.add_vertex(vertex);
    }

    // 37. Add weighted edges
    graph.add_edge(1, 2, 10);
    graph.add_edge(1, 3, 5);
    graph.add_edge(2, 3, 8);
    graph.add_edge(2, 4, 15);
    graph.add_edge(3, 4, 20);
    graph.add_edge(3, 5, 12);
    graph.add_edge(4, 5, 7);
    graph.add_edge(4, 6, 9);
    graph.add_edge(5, 6, 6);

    // 38. Display graph
    println!("\n1. Weighted Graph");
    graph.display();

    // 39. Vertex and edge counts
    println!("\n2. Graph Size");
    println!("Vertices: {}", graph.vertex_count());
    println!("Edges: {}", graph.edge_count());

    // 40. Check edge
    println!("\n3. Edge Search");
    println!("1 -- 2: {}", graph.contains_edge(1, 2));

    // 41. Get edge weight
    println!("\n4. Edge Weight");
    println!("Weight 1 -- 2: {:?}", graph.get_weight(1, 2));

    // 42. Update weight
    println!("\n5. Update Weight");
    println!("Updated: {}", graph.update_weight(1, 2, 11));
    println!("New weight: {:?}", graph.get_weight(1, 2));

    // 43. Neighbours
    println!("\n6. Neighbours");
    println!("{:?}", graph.neighbors(3));

    // 44. Degree
    println!("\n7. Degree");
    println!("Degree of 3: {:?}", graph.degree(3));

    // 45. Weighted degree
    println!("\n8. Weighted Degree");
    println!("Weighted degree of 3: {:?}", graph.weighted_degree(3));

    // 46. Total weight
    println!("\n9. Total Graph Weight");
    println!("{}", graph.total_weight());

    // 47. Minimum edge weight
    println!("\n10. Minimum Edge Weight");
    println!("{:?}", graph.minimum_edge_weight());

    // 48. Maximum edge weight
    println!("\n11. Maximum Edge Weight");
    println!("{:?}", graph.maximum_edge_weight());

    // 49. DFS
    println!("\n12. DFS");
    println!("{:?}", graph.dfs(1));

    // 50. BFS
    println!("\n13. BFS");
    println!("{:?}", graph.bfs(1));

    // 51. Path weight
    println!("\n14. Path Weight");
    let path = vec![1, 3, 5, 6];
    println!("Path: {:?}", path);
    println!("Total path weight: {:?}", graph.path_weight(&path));

    // 52. Dijkstra
    println!("\n15. Dijkstra Shortest Path");

    if let Some((distance, path)) = graph.dijkstra(1, 6) {
        println!("Shortest distance: {}", distance);
        println!("Shortest path: {:?}", path);
    } else {
        println!("No valid shortest path");
    }

    // 53. Prim MST
    println!("\n16. Prim Minimum Spanning Tree");

    if let Some((edges, total)) = graph.prim_mst(1) {
        println!("MST edges:");

        for (source, destination, weight) in edges {
            println!("{} -- {} : {}", source, destination, weight);
        }

        println!("MST total weight: {}", total);
    } else {
        println!("MST cannot be generated");
    }

    // 54. Reverse graph
    println!("\n17. Reverse Graph");
    let reversed = graph.reverse_graph();
    reversed.display();

    // 55. Connectivity
    println!("\n18. Connectivity");
    println!("Connected: {}", graph.is_connected());

    // 56. Self-loop
    println!("\n19. Self Loop");
    graph.add_edge(6, 6, 4);
    println!("Has self-loop: {}", graph.has_self_loop());

    // 57. Remove edge
    println!("\n20. Remove Edge");
    println!("Removed: {}", graph.remove_edge(1, 3));
    graph.display();

    // 58. Remove vertex
    println!("\n21. Remove Vertex");
    println!("Removed: {}", graph.remove_vertex(5));
    graph.display();

    // 59. Clone graph
    println!("\n22. Clone Graph");
    let cloned = graph.clone_graph();
    cloned.display();

    // 60. Memory estimation
    println!("\n23. Memory Estimation");
    println!("Estimated bytes: {}", graph.memory_size());

    // 61. Clear graph
    println!("\n24. Clear Graph");
    graph.clear();
    println!("Is empty: {}", graph.is_empty());

    println!("\n======================================");
    println!("          PROGRAM COMPLETE");
    println!("======================================");
}

/*
//---------------------------------------------------
We'll implement:

- Weighted vertex and edge operations.
- Edge weight search and update.
- Minimum and maximum edge weights.
- Total graph weight.
- Weighted DFS and BFS.
- Path weight calculation.
- Shortest path using Dijkstra's algorithm.
- Minimum spanning tree using Prim's algorithm.
- Graph reversal, cloning and memory estimation.

//---------------------------------------------------
Important concepts:

Operation - Complexity

Add vertex  - O(1) average
Add edge    - O(degree)
Search edge     - O(degree)
Update weight   - O(degree)
DFS / BFS       - O(V + E)
Dijkstra (binary heap)  - O((V + E) log V)
Prim MST (binary heap)  - O(E log E)

Three important distinctions:
1. Weighted degree: Sum of weights incident to a vertex.
2. Shortest path: Finds the minimum-cost route between two vertices. This example uses Dijkstra, which requires non-negative edge weights.
3. Minimum Spanning Tree (MST): Connects every vertex with the minimum possible total edge weight, without cycles.

//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261206_ds_WeightedGraph_01.rs -o weighted_graph_example

./weighted_graph_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

======================================
       RUST WEIGHTED GRAPH
======================================

1. Weighted Graph
Weighted Adjacency List:
1 -> [(2, 10), (3, 5)]
2 -> [(1, 10), (3, 8), (4, 15)]
3 -> [(1, 5), (2, 8), (4, 20), (5, 12)]
4 -> [(2, 15), (3, 20), (5, 7), (6, 9)]
5 -> [(3, 12), (4, 7), (6, 6)]
6 -> [(4, 9), (5, 6)]

2. Graph Size
Vertices: 6
Edges: 9

3. Edge Search
1 -- 2: true

4. Edge Weight
Weight 1 -- 2: Some(10)

5. Update Weight
Updated: true
New weight: Some(11)

6. Neighbours
Some([(1, 5), (2, 8), (4, 20), (5, 12)])

7. Degree
Degree of 3: Some(4)

8. Weighted Degree
Weighted degree of 3: Some(45)

9. Total Graph Weight
93

10. Minimum Edge Weight
Some(5)

11. Maximum Edge Weight
Some(20)

12. DFS
[1, 2, 3, 4, 5, 6]

13. BFS
[1, 2, 3, 4, 5, 6]

14. Path Weight
Path: [1, 3, 5, 6]
Total path weight: Some(23)

15. Dijkstra Shortest Path
Shortest distance: 23
Shortest path: [1, 3, 5, 6]

16. Prim Minimum Spanning Tree
MST edges:
1 -- 3 : 5
3 -- 2 : 8
3 -- 5 : 12
5 -- 6 : 6
5 -- 4 : 7
MST total weight: 38

17. Reverse Graph
Weighted Adjacency List:
1 -> [(2, 11), (3, 5)]
2 -> [(1, 11), (3, 8), (4, 15)]
3 -> [(4, 20), (5, 12), (1, 5), (2, 8)]
4 -> [(3, 20), (5, 7), (6, 9), (2, 15)]
5 -> [(3, 12), (6, 6), (4, 7)]
6 -> [(5, 6), (4, 9)]

18. Connectivity
Connected: true

19. Self Loop
Has self-loop: true

20. Remove Edge
Removed: true
Weighted Adjacency List:
1 -> [(2, 11)]
2 -> [(1, 11), (3, 8), (4, 15)]
3 -> [(2, 8), (4, 20), (5, 12)]
4 -> [(2, 15), (3, 20), (5, 7), (6, 9)]
5 -> [(3, 12), (4, 7), (6, 6)]
6 -> [(4, 9), (5, 6), (6, 4)]

21. Remove Vertex
Removed: true
Weighted Adjacency List:
1 -> [(2, 11)]
2 -> [(1, 11), (3, 8), (4, 15)]
3 -> [(2, 8), (4, 20)]
4 -> [(2, 15), (3, 20), (6, 9)]
6 -> [(4, 9), (6, 4)]

22. Clone Graph
Weighted Adjacency List:
1 -> [(2, 11)]
2 -> [(1, 11), (3, 8), (4, 15)]
3 -> [(2, 8), (4, 20)]
4 -> [(2, 15), (3, 20), (6, 9)]
6 -> [(4, 9), (6, 4)]

23. Memory Estimation
Estimated bytes: 136

24. Clear Graph
Is empty: true

======================================
          PROGRAM COMPLETE
======================================

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/



