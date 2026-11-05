//---------------------------------------------------
// AdjacencyMatrix - Rust Data Structures
// Date: 20261202
//---------------------------------------------------

use std::mem::size_of_val;

// ============================================================
// 01. ADJACENCY MATRIX GRAPH
// ============================================================

#[derive(Debug, Clone)]
struct AdjacencyMatrix {
    vertices: Vec<i32>,
    matrix: Vec<Vec<u8>>,
}

// ============================================================
// 02. IMPLEMENTATION
// ============================================================

impl AdjacencyMatrix {

    // 03. CREATE EMPTY GRAPH
    fn new() -> Self {
        Self {
            vertices: Vec::new(),
            matrix: Vec::new(),
        }
    }

    // 04. CHECK EMPTY
    fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    // 05. VERTEX COUNT
    fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    // 06. EDGE COUNT
    fn edge_count(&self) -> usize {
        let mut count = 0;

        for i in 0..self.matrix.len() {
            for j in (i + 1)..self.matrix.len() {
                if self.matrix[i][j] == 1 {
                    count += 1;
                }
            }

            // Count a self-loop once.
            if self.matrix[i][i] == 1 {
                count += 1;
            }
        }

        count
    }

    // 07. FIND VERTEX INDEX
    fn find_index(&self, vertex: i32) -> Option<usize> {
        self.vertices.iter().position(|&v| v == vertex)
    }

    // 08. ADD VERTEX
    fn add_vertex(&mut self, vertex: i32) -> bool {
        if self.find_index(vertex).is_some() {
            return false;
        }

        let old_size = self.vertices.len();

        // Add a new column to every existing row.
        for row in &mut self.matrix {
            row.push(0);
        }

        // Add a new row.
        self.matrix.push(vec![0; old_size + 1]);

        self.vertices.push(vertex);

        true
    }

    // 09. ADD UNDIRECTED EDGE
    fn add_edge(&mut self, source: i32, destination: i32) -> bool {
        let source_index = match self.find_index(source) {
            Some(index) => index,
            None => {
                self.add_vertex(source);
                self.vertices.len() - 1
            }
        };

        let destination_index = match self.find_index(destination) {
            Some(index) => index,
            None => {
                self.add_vertex(destination);
                self.vertices.len() - 1
            }
        };

        let inserted = self.matrix[source_index][destination_index] == 0;

        self.matrix[source_index][destination_index] = 1;
        self.matrix[destination_index][source_index] = 1;

        inserted
    }

    // 10. CHECK VERTEX
    fn contains_vertex(&self, vertex: i32) -> bool {
        self.find_index(vertex).is_some()
    }

    // 11. CHECK EDGE
    fn contains_edge(&self, source: i32, destination: i32) -> bool {
        match (
            self.find_index(source),
            self.find_index(destination),
        ) {
            (Some(i), Some(j)) => self.matrix[i][j] == 1,
            _ => false,
        }
    }

    // 12. REMOVE EDGE
    fn remove_edge(&mut self, source: i32, destination: i32) -> bool {
        match (
            self.find_index(source),
            self.find_index(destination),
        ) {
            (Some(i), Some(j)) => {
                if self.matrix[i][j] == 0 {
                    return false;
                }

                self.matrix[i][j] = 0;
                self.matrix[j][i] = 0;

                true
            }
            _ => false,
        }
    }

    // 13. REMOVE VERTEX
    fn remove_vertex(&mut self, vertex: i32) -> bool {
        let index = match self.find_index(vertex) {
            Some(index) => index,
            None => return false,
        };

        self.vertices.remove(index);
        self.matrix.remove(index);

        // Remove the corresponding column.
        for row in &mut self.matrix {
            row.remove(index);
        }

        true
    }

    // 14. GET DEGREE
    fn degree(&self, vertex: i32) -> Option<usize> {
        let index = self.find_index(vertex)?;

        Some(
            self.matrix[index]
                .iter()
                .map(|&value| value as usize)
                .sum()
        )
    }

    // 15. GET NEIGHBORS
    fn neighbors(&self, vertex: i32) -> Option<Vec<i32>> {
        let index = self.find_index(vertex)?;

        let mut result = Vec::new();

        for (j, &connected) in self.matrix[index].iter().enumerate() {
            if connected == 1 {
                result.push(self.vertices[j]);
            }
        }

        Some(result)
    }

    // 16. DISPLAY VERTICES
    fn display_vertices(&self) {
        println!("Vertices: {:?}", self.vertices);
    }

    // 17. DISPLAY MATRIX
    fn display_matrix(&self) {
        println!("\nAdjacency Matrix:");

        print!("     ");

        for vertex in &self.vertices {
            print!("{:4}", vertex);
        }

        println!();

        for (i, row) in self.matrix.iter().enumerate() {
            print!("{:4} ", self.vertices[i]);

            for value in row {
                print!("{:4}", value);
            }

            println!();
        }
    }

    // 18. DISPLAY GRAPH
    fn display_graph(&self) {
        println!("\nAdjacency List:");

        for vertex in &self.vertices {
            println!(
                "{} -> {:?}",
                vertex,
                self.neighbors(*vertex).unwrap()
            );
        }
    }

    // 19. DEPTH FIRST SEARCH
    fn dfs(&self, start: i32) -> Vec<i32> {
        let start_index = match self.find_index(start) {
            Some(index) => index,
            None => return Vec::new(),
        };

        let mut visited = vec![false; self.vertex_count()];
        let mut stack = vec![start_index];
        let mut result = Vec::new();

        while let Some(index) = stack.pop() {
            if visited[index] {
                continue;
            }

            visited[index] = true;
            result.push(self.vertices[index]);

            // Reverse order for ascending traversal.
            for j in (0..self.vertex_count()).rev() {
                if self.matrix[index][j] == 1 && !visited[j] {
                    stack.push(j);
                }
            }
        }

        result
    }

    // 20. BREADTH FIRST SEARCH
    fn bfs(&self, start: i32) -> Vec<i32> {
        let start_index = match self.find_index(start) {
            Some(index) => index,
            None => return Vec::new(),
        };

        let mut visited = vec![false; self.vertex_count()];
        let mut queue = std::collections::VecDeque::new();
        let mut result = Vec::new();

        visited[start_index] = true;
        queue.push_back(start_index);

        while let Some(index) = queue.pop_front() {
            result.push(self.vertices[index]);

            for j in 0..self.vertex_count() {
                if self.matrix[index][j] == 1 && !visited[j] {
                    visited[j] = true;
                    queue.push_back(j);
                }
            }
        }

        result
    }

    // 21. CHECK PATH
    fn has_path(&self, source: i32, destination: i32) -> bool {
        self.bfs(source).contains(&destination)
    }

    // 22. CHECK SYMMETRY
    fn is_symmetric(&self) -> bool {
        for i in 0..self.vertex_count() {
            for j in 0..self.vertex_count() {
                if self.matrix[i][j] != self.matrix[j][i] {
                    return false;
                }
            }
        }

        true
    }

    // 23. CHECK COMPLETE GRAPH
    fn is_complete(&self) -> bool {
        let n = self.vertex_count();

        for i in 0..n {
            for j in 0..n {
                if i != j && self.matrix[i][j] != 1 {
                    return false;
                }
            }
        }

        true
    }

    // 24. COUNT ISOLATED VERTICES
    fn isolated_vertices(&self) -> Vec<i32> {
        let mut result = Vec::new();

        for (i, row) in self.matrix.iter().enumerate() {
            if row.iter().all(|&value| value == 0) {
                result.push(self.vertices[i]);
            }
        }

        result
    }

    // 25. TRANSPOSE MATRIX
    // For an undirected graph, transpose equals original matrix.
    fn transpose(&self) -> Vec<Vec<u8>> {
        let n = self.vertex_count();
        let mut result = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                result[j][i] = self.matrix[i][j];
            }
        }

        result
    }

    // 26. CLEAR GRAPH
    fn clear(&mut self) {
        self.vertices.clear();
        self.matrix.clear();
    }

    // 27. CLONE GRAPH
    fn clone_graph(&self) -> Self {
        self.clone()
    }

    // 28. MATRIX MEMORY SIZE
    fn matrix_memory_size(&self) -> usize {
        self.matrix
            .iter()
            .map(|row| size_of_val(row.as_slice()))
            .sum()
    }
}

// ============================================================
// 29. MAIN FUNCTION
// ============================================================

fn main() {

    // 30. CREATE GRAPH
    println!("\n30. CREATE GRAPH");

    let mut graph = AdjacencyMatrix::new();

    println!("Is empty: {}", graph.is_empty());

    // 31. ADD VERTICES
    println!("\n31. ADD VERTICES");

    for vertex in 1..=6 {
        graph.add_vertex(vertex);
    }

    graph.display_vertices();

    // 32. ADD EDGES
    println!("\n32. ADD EDGES");

    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 4);
    graph.add_edge(2, 5);
    graph.add_edge(3, 5);
    graph.add_edge(4, 6);
    graph.add_edge(5, 6);

    // 33. DISPLAY MATRIX
    println!("\n33. DISPLAY MATRIX");

    graph.display_matrix();

    // 34. DISPLAY GRAPH
    println!("\n34. DISPLAY GRAPH");

    graph.display_graph();

    // 35. VERTEX COUNT
    println!("\n35. VERTEX COUNT");

    println!("Vertices: {}", graph.vertex_count());

    // 36. EDGE COUNT
    println!("\n36. EDGE COUNT");

    println!("Edges: {}", graph.edge_count());

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

    // 40. DEGREE
    println!("\n40. DEGREE");

    for vertex in 1..=6 {
        println!("Degree {}: {:?}", vertex, graph.degree(vertex));
    }

    // 41. DFS
    println!("\n41. DFS");

    println!("DFS: {:?}", graph.dfs(1));

    // 42. BFS
    println!("\n42. BFS");

    println!("BFS: {:?}", graph.bfs(1));

    // 43. PATH DETECTION
    println!("\n43. PATH DETECTION");

    println!("1 to 6: {}", graph.has_path(1, 6));
    println!("1 to 9: {}", graph.has_path(1, 9));

    // 44. SYMMETRY
    println!("\n44. SYMMETRY");

    println!("Symmetric: {}", graph.is_symmetric());

    // 45. COMPLETE GRAPH
    println!("\n45. COMPLETE GRAPH");

    println!("Complete: {}", graph.is_complete());

    // 46. ISOLATED VERTICES
    println!("\n46. ISOLATED VERTICES");

    graph.add_vertex(7);

    println!("Isolated: {:?}", graph.isolated_vertices());

    // 47. TRANSPOSE
    println!("\n47. TRANSPOSE");

    println!("Transpose: {:?}", graph.transpose());

    // 48. REMOVE EDGE
    println!("\n48. REMOVE EDGE");

    println!("Removed: {}", graph.remove_edge(1, 2));

    graph.display_matrix();

    // 49. REMOVE VERTEX
    println!("\n49. REMOVE VERTEX");

    println!("Removed: {}", graph.remove_vertex(6));

    graph.display_matrix();

    // 50. CLONE GRAPH
    println!("\n50. CLONE GRAPH");

    let cloned_graph = graph.clone_graph();

    println!("Original vertices: {}", graph.vertex_count());
    println!("Cloned vertices: {}", cloned_graph.vertex_count());

    // 51. MATRIX MEMORY
    println!("\n51. MATRIX MEMORY");

    println!(
        "Matrix row storage: {} bytes",
        graph.matrix_memory_size()
    );

    println!(
        "Graph descriptor size: {} bytes",
        size_of_val(&graph)
    );

    // 52. CLEAR GRAPH
    println!("\n52. CLEAR GRAPH");

    graph.clear();

    println!("Is empty: {}", graph.is_empty());
    println!("Vertices: {}", graph.vertex_count());
    println!("Edges: {}", graph.edge_count());

    // 53. FINAL STATUS
    println!("\n53. FINAL STATUS");

    println!("Adjacency Matrix demonstration completed.");
}



/*
//---------------------------------------------------
Adjacency Matrix vs Adjacency Lis:

Feature - Matrix - List

Storage - O(V²) - O(V + E)
Check edge - O(1) - Average O(1) with HashSet
Find neighbors - O(V) - O(degree)
Add vertex - O(V²) resizing - Average O(1)
Best suited for - Dense graphs - Sparse graphs

//---------------------------------------------------


//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261202_ds_AdjacencyMatrix_01.rs -o adjacency_matrix_example

./adjacency_matrix_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

30. CREATE GRAPH
Is empty: true

31. ADD VERTICES
Vertices: [1, 2, 3, 4, 5, 6]

32. ADD EDGES

33. DISPLAY MATRIX

Adjacency Matrix:
        1   2   3   4   5   6
   1    0   1   1   0   0   0
   2    1   0   0   1   1   0
   3    1   0   0   0   1   0
   4    0   1   0   0   0   1
   5    0   1   1   0   0   1
   6    0   0   0   1   1   0

34. DISPLAY GRAPH

Adjacency List:
1 -> [2, 3]
2 -> [1, 4, 5]
3 -> [1, 5]
4 -> [2, 6]
5 -> [2, 3, 6]
6 -> [4, 5]

35. VERTEX COUNT
Vertices: 6

36. EDGE COUNT
Edges: 7

37. CHECK VERTEX
Vertex 3: true
Vertex 9: false

38. CHECK EDGE
1 - 2: true
1 - 6: false

39. GET NEIGHBORS
Neighbors of 2: Some([1, 4, 5])
Neighbors of 5: Some([2, 3, 6])

40. DEGREE
Degree 1: Some(2)
Degree 2: Some(3)
Degree 3: Some(2)
Degree 4: Some(2)
Degree 5: Some(3)
Degree 6: Some(2)

41. DFS
DFS: [1, 2, 4, 6, 5, 3]

42. BFS
BFS: [1, 2, 3, 4, 5, 6]

43. PATH DETECTION
1 to 6: true
1 to 9: false

44. SYMMETRY
Symmetric: true

45. COMPLETE GRAPH
Complete: false

46. ISOLATED VERTICES
Isolated: [7]

47. TRANSPOSE
Transpose: [[0, 1, 1, 0, 0, 0, 0], [1, 0, 0, 1, 1, 0, 0], [1, 0, 0, 0, 1, 0, 0], [0, 1, 0, 0, 0, 1, 0], [0, 1, 1, 0, 0, 1, 0], [0, 0, 0, 1, 1, 0, 0], [0, 0, 0, 0, 0, 0, 0]]

48. REMOVE EDGE
Removed: true

Adjacency Matrix:
        1   2   3   4   5   6   7
   1    0   0   1   0   0   0   0
   2    0   0   0   1   1   0   0
   3    1   0   0   0   1   0   0
   4    0   1   0   0   0   1   0
   5    0   1   1   0   0   1   0
   6    0   0   0   1   1   0   0
   7    0   0   0   0   0   0   0

49. REMOVE VERTEX
Removed: true

Adjacency Matrix:
        1   2   3   4   5   7
   1    0   0   1   0   0   0
   2    0   0   0   1   1   0
   3    1   0   0   0   1   0
   4    0   1   0   0   0   0
   5    0   1   1   0   0   0
   7    0   0   0   0   0   0

50. CLONE GRAPH
Original vertices: 6
Cloned vertices: 6

51. MATRIX MEMORY
Matrix row storage: 36 bytes
Graph descriptor size: 48 bytes

52. CLEAR GRAPH
Is empty: true
Vertices: 0
Edges: 0

53. FINAL STATUS
Adjacency Matrix demonstration completed.

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/



