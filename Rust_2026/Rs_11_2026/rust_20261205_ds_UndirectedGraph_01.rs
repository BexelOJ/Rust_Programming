//---------------------------------------------------
// UndirectedGraph - Rust Data Structures
// Date: 20261205
//---------------------------------------------------


use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::size_of_val;

// ============================================================
// 1. UNDIRECTED GRAPH STRUCTURE
// ============================================================

#[derive(Debug, Clone)]
struct UndirectedGraph {
    adjacency_list: HashMap<i32, Vec<i32>>,
}

// ============================================================
// 2. IMPLEMENTATION
// ============================================================

impl UndirectedGraph {

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

    // 8. Add undirected edge
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

    // 9. Check vertex
    fn contains_vertex(&self, vertex: i32) -> bool {
        self.adjacency_list.contains_key(&vertex)
    }

    // 10. Check undirected edge
    fn contains_edge(&self, source: i32, destination: i32) -> bool {
        self.adjacency_list
            .get(&source)
            .map_or(false, |neighbors| neighbors.contains(&destination))
    }

    // 11. Remove edge
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

    // 12. Remove vertex and connected edges
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

    // 13. Get neighbours
    fn neighbors(&self, vertex: i32) -> Option<&Vec<i32>> {
        self.adjacency_list.get(&vertex)
    }

    // 14. Calculate degree
    // An undirected self-loop contributes 2 to degree.
    fn degree(&self, vertex: i32) -> Option<usize> {
        self.adjacency_list.get(&vertex).map(|neighbors| {
            neighbors.iter()
                .map(|&neighbor| {
                    if neighbor == vertex { 2 } else { 1 }
                })
                .sum()
        })
    }

    // 15. Get vertices
    fn vertices(&self) -> Vec<i32> {
        let mut result: Vec<i32> =
            self.adjacency_list.keys().copied().collect();

        result.sort();
        result
    }

    // 16. Display graph
    fn display(&self) {
        println!("Undirected Adjacency List:");

        for vertex in self.vertices() {
            println!(
                "{} -- {:?}",
                vertex,
                self.adjacency_list.get(&vertex).unwrap()
            );
        }
    }

    // 17. Depth First Search
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

    // 19. Check path
    fn has_path(&self, source: i32, destination: i32) -> bool {
        self.bfs(source).contains(&destination)
    }

    // 20. Connected components
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

    // 21. Check connectivity
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

    // 23. Detect self-loop
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

    // 25. Check complete graph
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

    // 26. Check tree
    fn is_tree(&self) -> bool {
        if self.is_empty() || self.has_self_loop() {
            return false;
        }

        self.is_connected()
            && self.edge_count() == self.vertex_count() - 1
    }

    // 27. Detect cycle using DFS
    fn has_cycle(&self) -> bool {
        fn dfs_cycle(
            graph: &UndirectedGraph,
            vertex: i32,
            parent: Option<i32>,
            visited: &mut HashSet<i32>,
        ) -> bool {
            visited.insert(vertex);

            if let Some(neighbors) = graph.neighbors(vertex) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        if dfs_cycle(
                            graph,
                            neighbor,
                            Some(vertex),
                            visited,
                        ) {
                            return true;
                        }
                    } else if Some(neighbor) != parent {
                        return true;
                    }
                }
            }

            false
        }

        let mut visited = HashSet::new();

        for vertex in self.vertices() {
            if !visited.contains(&vertex) {
                if dfs_cycle(self, vertex, None, &mut visited) {
                    return true;
                }
            }
        }

        false
    }

    // 28. Check whether graph is bipartite
    fn is_bipartite(&self) -> bool {
        let mut colors: HashMap<i32, bool> = HashMap::new();

        for start in self.vertices() {
            if colors.contains_key(&start) {
                continue;
            }

            colors.insert(start, false);

            let mut queue = VecDeque::new();
            queue.push_back(start);

            while let Some(vertex) = queue.pop_front() {
                let current_color = colors[&vertex];

                if let Some(neighbors) = self.neighbors(vertex) {
                    for &neighbor in neighbors {
                        if !colors.contains_key(&neighbor) {
                            colors.insert(neighbor, !current_color);
                            queue.push_back(neighbor);
                        } else if colors[&neighbor] == current_color {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    // 29. Find bridges using Tarjan's algorithm
    fn bridges(&self) -> Vec<(i32, i32)> {
        fn dfs_bridge(
            graph: &UndirectedGraph,
            vertex: i32,
            parent: Option<i32>,
            timer: &mut usize,
            visited: &mut HashSet<i32>,
            tin: &mut HashMap<i32, usize>,
            low: &mut HashMap<i32, usize>,
            result: &mut Vec<(i32, i32)>,
        ) {
            visited.insert(vertex);

            *timer += 1;
            tin.insert(vertex, *timer);
            low.insert(vertex, *timer);

            if let Some(neighbors) = graph.neighbors(vertex) {
                for &neighbor in neighbors {
                    if Some(neighbor) == parent {
                        continue;
                    }

                    if visited.contains(&neighbor) {
                        let low_value = low[&vertex].min(tin[&neighbor]);
                        low.insert(vertex, low_value);
                    } else {
                        dfs_bridge(
                            graph,
                            neighbor,
                            Some(vertex),
                            timer,
                            visited,
                            tin,
                            low,
                            result,
                        );

                        let low_value = low[&vertex].min(low[&neighbor]);
                        low.insert(vertex, low_value);

                        if low[&neighbor] > tin[&vertex] {
                            result.push((
                                vertex.min(neighbor),
                                vertex.max(neighbor),
                            ));
                        }
                    }
                }
            }
        }

        let mut timer = 0;
        let mut visited = HashSet::new();
        let mut tin = HashMap::new();
        let mut low = HashMap::new();
        let mut result = Vec::new();

        for vertex in self.vertices() {
            if !visited.contains(&vertex) {
                dfs_bridge(
                    self,
                    vertex,
                    None,
                    &mut timer,
                    &mut visited,
                    &mut tin,
                    &mut low,
                    &mut result,
                );
            }
        }

        result.sort();
        result
    }

    // 30. Find articulation points using Tarjan's algorithm
    fn articulation_points(&self) -> Vec<i32> {
        fn dfs_articulation(
            graph: &UndirectedGraph,
            vertex: i32,
            parent: Option<i32>,
            timer: &mut usize,
            visited: &mut HashSet<i32>,
            tin: &mut HashMap<i32, usize>,
            low: &mut HashMap<i32, usize>,
            result: &mut HashSet<i32>,
        ) {
            visited.insert(vertex);

            *timer += 1;
            tin.insert(vertex, *timer);
            low.insert(vertex, *timer);

            let mut children = 0;

            if let Some(neighbors) = graph.neighbors(vertex) {
                for &neighbor in neighbors {
                    if Some(neighbor) == parent {
                        continue;
                    }

                    if visited.contains(&neighbor) {
                        let low_value = low[&vertex].min(tin[&neighbor]);
                        low.insert(vertex, low_value);
                    } else {
                        children += 1;

                        dfs_articulation(
                            graph,
                            neighbor,
                            Some(vertex),
                            timer,
                            visited,
                            tin,
                            low,
                            result,
                        );

                        let low_value = low[&vertex].min(low[&neighbor]);
                        low.insert(vertex, low_value);

                        if parent.is_some() && low[&neighbor] >= tin[&vertex] {
                            result.insert(vertex);
                        }
                    }
                }
            }

            if parent.is_none() && children > 1 {
                result.insert(vertex);
            }
        }

        let mut timer = 0;
        let mut visited = HashSet::new();
        let mut tin = HashMap::new();
        let mut low = HashMap::new();
        let mut result = HashSet::new();

        for vertex in self.vertices() {
            if !visited.contains(&vertex) {
                dfs_articulation(
                    self,
                    vertex,
                    None,
                    &mut timer,
                    &mut visited,
                    &mut tin,
                    &mut low,
                    &mut result,
                );
            }
        }

        let mut points: Vec<i32> = result.into_iter().collect();
        points.sort();
        points
    }

    // 31. Clear graph
    fn clear(&mut self) {
        self.adjacency_list.clear();
    }

    // 32. Clone graph
    fn clone_graph(&self) -> Self {
        self.clone()
    }

    // 33. Estimate adjacency-list storage
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
    println!("      RUST UNDIRECTED GRAPH");
    println!("======================================");

    // 35. Create graph
    let mut graph = UndirectedGraph::new();

    // 36. Add vertices
    for vertex in 1..=6 {
        graph.add_vertex(vertex);
    }

    // 37. Add edges
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 3);
    graph.add_edge(2, 4);
    graph.add_edge(3, 5);
    graph.add_edge(4, 5);
    graph.add_edge(5, 6);

    // 38. Display graph
    println!("\n1. Graph Structure");
    graph.display();

    // 39. Vertex and edge counts
    println!("\n2. Graph Size");
    println!("Vertices: {}", graph.vertex_count());
    println!("Edges: {}", graph.edge_count());

    // 40. Vertex and edge search
    println!("\n3. Search");
    println!("Vertex 3: {}", graph.contains_vertex(3));
    println!("Edge 1 -- 2: {}", graph.contains_edge(1, 2));
    println!("Edge 2 -- 1: {}", graph.contains_edge(2, 1));

    // 41. Neighbours
    println!("\n4. Neighbours");
    println!("Neighbours of 2: {:?}", graph.neighbors(2));

    // 42. Degree
    println!("\n5. Degree");
    println!("Degree of 2: {:?}", graph.degree(2));

    // 43. DFS
    println!("\n6. DFS");
    println!("{:?}", graph.dfs(1));

    // 44. BFS
    println!("\n7. BFS");
    println!("{:?}", graph.bfs(1));

    // 45. Path
    println!("\n8. Path Detection");
    println!("1 to 6: {}", graph.has_path(1, 6));

    // 46. Connected components
    println!("\n9. Connected Components");
    println!("{:?}", graph.connected_components());

    // 47. Connectivity
    println!("\n10. Connectivity");
    println!("Connected: {}", graph.is_connected());

    // 48. Isolated vertices
    println!("\n11. Isolated Vertices");
    graph.add_vertex(10);
    println!("{:?}", graph.isolated_vertices());

    // 49. Cycle detection
    println!("\n12. Cycle Detection");
    println!("Has cycle: {}", graph.has_cycle());

    // 50. Bipartite check
    println!("\n13. Bipartite Graph");
    println!("Is bipartite: {}", graph.is_bipartite());

    // 51. Complete graph
    println!("\n14. Complete Graph");

    let mut complete = UndirectedGraph::new();
    complete.add_edge(1, 2);
    complete.add_edge(1, 3);
    complete.add_edge(2, 3);

    println!("Is complete: {}", complete.is_complete());

    // 52. Tree graph
    println!("\n15. Tree Graph");

    let mut tree = UndirectedGraph::new();
    tree.add_edge(1, 2);
    tree.add_edge(1, 3);
    tree.add_edge(2, 4);

    println!("Is tree: {}", tree.is_tree());

    // 53. Bridges
    println!("\n16. Bridges");
    println!("{:?}", graph.bridges());

    // 54. Articulation points
    println!("\n17. Articulation Points");
    println!("{:?}", graph.articulation_points());

    // 55. Self-loop
    println!("\n18. Self Loop");
    graph.add_edge(10, 10);
    println!("Has self-loop: {}", graph.has_self_loop());
    println!("Self-loop count: {}", graph.self_loop_count());

    // 56. Remove edge
    println!("\n19. Remove Edge");
    println!("Removed: {}", graph.remove_edge(1, 2));
    graph.display();

    // 57. Remove vertex
    println!("\n20. Remove Vertex");
    println!("Removed: {}", graph.remove_vertex(3));
    graph.display();

    // 58. Clone graph
    println!("\n21. Clone Graph");
    let cloned = graph.clone_graph();
    cloned.display();

    // 59. Memory estimation
    println!("\n22. Memory Estimation");
    println!("Estimated bytes: {}", graph.memory_size());

    // 60. Clear graph
    println!("\n23. Clear Graph");
    graph.clear();
    println!("Is empty: {}", graph.is_empty());

    println!("\n======================================");
    println!("          PROGRAM COMPLETE");
    println!("======================================");
}


/*
//---------------------------------------------------
We'll implement:
- Vertex and edge insertion/deletion.
- Neighbour and degree operations.
- DFS and BFS traversal.
- Path detection.
- Connected components.
- Cycle detection.
- Bipartite graph checking.
- Complete graph and tree checking.
- Bridges and articulation points.
- Graph cloning and memory estimation.

//---------------------------------------------------
Important concepts:

Operation - Average / overall complexity

Add vertex  - O(1)
Add edge    - O(degree)
Remove edge     - O(degree)
Remove vertex   - O(V + E)
DFS / BFS       - O(V + E)
Cycle detection     - O(V + E)
Bipartite checking  - O(V + E)
Bridges             - O(V + E)
Articulation points - O(V + E)

//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261205_ds_UndirectedGraph_01.rs -o undirected_graph_example

./undirected_graph_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

======================================
      RUST UNDIRECTED GRAPH
======================================

1. Graph Structure
Undirected Adjacency List:
1 -- [2, 3]
2 -- [1, 3, 4]
3 -- [1, 2, 5]
4 -- [2, 5]
5 -- [3, 4, 6]
6 -- [5]

2. Graph Size
Vertices: 6
Edges: 7

3. Search
Vertex 3: true
Edge 1 -- 2: true
Edge 2 -- 1: true

4. Neighbours
Neighbours of 2: Some([1, 3, 4])

5. Degree
Degree of 2: Some(3)

6. DFS
[1, 2, 3, 5, 4, 6]

7. BFS
[1, 2, 3, 4, 5, 6]

8. Path Detection
1 to 6: true

9. Connected Components
[[1, 2, 3, 5, 4, 6]]

10. Connectivity
Connected: true

11. Isolated Vertices
[10]

12. Cycle Detection
Has cycle: true

13. Bipartite Graph
Is bipartite: false

14. Complete Graph
Is complete: true

15. Tree Graph
Is tree: true

16. Bridges
[(5, 6)]

17. Articulation Points
[5]

18. Self Loop
Has self-loop: true
Self-loop count: 1

19. Remove Edge
Removed: true
Undirected Adjacency List:
1 -- [3]
2 -- [3, 4]
3 -- [1, 2, 5]
4 -- [2, 5]
5 -- [3, 4, 6]
6 -- [5]
10 -- [10]

20. Remove Vertex
Removed: true
Undirected Adjacency List:
1 -- []
2 -- [4]
4 -- [2, 5]
5 -- [4, 6]
6 -- [5]
10 -- [10]

21. Clone Graph
Undirected Adjacency List:
1 -- []
2 -- [4]
4 -- [2, 5]
5 -- [4, 6]
6 -- [5]
10 -- [10]

22. Memory Estimation
Estimated bytes: 76

23. Clear Graph
Is empty: true

======================================
          PROGRAM COMPLETE
======================================

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/


