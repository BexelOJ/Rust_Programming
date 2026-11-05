//---------------------------------------------------
// Graph - Rust Data Structures
// Date: 20261201
//---------------------------------------------------

use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::size_of;

// ============================================================
// 01. GRAPH DATA STRUCTURE
// ============================================================

#[derive(Debug, Clone)]
struct Graph {
    adjacency_list: HashMap<i32, HashSet<i32>>,
}

// ============================================================
// 02. GRAPH IMPLEMENTATION
// ============================================================

impl Graph {

    // 03. CREATE GRAPH
    fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    // 04. CHECK EMPTY
    fn is_empty(&self) -> bool {
        self.adjacency_list.is_empty()
    }

    // 05. VERTEX COUNT
    fn vertex_count(&self) -> usize {
        self.adjacency_list.len()
    }

    // 06. EDGE COUNT
    fn edge_count(&self) -> usize {
        self.adjacency_list
            .values()
            .map(|neighbors| neighbors.len())
            .sum::<usize>() / 2
    }

    // 07. ADD VERTEX
    fn add_vertex(&mut self, vertex: i32) -> bool {
        if self.adjacency_list.contains_key(&vertex) {
            return false;
        }

        self.adjacency_list.insert(vertex, HashSet::new());
        true
    }

    // 08. ADD UNDIRECTED EDGE
    fn add_edge(&mut self, source: i32, destination: i32) -> bool {
        self.add_vertex(source);
        self.add_vertex(destination);

        let inserted = self.adjacency_list
            .get_mut(&source)
            .unwrap()
            .insert(destination);

        self.adjacency_list
            .get_mut(&destination)
            .unwrap()
            .insert(source);

        inserted
    }

    // 09. CHECK VERTEX
    fn contains_vertex(&self, vertex: i32) -> bool {
        self.adjacency_list.contains_key(&vertex)
    }

    // 10. CHECK EDGE
    fn contains_edge(&self, source: i32, destination: i32) -> bool {
        self.adjacency_list
            .get(&source)
            .map(|neighbors| neighbors.contains(&destination))
            .unwrap_or(false)
    }

    // 11. REMOVE EDGE
    fn remove_edge(&mut self, source: i32, destination: i32) -> bool {
        let removed = self.adjacency_list
            .get_mut(&source)
            .map(|neighbors| neighbors.remove(&destination))
            .unwrap_or(false);

        if removed {
            if let Some(neighbors) = self.adjacency_list.get_mut(&destination) {
                neighbors.remove(&source);
            }
        }

        removed
    }

    // 12. REMOVE VERTEX
    fn remove_vertex(&mut self, vertex: i32) -> bool {
        let neighbors = match self.adjacency_list.remove(&vertex) {
            Some(neighbors) => neighbors,
            None => return false,
        };

        for neighbor in neighbors {
            if let Some(list) = self.adjacency_list.get_mut(&neighbor) {
                list.remove(&vertex);
            }
        }

        true
    }

    // 13. GET NEIGHBORS
    fn neighbors(&self, vertex: i32) -> Option<Vec<i32>> {
        self.adjacency_list.get(&vertex).map(|neighbors| {
            let mut result: Vec<i32> = neighbors.iter().copied().collect();
            result.sort_unstable();
            result
        })
    }

    // 14. VERTEX DEGREE
    fn degree(&self, vertex: i32) -> Option<usize> {
        self.adjacency_list
            .get(&vertex)
            .map(|neighbors| neighbors.len())
    }

    // 15. GET ALL VERTICES
    fn vertices(&self) -> Vec<i32> {
        let mut result: Vec<i32> =
            self.adjacency_list.keys().copied().collect();

        result.sort_unstable();
        result
    }

    // 16. DISPLAY GRAPH
    fn display(&self) {
        for vertex in self.vertices() {
            println!(
                "{} -> {:?}",
                vertex,
                self.neighbors(vertex).unwrap()
            );
        }
    }

    // 17. DEPTH FIRST SEARCH
    fn dfs(&self, start: i32) -> Vec<i32> {
        if !self.contains_vertex(start) {
            return Vec::new();
        }

        let mut visited = HashSet::new();
        let mut stack = vec![start];
        let mut result = Vec::new();

        while let Some(vertex) = stack.pop() {
            if !visited.insert(vertex) {
                continue;
            }

            result.push(vertex);

            if let Some(neighbors) = self.neighbors(vertex) {
                for neighbor in neighbors.into_iter().rev() {
                    if !visited.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }

        result
    }

    // 18. BREADTH FIRST SEARCH
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
                for neighbor in neighbors {
                    if visited.insert(neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        result
    }

    // 19. CHECK PATH BETWEEN TWO VERTICES
    fn has_path(&self, source: i32, destination: i32) -> bool {
        if !self.contains_vertex(source)
            || !self.contains_vertex(destination)
        {
            return false;
        }

        if source == destination {
            return true;
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(source);
        queue.push_back(source);

        while let Some(vertex) = queue.pop_front() {
            if vertex == destination {
                return true;
            }

            if let Some(neighbors) = self.adjacency_list.get(&vertex) {
                for &neighbor in neighbors {
                    if visited.insert(neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        false
    }

    // 20. CONNECTED COMPONENTS
    fn connected_components(&self) -> Vec<Vec<i32>> {
        let mut visited = HashSet::new();
        let mut components = Vec::new();

        for start in self.vertices() {
            if visited.contains(&start) {
                continue;
            }

            let mut component = Vec::new();
            let mut queue = VecDeque::new();

            visited.insert(start);
            queue.push_back(start);

            while let Some(vertex) = queue.pop_front() {
                component.push(vertex);

                if let Some(neighbors) = self.neighbors(vertex) {
                    for neighbor in neighbors {
                        if visited.insert(neighbor) {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }

            component.sort_unstable();
            components.push(component);
        }

        components
    }

    // 21. CHECK CONNECTIVITY
    fn is_connected(&self) -> bool {
        self.is_empty() || self.connected_components().len() == 1
    }

    // 22. FIND ISOLATED VERTICES
    fn isolated_vertices(&self) -> Vec<i32> {
        self.vertices()
            .into_iter()
            .filter(|vertex| self.degree(*vertex) == Some(0))
            .collect()
    }

    // 23. CHECK SELF LOOP
    fn has_self_loop(&self, vertex: i32) -> bool {
        self.contains_edge(vertex, vertex)
    }

    // 24. COUNT SELF LOOPS
    fn self_loop_count(&self) -> usize {
        self.adjacency_list
            .iter()
            .filter(|(vertex, neighbors)| neighbors.contains(vertex))
            .count()
    }

    // 25. CHECK COMPLETE GRAPH
    fn is_complete(&self) -> bool {
        let vertices = self.vertex_count();

        self.adjacency_list
            .values()
            .all(|neighbors| neighbors.len() == vertices - 1)
    }

    // 26. CHECK WHETHER GRAPH IS A TREE
    fn is_tree(&self) -> bool {
        !self.is_empty()
            && self.is_connected()
            && self.edge_count() == self.vertex_count() - 1
    }

    // 27. CHECK REVERSE EDGE
    // In an undirected graph, reverse connectivity is automatic.
    fn has_reverse_edge(&self, source: i32, destination: i32) -> bool {
        self.contains_edge(destination, source)
    }

    // 28. CLEAR GRAPH
    fn clear(&mut self) {
        self.adjacency_list.clear();
    }

    // 29. CLONE GRAPH
    fn clone_graph(&self) -> Self {
        self.clone()
    }

    // 30. GRAPH DESCRIPTOR MEMORY SIZE
    fn memory_size(&self) -> usize {
        size_of::<Self>()
    }
}

// ============================================================
// 31. MAIN FUNCTION
// ============================================================

fn main() {

    // 32. CREATE GRAPH
    println!("\n32. CREATE GRAPH");

    let mut graph = Graph::new();

    println!("Is empty: {}", graph.is_empty());

    // 33. ADD VERTICES
    println!("\n33. ADD VERTICES");

    for vertex in 1..=6 {
        graph.add_vertex(vertex);
    }

    println!("Vertices: {:?}", graph.vertices());

    // 34. ADD EDGES
    println!("\n34. ADD EDGES");

    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 4);
    graph.add_edge(2, 5);
    graph.add_edge(3, 5);
    graph.add_edge(4, 6);
    graph.add_edge(5, 6);

    // 35. DISPLAY GRAPH
    println!("\n35. DISPLAY GRAPH");
    graph.display();

    // 36. VERTEX AND EDGE COUNT
    println!("\n36. VERTEX AND EDGE COUNT");

    println!("Vertices: {}", graph.vertex_count());
    println!("Edges   : {}", graph.edge_count());

    // 37. CHECK VERTEX
    println!("\n37. CHECK VERTEX");

    println!("Vertex 3: {}", graph.contains_vertex(3));
    println!("Vertex 9: {}", graph.contains_vertex(9));

    // 38. CHECK EDGE
    println!("\n38. CHECK EDGE");

    println!("1 - 2: {}", graph.contains_edge(1, 2));
    println!("1 - 6: {}", graph.contains_edge(1, 6));

    // 39. GET NEIGHBORS
    println!("\n39. GET NEIGHBORS");

    println!("Neighbors of 2: {:?}", graph.neighbors(2));
    println!("Neighbors of 5: {:?}", graph.neighbors(5));

    // 40. VERTEX DEGREE
    println!("\n40. VERTEX DEGREE");

    for vertex in graph.vertices() {
        println!("Degree {}: {:?}", vertex, graph.degree(vertex));
    }

    // 41. DFS
    println!("\n41. DEPTH FIRST SEARCH");

    println!("DFS from 1: {:?}", graph.dfs(1));

    // 42. BFS
    println!("\n42. BREADTH FIRST SEARCH");

    println!("BFS from 1: {:?}", graph.bfs(1));

    // 43. PATH DETECTION
    println!("\n43. PATH DETECTION");

    println!("1 to 6: {}", graph.has_path(1, 6));
    println!("1 to 9: {}", graph.has_path(1, 9));

    // 44. CONNECTED COMPONENTS
    println!("\n44. CONNECTED COMPONENTS");

    println!("{:?}", graph.connected_components());

    // 45. CONNECTIVITY
    println!("\n45. CONNECTIVITY");

    println!("Connected: {}", graph.is_connected());

    // 46. ISOLATED VERTICES
    println!("\n46. ISOLATED VERTICES");

    graph.add_vertex(7);

    println!("Isolated: {:?}", graph.isolated_vertices());

    // 47. SELF LOOP
    println!("\n47. SELF LOOP");

    graph.add_edge(7, 7);

    println!("Vertex 7 self loop: {}", graph.has_self_loop(7));
    println!("Self loop count: {}", graph.self_loop_count());

    // 48. COMPLETE GRAPH
    println!("\n48. COMPLETE GRAPH");

    println!("Is complete: {}", graph.is_complete());

    // 49. TREE GRAPH
    println!("\n49. TREE GRAPH");

    let mut tree_graph = Graph::new();

    tree_graph.add_edge(1, 2);
    tree_graph.add_edge(1, 3);
    tree_graph.add_edge(2, 4);

    println!("Is tree: {}", tree_graph.is_tree());

    // 50. REVERSE EDGE
    println!("\n50. REVERSE EDGE");

    println!("2 -> 1: {}", graph.has_reverse_edge(2, 1));

    // 51. REMOVE EDGE
    println!("\n51. REMOVE EDGE");

    println!("Removed: {}", graph.remove_edge(1, 2));
    println!("Exists: {}", graph.contains_edge(1, 2));

    // 52. REMOVE VERTEX
    println!("\n52. REMOVE VERTEX");

    println!("Removed vertex 6: {}", graph.remove_vertex(6));
    graph.display();

    // 53. CLONE GRAPH
    println!("\n53. CLONE GRAPH");

    let cloned_graph = graph.clone_graph();

    println!("Original vertices: {}", graph.vertex_count());
    println!("Cloned vertices: {}", cloned_graph.vertex_count());

    // 54. MEMORY SIZE
    println!("\n54. MEMORY SIZE");

    println!("Graph descriptor: {} bytes", graph.memory_size());
    println!("Note: Heap allocations are not included.");

    // 55. CLEAR GRAPH
    println!("\n55. CLEAR GRAPH");

    graph.clear();

    println!("Is empty: {}", graph.is_empty());
    println!("Vertices: {}", graph.vertex_count());
    println!("Edges: {}", graph.edge_count());

    // 56. FINAL STATUS
    println!("\n56. FINAL STATUS");

    println!("Graph demonstration completed successfully.");
}


/*
//---------------------------------------------------
The implementation uses:

HashMap — stores vertices and their adjacency lists.
HashSet — stores unique neighboring vertices.
VecDeque — supports BFS traversal.

//---------------------------------------------------
Complexity:

Let 
V = vertices and 
E = edges.

Operation - Average complexity

Add vertex              - O(1)
Add edge                - O(1)
Search vertex           - O(1)
Search edge             - O(1)
Remove vertex           - O(V + E)
DFS                     - O(V + E)
BFS                     - O(V + E)
Connected components    - O(V + E)

Note: HashMap and HashSet provide average-case O(1) lookups.

//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261201_ds_Graph_01.rs -o graph_example

./graph_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

32. CREATE GRAPH
Is empty: true

33. ADD VERTICES
Vertices: [1, 2, 3, 4, 5, 6]

34. ADD EDGES

35. DISPLAY GRAPH
1 -> [2, 3]
2 -> [1, 4, 5]
3 -> [1, 5]
4 -> [2, 6]
5 -> [2, 3, 6]
6 -> [4, 5]

36. VERTEX AND EDGE COUNT
Vertices: 6
Edges   : 7

37. CHECK VERTEX
Vertex 3: true
Vertex 9: false

38. CHECK EDGE
1 - 2: true
1 - 6: false

39. GET NEIGHBORS
Neighbors of 2: Some([1, 4, 5])
Neighbors of 5: Some([2, 3, 6])

40. VERTEX DEGREE
Degree 1: Some(2)
Degree 2: Some(3)
Degree 3: Some(2)
Degree 4: Some(2)
Degree 5: Some(3)
Degree 6: Some(2)

41. DEPTH FIRST SEARCH
DFS from 1: [1, 2, 4, 6, 5, 3]

42. BREADTH FIRST SEARCH
BFS from 1: [1, 2, 3, 4, 5, 6]

43. PATH DETECTION
1 to 6: true
1 to 9: false

44. CONNECTED COMPONENTS
[[1, 2, 3, 4, 5, 6]]

45. CONNECTIVITY
Connected: true

46. ISOLATED VERTICES
Isolated: [7]

47. SELF LOOP
Vertex 7 self loop: true
Self loop count: 1

48. COMPLETE GRAPH
Is complete: false

49. TREE GRAPH
Is tree: true

50. REVERSE EDGE
2 -> 1: true

51. REMOVE EDGE
Removed: true
Exists: false

52. REMOVE VERTEX
Removed vertex 6: true
1 -> [3]
2 -> [4, 5]
3 -> [1, 5]
4 -> [2]
5 -> [2, 3]
7 -> [7]

53. CLONE GRAPH
Original vertices: 6
Cloned vertices: 6

54. MEMORY SIZE
Graph descriptor: 48 bytes
Note: Heap allocations are not included.

55. CLEAR GRAPH
Is empty: true
Vertices: 0
Edges: 0

56. FINAL STATUS
Graph demonstration completed successfully.

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/


