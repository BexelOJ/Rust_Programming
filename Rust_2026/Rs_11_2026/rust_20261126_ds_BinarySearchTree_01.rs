//---------------------------------------------------
// BinarySearchTree - Rust Data Structures
// Date: 20261126
//---------------------------------------------------

use std::collections::VecDeque;
use std::mem::size_of;

// ============================================================
// 01. BINARY SEARCH TREE NODE
// ============================================================

#[derive(Debug, Clone)]
struct Node {
    data: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// ============================================================
// 02. BINARY SEARCH TREE STRUCTURE
// ============================================================

#[derive(Debug, Clone)]
struct BinarySearchTree {
    root: Option<Box<Node>>,
}

// ============================================================
// 03. IMPLEMENT BINARY SEARCH TREE
// ============================================================

impl BinarySearchTree {

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

    // 07. INSERT A VALUE
    fn insert(&mut self, data: i32) {
        Self::insert_node(&mut self.root, data);
    }

    fn insert_node(node: &mut Option<Box<Node>>, data: i32) {
        match node {
            Some(current) => {
                if data < current.data {
                    Self::insert_node(&mut current.left, data);
                } else if data > current.data {
                    Self::insert_node(&mut current.right, data);
                }
                // Duplicate values are ignored.
            }
            None => {
                *node = Some(Self::create_node(data));
            }
        }
    }

    // 08. SEARCH FOR A VALUE
    fn search(&self, target: i32) -> bool {
        Self::search_node(&self.root, target)
    }

    fn search_node(node: &Option<Box<Node>>, target: i32) -> bool {
        match node {
            Some(current) => {
                if target == current.data {
                    true
                } else if target < current.data {
                    Self::search_node(&current.left, target)
                } else {
                    Self::search_node(&current.right, target)
                }
            }
            None => false,
        }
    }

    // 09. FIND MINIMUM VALUE
    fn min_value(&self) -> Option<i32> {
        Self::find_min(&self.root)
    }

    fn find_min(node: &Option<Box<Node>>) -> Option<i32> {
        let mut current = node.as_deref()?;

        while let Some(left) = current.left.as_deref() {
            current = left;
        }

        Some(current.data)
    }

    // 10. FIND MAXIMUM VALUE
    fn max_value(&self) -> Option<i32> {
        Self::find_max(&self.root)
    }

    fn find_max(node: &Option<Box<Node>>) -> Option<i32> {
        let mut current = node.as_deref()?;

        while let Some(right) = current.right.as_deref() {
            current = right;
        }

        Some(current.data)
    }

    // 11. INORDER: LEFT, ROOT, RIGHT
    // In a BST, this produces ascending sorted values.
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

    // 12. PREORDER: ROOT, LEFT, RIGHT
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

    // 13. POSTORDER: LEFT, RIGHT, ROOT
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

    // 14. LEVEL ORDER: BREADTH-FIRST
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

    // 15. COUNT TOTAL NODES
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

    // 16. COUNT LEAF NODES
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

    // 17. COUNT INTERNAL NODES
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

    // 18. CALCULATE TREE HEIGHT
    // Empty tree = 0; single node = 1.
    fn height(&self) -> usize {
        Self::node_height(&self.root)
    }

    fn node_height(node: &Option<Box<Node>>) -> usize {
        match node {
            Some(current) => {
                1 + Self::node_height(&current.left)
                    .max(Self::node_height(&current.right))
            }
            None => 0,
        }
    }

    // 19. FIND PREDECESSOR
    // Largest value smaller than target.
    fn predecessor(&self, target: i32) -> Option<i32> {
        let mut current = self.root.as_deref();
        let mut result = None;

        while let Some(node) = current {
            if target > node.data {
                result = Some(node.data);
                current = node.right.as_deref();
            } else {
                current = node.left.as_deref();
            }
        }

        result
    }

    // 20. FIND SUCCESSOR
    // Smallest value greater than target.
    fn successor(&self, target: i32) -> Option<i32> {
        let mut current = self.root.as_deref();
        let mut result = None;

        while let Some(node) = current {
            if target < node.data {
                result = Some(node.data);
                current = node.left.as_deref();
            } else {
                current = node.right.as_deref();
            }
        }

        result
    }

    // 21. DELETE A VALUE
    fn delete(&mut self, target: i32) -> bool {
        let (new_root, deleted) = Self::delete_node(self.root.take(), target);
        self.root = new_root;
        deleted
    }

    fn delete_node(
        node: Option<Box<Node>>,
        target: i32,
    ) -> (Option<Box<Node>>, bool) {

        match node {
            None => (None, false),

            Some(mut current) => {
                if target < current.data {
                    let (new_left, deleted) =
                        Self::delete_node(current.left.take(), target);

                    current.left = new_left;

                    (Some(current), deleted)

                } else if target > current.data {
                    let (new_right, deleted) =
                        Self::delete_node(current.right.take(), target);

                    current.right = new_right;

                    (Some(current), deleted)

                } else {
                    // CASE 1: NO CHILDREN
                    if current.left.is_none() && current.right.is_none() {
                        (None, true)

                    // CASE 2: ONLY RIGHT CHILD
                    } else if current.left.is_none() {
                        (current.right.take(), true)

                    // CASE 2: ONLY LEFT CHILD
                    } else if current.right.is_none() {
                        (current.left.take(), true)

                    // CASE 3: TWO CHILDREN
                    } else {
                        let successor =
                            Self::find_min(&current.right).unwrap();

                        current.data = successor;

                        let (new_right, _) =
                            Self::delete_node(
                                current.right.take(),
                                successor,
                            );

                        current.right = new_right;

                        (Some(current), true)
                    }
                }
            }
        }
    }

    // 22. CHECK WHETHER TREE IS FULL
    fn is_full(&self) -> bool {
        Self::check_full(&self.root)
    }

    fn check_full(node: &Option<Box<Node>>) -> bool {
        match node {
            Some(current) => {
                let left = current.left.is_some();
                let right = current.right.is_some();

                if left != right {
                    return false;
                }

                Self::check_full(&current.left)
                    && Self::check_full(&current.right)
            }
            None => true,
        }
    }

    // 23. CHECK WHETHER TREE IS COMPLETE
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

    // 24. CHECK WHETHER TREE IS PERFECT
    fn is_perfect(&self) -> bool {
        let height = self.height();
        let count = self.count_nodes();

        count == (1usize << height) - 1
    }

    // 25. DISPLAY TREE STRUCTURE
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

    // 26. FIND A VALUE'S LEVEL
    // Root is at level 1.
    fn find_level(&self, target: i32) -> Option<usize> {
        let mut current = self.root.as_deref();
        let mut level = 1;

        while let Some(node) = current {
            if target == node.data {
                return Some(level);
            }

            if target < node.data {
                current = node.left.as_deref();
            } else {
                current = node.right.as_deref();
            }

            level += 1;
        }

        None
    }

    // 27. CLONE TREE
    fn clone_tree(&self) -> Self {
        self.clone()
    }

    // 28. CLEAR TREE
    fn clear(&mut self) {
        self.root = None;
    }

    // 29. TREE MEMORY SIZE
    fn node_memory_size() -> usize {
        size_of::<Node>()
    }

    // 30. VALIDATE BST PROPERTY
    fn is_valid_bst(&self) -> bool {
        Self::validate_node(&self.root, None, None)
    }

    fn validate_node(
        node: &Option<Box<Node>>,
        min: Option<i32>,
        max: Option<i32>,
    ) -> bool {
        match node {
            Some(current) => {
                if let Some(min_value) = min {
                    if current.data <= min_value {
                        return false;
                    }
                }

                if let Some(max_value) = max {
                    if current.data >= max_value {
                        return false;
                    }
                }

                Self::validate_node(
                    &current.left,
                    min,
                    Some(current.data),
                ) && Self::validate_node(
                    &current.right,
                    Some(current.data),
                    max,
                )
            }
            None => true,
        }
    }
}

// ============================================================
// 31. MAIN FUNCTION
// ============================================================

fn main() {

    println!("======================================");
    println!("  RUST BINARY SEARCH TREE EXAMPLE");
    println!("======================================");

    // 32. CREATE BST
    println!("\n32. CREATE BST");

    let mut tree = BinarySearchTree::new();

    println!("Is empty: {}", tree.is_empty());

    // 33. INSERT VALUES
    println!("\n33. INSERT VALUES");

    let values = [50, 30, 70, 20, 40, 60, 80];

    for value in values {
        tree.insert(value);
    }

    println!("Inserted: {:?}", values);

    // 34. INSERT DUPLICATE
    println!("\n34. INSERT DUPLICATE");

    tree.insert(50);

    println!("Node count: {}", tree.count_nodes());

    // 35. DISPLAY TREE
    println!("\n35. TREE STRUCTURE");

    tree.display();

    // 36. INORDER
    println!("\n36. INORDER TRAVERSAL");

    tree.inorder();

    // 37. PREORDER
    println!("\n37. PREORDER TRAVERSAL");

    tree.preorder();

    // 38. POSTORDER
    println!("\n38. POSTORDER TRAVERSAL");

    tree.postorder();

    // 39. LEVEL ORDER
    println!("\n39. LEVEL ORDER TRAVERSAL");

    tree.level_order();

    // 40. SEARCH
    println!("\n40. SEARCH");

    println!("Search 40: {}", tree.search(40));
    println!("Search 90: {}", tree.search(90));

    // 41. MINIMUM AND MAXIMUM
    println!("\n41. MINIMUM AND MAXIMUM");

    println!("Minimum: {:?}", tree.min_value());
    println!("Maximum: {:?}", tree.max_value());

    // 42. PREDECESSOR AND SUCCESSOR
    println!("\n42. PREDECESSOR AND SUCCESSOR");

    println!("Predecessor of 50: {:?}", tree.predecessor(50));
    println!("Successor of 50: {:?}", tree.successor(50));

    // 43. NODE COUNTS
    println!("\n43. NODE COUNTS");

    println!("Total nodes: {}", tree.count_nodes());
    println!("Leaf nodes: {}", tree.count_leaves());
    println!("Internal nodes: {}", tree.count_internal_nodes());

    // 44. HEIGHT
    println!("\n44. TREE HEIGHT");

    println!("Height: {}", tree.height());

    // 45. TREE PROPERTIES
    println!("\n45. TREE PROPERTIES");

    println!("Is full: {}", tree.is_full());
    println!("Is complete: {}", tree.is_complete());
    println!("Is perfect: {}", tree.is_perfect());
    println!("Is valid BST: {}", tree.is_valid_bst());

    // 46. FIND LEVEL
    println!("\n46. FIND LEVEL");

    println!("Level of 20: {:?}", tree.find_level(20));
    println!("Level of 80: {:?}", tree.find_level(80));
    println!("Level of 99: {:?}", tree.find_level(99));

    // 47. CLONE TREE
    println!("\n47. CLONE TREE");

    let cloned_tree = tree.clone_tree();

    println!("Cloned BST inorder:");
    cloned_tree.inorder();

    // 48. DELETE LEAF NODE
    println!("\n48. DELETE LEAF NODE");

    println!("Deleted 20: {}", tree.delete(20));
    tree.inorder();

    // 49. DELETE NODE WITH ONE CHILD
    println!("\n49. DELETE NODE WITH ONE CHILD");

    // Insert a value to demonstrate the one-child case.
    tree.insert(10);

    println!("Before deleting 30:");
    tree.display();

    println!("Deleted 30: {}", tree.delete(30));
    tree.display();

    // 50. DELETE NODE WITH TWO CHILDREN
    println!("\n50. DELETE NODE WITH TWO CHILDREN");

    println!("Deleted 50: {}", tree.delete(50));
    tree.display();

    // 51. VALIDATE AFTER DELETION
    println!("\n51. VALIDATE AFTER DELETION");

    println!("Is valid BST: {}", tree.is_valid_bst());
    println!("Inorder:");
    tree.inorder();

    // 52. NODE MEMORY SIZE
    println!("\n52. NODE MEMORY SIZE");

    println!(
        "Node structure size: {} bytes",
        BinarySearchTree::node_memory_size()
    );

    // 53. CLEAR TREE
    println!("\n53. CLEAR TREE");

    tree.clear();

    println!("Is empty: {}", tree.is_empty());
    println!("Node count: {}", tree.count_nodes());

    // 54. FINAL STATUS
    println!("\n54. FINAL STATUS");

    println!("Binary Search Tree demonstration completed.");
}

/*
//---------------------------------------------------
A Binary Search Tree (BST) is a binary tree with an ordering rule:
- Values smaller than a node go to its left subtree.
- Values greater than a node go to its right subtree.
- Duplicate values will be ignored in this implementation.

//---------------------------------------------------
Time complexity:

Let h be the height of the tree and n the number of nodes.

Operation	Average (balanced)	Worst case

Insert - O(log n) - O(n)
Search - O(log n) - O(n)
Delete - O(log n) - O(n)
Find min/max - O(log n) - O(n)
Traversal - O(n) - O(n)

//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261126_ds_BinarySearchTree_01.rs -o bst_example

./bst_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

======================================
  RUST BINARY SEARCH TREE EXAMPLE
======================================

32. CREATE BST
Is empty: true

33. INSERT VALUES
Inserted: [50, 30, 70, 20, 40, 60, 80]

34. INSERT DUPLICATE
Node count: 7

35. TREE STRUCTURE
        80
    70
        60
50
        40
    30
        20

36. INORDER TRAVERSAL
20 30 40 50 60 70 80

37. PREORDER TRAVERSAL
50 30 20 40 70 60 80

38. POSTORDER TRAVERSAL
20 40 30 60 80 70 50

39. LEVEL ORDER TRAVERSAL
50 30 70 20 40 60 80

40. SEARCH
Search 40: true
Search 90: false

41. MINIMUM AND MAXIMUM
Minimum: Some(20)
Maximum: Some(80)

42. PREDECESSOR AND SUCCESSOR
Predecessor of 50: Some(40)
Successor of 50: Some(60)

43. NODE COUNTS
Total nodes: 7
Leaf nodes: 4
Internal nodes: 3

44. TREE HEIGHT
Height: 3

45. TREE PROPERTIES
Is full: true
Is complete: true
Is perfect: true
Is valid BST: true

46. FIND LEVEL
Level of 20: Some(3)
Level of 80: Some(3)
Level of 99: None

47. CLONE TREE
Cloned BST inorder:
20 30 40 50 60 70 80

48. DELETE LEAF NODE
Deleted 20: true
30 40 50 60 70 80

49. DELETE NODE WITH ONE CHILD
Before deleting 30:
        80
    70
        60
50
        40
    30
        10
Deleted 30: true
        80
    70
        60
50
    40
        10

50. DELETE NODE WITH TWO CHILDREN
Deleted 50: true
        80
    70
60
    40
        10

51. VALIDATE AFTER DELETION
Is valid BST: true
Inorder:
10 40 60 70 80

52. NODE MEMORY SIZE
Node structure size: 24 bytes

53. CLEAR TREE
Is empty: true
Node count: 0

54. FINAL STATUS
Binary Search Tree demonstration completed.

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/



