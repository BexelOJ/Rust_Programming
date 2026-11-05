//---------------------------------------------------
// BinaryTree - Rust Data Structures
// Date: 20261125
//---------------------------------------------------

use std::collections::VecDeque;
use std::mem::size_of;

// ============================================================
// 01. BINARY TREE NODE
// ============================================================

#[derive(Debug, Clone)]
struct Node {
    data: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// ============================================================
// 02. BINARY TREE STRUCTURE
// ============================================================

#[derive(Debug, Clone)]
struct BinaryTree {
    root: Option<Box<Node>>,
}

// ============================================================
// 03. IMPLEMENT BINARY TREE
// ============================================================

impl BinaryTree {

    // 04. CREATE AN EMPTY TREE
    fn new() -> Self {
        Self { root: None }
    }

    // 05. CHECK WHETHER TREE IS EMPTY
    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    // 06. CREATE A NEW NODE
    fn create_node(data: i32) -> Box<Node> {
        Box::new(Node {
            data,
            left: None,
            right: None,
        })
    }

    // 07. INSERT ROOT NODE
    fn insert_root(&mut self, data: i32) {
        self.root = Some(Self::create_node(data));
    }

    // 08. INSERT LEFT CHILD
    fn insert_left(&mut self, parent_data: i32, data: i32) -> bool {
        if let Some(parent) = Self::find_node_mut(&mut self.root, parent_data) {
            if parent.left.is_none() {
                parent.left = Some(Self::create_node(data));
                return true;
            }
        }

        false
    }

    // 09. INSERT RIGHT CHILD
    fn insert_right(&mut self, parent_data: i32, data: i32) -> bool {
        if let Some(parent) = Self::find_node_mut(&mut self.root, parent_data) {
            if parent.right.is_none() {
                parent.right = Some(Self::create_node(data));
                return true;
            }
        }

        false
    }

    // 10. FIND NODE MUTABLY
    fn find_node_mut(
        node: &mut Option<Box<Node>>,
        target: i32,
    ) -> Option<&mut Node> {

        if let Some(current) = node.as_deref_mut() {

            if current.data == target {
                return Some(current);
            }

            if let Some(found) =
                Self::find_node_mut(&mut current.left, target)
            {
                return Some(found);
            }

            if let Some(found) =
                Self::find_node_mut(&mut current.right, target)
            {
                return Some(found);
            }
        }

        None
    }

    // 11. SEARCH FOR A VALUE
    fn search(&self, target: i32) -> bool {
        Self::search_node(&self.root, target)
    }

    fn search_node(node: &Option<Box<Node>>, target: i32) -> bool {
        match node {
            Some(current) => {
                if current.data == target {
                    true
                } else {
                    Self::search_node(&current.left, target)
                        || Self::search_node(&current.right, target)
                }
            }
            None => false,
        }
    }

    // 12. INORDER TRAVERSAL: LEFT, ROOT, RIGHT
    fn inorder(&self) {
        Self::inorder_node(&self.root);
        println!();
    }

    fn inorder_node(node: &Option<Box<Node>>) {
        if let Some(current) = node {
            Self::inorder_node(&current.left);
            print!("{} ", current.data);
            Self::inorder_node(&current.right);
        }
    }

    // 13. PREORDER TRAVERSAL: ROOT, LEFT, RIGHT
    fn preorder(&self) {
        Self::preorder_node(&self.root);
        println!();
    }

    fn preorder_node(node: &Option<Box<Node>>) {
        if let Some(current) = node {
            print!("{} ", current.data);
            Self::preorder_node(&current.left);
            Self::preorder_node(&current.right);
        }
    }

    // 14. POSTORDER TRAVERSAL: LEFT, RIGHT, ROOT
    fn postorder(&self) {
        Self::postorder_node(&self.root);
        println!();
    }

    fn postorder_node(node: &Option<Box<Node>>) {
        if let Some(current) = node {
            Self::postorder_node(&current.left);
            Self::postorder_node(&current.right);
            print!("{} ", current.data);
        }
    }

    // 15. LEVEL ORDER TRAVERSAL: BREADTH-FIRST
    fn level_order(&self) {
        let mut queue: VecDeque<&Node> = VecDeque::new();

        if let Some(root) = self.root.as_deref() {
            queue.push_back(root);
        }

        while let Some(current) = queue.pop_front() {
            print!("{} ", current.data);

            if let Some(left) = current.left.as_deref() {
                queue.push_back(left);
            }

            if let Some(right) = current.right.as_deref() {
                queue.push_back(right);
            }
        }

        println!();
    }

    // 16. COUNT TOTAL NODES
    fn count_nodes(&self) -> usize {
        Self::count_node(&self.root)
    }

    fn count_node(node: &Option<Box<Node>>) -> usize {
        match node {
            Some(current) => {
                1 + Self::count_node(&current.left)
                    + Self::count_node(&current.right)
            }
            None => 0,
        }
    }

    // 17. COUNT LEAF NODES
    fn count_leaves(&self) -> usize {
        Self::count_leaf_nodes(&self.root)
    }

    fn count_leaf_nodes(node: &Option<Box<Node>>) -> usize {
        match node {
            Some(current) => {
                if current.left.is_none() && current.right.is_none() {
                    1
                } else {
                    Self::count_leaf_nodes(&current.left)
                        + Self::count_leaf_nodes(&current.right)
                }
            }
            None => 0,
        }
    }

    // 18. COUNT INTERNAL NODES
    fn count_internal_nodes(&self) -> usize {
        Self::count_internal(&self.root)
    }

    fn count_internal(node: &Option<Box<Node>>) -> usize {
        match node {
            Some(current) => {
                let is_internal =
                    current.left.is_some() || current.right.is_some();

                usize::from(is_internal)
                    + Self::count_internal(&current.left)
                    + Self::count_internal(&current.right)
            }
            None => 0,
        }
    }

    // 19. CALCULATE TREE HEIGHT
    // Empty tree = 0, single-node tree = 1
    fn height(&self) -> usize {
        Self::node_height(&self.root)
    }

    fn node_height(node: &Option<Box<Node>>) -> usize {
        match node {
            Some(current) => {
                let left_height = Self::node_height(&current.left);
                let right_height = Self::node_height(&current.right);

                1 + left_height.max(right_height)
            }
            None => 0,
        }
    }

    // 20. FIND MINIMUM VALUE
    fn min_value(&self) -> Option<i32> {
        Self::find_min(&self.root)
    }

    fn find_min(node: &Option<Box<Node>>) -> Option<i32> {
        match node {
            Some(current) => {
                let mut min = current.data;

                if let Some(left_min) = Self::find_min(&current.left) {
                    min = min.min(left_min);
                }

                if let Some(right_min) = Self::find_min(&current.right) {
                    min = min.min(right_min);
                }

                Some(min)
            }
            None => None,
        }
    }

    // 21. FIND MAXIMUM VALUE
    fn max_value(&self) -> Option<i32> {
        Self::find_max(&self.root)
    }

    fn find_max(node: &Option<Box<Node>>) -> Option<i32> {
        match node {
            Some(current) => {
                let mut max = current.data;

                if let Some(left_max) = Self::find_max(&current.left) {
                    max = max.max(left_max);
                }

                if let Some(right_max) = Self::find_max(&current.right) {
                    max = max.max(right_max);
                }

                Some(max)
            }
            None => None,
        }
    }

    // 22. CALCULATE SUM OF ALL NODES
    fn sum(&self) -> i32 {
        Self::node_sum(&self.root)
    }

    fn node_sum(node: &Option<Box<Node>>) -> i32 {
        match node {
            Some(current) => {
                current.data
                    + Self::node_sum(&current.left)
                    + Self::node_sum(&current.right)
            }
            None => 0,
        }
    }

    // 23. CHECK WHETHER TREE IS FULL
    // Every node has either zero or two children.
    fn is_full(&self) -> bool {
        Self::check_full(&self.root)
    }

    fn check_full(node: &Option<Box<Node>>) -> bool {
        match node {
            Some(current) => {
                let left_exists = current.left.is_some();
                let right_exists = current.right.is_some();

                if left_exists != right_exists {
                    return false;
                }

                Self::check_full(&current.left)
                    && Self::check_full(&current.right)
            }
            None => true,
        }
    }

    // 24. CHECK WHETHER TREE IS COMPLETE
    fn is_complete(&self) -> bool {
        let mut queue: VecDeque<Option<&Node>> = VecDeque::new();
        queue.push_back(self.root.as_deref());

        let mut found_empty = false;

        while let Some(node) = queue.pop_front() {
            match node {
                Some(current) => {
                    if found_empty {
                        return false;
                    }

                    queue.push_back(current.left.as_deref());
                    queue.push_back(current.right.as_deref());
                }
                None => {
                    found_empty = true;
                }
            }
        }

        true
    }

    // 25. CHECK WHETHER TREE IS PERFECT
    fn is_perfect(&self) -> bool {
        let height = self.height();
        let count = self.count_nodes();

        count == (1usize << height) - 1
    }

    // 26. MIRROR THE TREE
    fn mirror(&mut self) {
        Self::mirror_node(&mut self.root);
    }

    fn mirror_node(node: &mut Option<Box<Node>>) {
        if let Some(current) = node {
            std::mem::swap(&mut current.left, &mut current.right);

            Self::mirror_node(&mut current.left);
            Self::mirror_node(&mut current.right);
        }
    }

    // 27. FIND A NODE AT A GIVEN LEVEL
    // Root is at level 1.
    fn print_level(&self, target_level: usize) {
        Self::print_level_node(&self.root, target_level, 1);
        println!();
    }

    fn print_level_node(
        node: &Option<Box<Node>>,
        target_level: usize,
        current_level: usize,
    ) {
        if let Some(current) = node {
            if current_level == target_level {
                print!("{} ", current.data);
                return;
            }

            Self::print_level_node(
                &current.left,
                target_level,
                current_level + 1,
            );

            Self::print_level_node(
                &current.right,
                target_level,
                current_level + 1,
            );
        }
    }

    // 28. DISPLAY TREE STRUCTURE
    fn display(&self) {
        Self::display_node(&self.root, 0);
    }

    fn display_node(node: &Option<Box<Node>>, depth: usize) {
        if let Some(current) = node {
            Self::display_node(&current.right, depth + 1);

            println!(
                "{}{}",
                "    ".repeat(depth),
                current.data
            );

            Self::display_node(&current.left, depth + 1);
        }
    }

    // 29. CLONE TREE
    fn clone_tree(&self) -> Self {
        self.clone()
    }

    // 30. CLEAR TREE
    fn clear(&mut self) {
        self.root = None;
    }

    // 31. GET ROOT VALUE
    fn root_value(&self) -> Option<i32> {
        self.root.as_ref().map(|node| node.data)
    }

    // 32. TREE MEMORY SIZE
    fn node_memory_size() -> usize {
        size_of::<Node>()
    }
}

// ============================================================
// 33. MAIN FUNCTION
// ============================================================

fn main() {

    println!("======================================");
    println!("       RUST BINARY TREE EXAMPLE");
    println!("======================================");

    // 34. CREATE EMPTY TREE
    println!("\n34. CREATE EMPTY TREE");

    let mut tree = BinaryTree::new();

    println!("Tree is empty: {}", tree.is_empty());

    // 35. INSERT ROOT
    println!("\n35. INSERT ROOT");

    tree.insert_root(10);

    println!("Root value: {:?}", tree.root_value());

    // 36. INSERT CHILD NODES
    println!("\n36. INSERT CHILD NODES");

    tree.insert_left(10, 20);
    tree.insert_right(10, 30);

    tree.insert_left(20, 40);
    tree.insert_right(20, 50);

    tree.insert_left(30, 60);
    tree.insert_right(30, 70);

    println!("Binary tree created.");

    // 37. DISPLAY TREE
    println!("\n37. TREE STRUCTURE");

    tree.display();

    // 38. INORDER TRAVERSAL
    println!("\n38. INORDER TRAVERSAL");

    tree.inorder();

    // 39. PREORDER TRAVERSAL
    println!("\n39. PREORDER TRAVERSAL");

    tree.preorder();

    // 40. POSTORDER TRAVERSAL
    println!("\n40. POSTORDER TRAVERSAL");

    tree.postorder();

    // 41. LEVEL ORDER TRAVERSAL
    println!("\n41. LEVEL ORDER TRAVERSAL");

    tree.level_order();

    // 42. SEARCH
    println!("\n42. SEARCH");

    println!("Search 50: {}", tree.search(50));
    println!("Search 99: {}", tree.search(99));

    // 43. NODE COUNTS
    println!("\n43. NODE COUNTS");

    println!("Total nodes: {}", tree.count_nodes());
    println!("Leaf nodes: {}", tree.count_leaves());
    println!("Internal nodes: {}", tree.count_internal_nodes());

    // 44. HEIGHT
    println!("\n44. TREE HEIGHT");

    println!("Height: {}", tree.height());

    // 45. MINIMUM AND MAXIMUM
    println!("\n45. MINIMUM AND MAXIMUM");

    println!("Minimum: {:?}", tree.min_value());
    println!("Maximum: {:?}", tree.max_value());

    // 46. SUM
    println!("\n46. SUM OF NODES");

    println!("Sum: {}", tree.sum());

    // 47. TREE PROPERTIES
    println!("\n47. TREE PROPERTIES");

    println!("Is full: {}", tree.is_full());
    println!("Is complete: {}", tree.is_complete());
    println!("Is perfect: {}", tree.is_perfect());

    // 48. PRINT LEVEL
    println!("\n48. PRINT LEVEL 2");

    tree.print_level(2);

    // 49. CLONE TREE
    println!("\n49. CLONE TREE");

    let cloned_tree = tree.clone_tree();

    println!("Cloned tree inorder:");
    cloned_tree.inorder();

    // 50. MIRROR TREE
    println!("\n50. MIRROR TREE");

    tree.mirror();

    println!("Mirrored tree:");
    tree.display();

    println!("Mirrored inorder:");
    tree.inorder();

    // 51. CLEAR TREE
    println!("\n51. CLEAR TREE");

    tree.clear();

    println!("Tree is empty: {}", tree.is_empty());
    println!("Node count: {}", tree.count_nodes());

    // 52. NODE MEMORY SIZE
    println!("\n52. NODE MEMORY SIZE");

    println!("Node structure size: {} bytes", BinaryTree::node_memory_size());

    // 53. FINAL STATUS
    println!("\n53. FINAL STATUS");

    println!("Binary Tree demonstration completed.");

}


/*
//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261125_ds_BinaryTree_01.rs -o binarytree_example

./binarytree_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

======================================
       RUST BINARY TREE EXAMPLE
======================================

34. CREATE EMPTY TREE
Tree is empty: true

35. INSERT ROOT
Root value: Some(10)

36. INSERT CHILD NODES
Binary tree created.

37. TREE STRUCTURE
        70
    30
        60
10
        50
    20
        40

38. INORDER TRAVERSAL
40 20 50 10 60 30 70

39. PREORDER TRAVERSAL
10 20 40 50 30 60 70

40. POSTORDER TRAVERSAL
40 50 20 60 70 30 10

41. LEVEL ORDER TRAVERSAL
10 20 30 40 50 60 70

42. SEARCH
Search 50: true
Search 99: false

43. NODE COUNTS
Total nodes: 7
Leaf nodes: 4
Internal nodes: 3

44. TREE HEIGHT
Height: 3

45. MINIMUM AND MAXIMUM
Minimum: Some(10)
Maximum: Some(70)

46. SUM OF NODES
Sum: 280

47. TREE PROPERTIES
Is full: true
Is complete: true
Is perfect: true

48. PRINT LEVEL 2
20 30

49. CLONE TREE
Cloned tree inorder:
40 20 50 10 60 30 70

50. MIRROR TREE
Mirrored tree:
        40
    20
        50
10
        60
    30
        70
Mirrored inorder:
70 30 60 10 50 20 40

51. CLEAR TREE
Tree is empty: true
Node count: 0

52. NODE MEMORY SIZE
Node structure size: 24 bytes

53. FINAL STATUS
Binary Tree demonstration completed.

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/



