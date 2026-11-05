//---------------------------------------------------
// AVLTree - Rust Data Structures
// Date: 20261127
//---------------------------------------------------

use std::mem::size_of;

// ============================================================
// 01. AVL TREE NODE
// ============================================================

#[derive(Debug, Clone)]
struct Node {
    data: i32,
    height: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// ============================================================
// 02. AVL TREE STRUCTURE
// ============================================================

#[derive(Debug, Clone)]
struct AVLTree {
    root: Option<Box<Node>>,
}

// ============================================================
// 03. IMPLEMENT AVL TREE
// ============================================================

impl AVLTree {

    // 04. CREATE EMPTY TREE
    fn new() -> Self {
        Self { root: None }
    }

    // 05. CREATE A NEW NODE
    fn create_node(data: i32) -> Box<Node> {
        Box::new(Node {
            data,
            height: 1,
            left: None,
            right: None,
        })
    }

    // 06. GET NODE HEIGHT
    fn node_height(node: &Option<Box<Node>>) -> i32 {
        match node {
            Some(current) => current.height,
            None => 0,
        }
    }

    // 07. UPDATE NODE HEIGHT
    fn update_height(node: &mut Box<Node>) {
        let left_height = Self::node_height(&node.left);
        let right_height = Self::node_height(&node.right);

        node.height = 1 + left_height.max(right_height);
    }

    // 08. CALCULATE BALANCE FACTOR
    fn balance_factor(node: &Option<Box<Node>>) -> i32 {
        match node {
            Some(current) => {
                Self::node_height(&current.left)
                    - Self::node_height(&current.right)
            }
            None => 0,
        }
    }

    // 09. RIGHT ROTATION
    //
    //       y                  x
    //      / \                / \
    //     x   T3    ==>      T1  y
    //    / \                    / \
    //   T1 T2                  T2 T3
    //
    fn rotate_right(mut y: Box<Node>) -> Box<Node> {
        let mut x = y.left.take().expect("Left child must exist");
        let t2 = x.right.take();

        y.left = t2;
        Self::update_height(&mut y);

        x.right = Some(y);
        Self::update_height(&mut x);

        x
    }

    // 10. LEFT ROTATION
    //
    //       x                    y
    //      / \                  / \
    //     T1  y      ==>       x  T3
    //        / \              / \
    //       T2 T3            T1 T2
    //
    fn rotate_left(mut x: Box<Node>) -> Box<Node> {
        let mut y = x.right.take().expect("Right child must exist");
        let t2 = y.left.take();

        x.right = t2;
        Self::update_height(&mut x);

        y.left = Some(x);
        Self::update_height(&mut y);

        y
    }

    // 11. BALANCE A NODE
    fn balance_node(mut node: Box<Node>) -> Box<Node> {
        Self::update_height(&mut node);

        let balance = Self::node_height(&node.left)
            - Self::node_height(&node.right);

        // LEFT HEAVY
        if balance > 1 {
            let left_balance = Self::balance_factor(&node.left);

            // LEFT-RIGHT CASE
            if left_balance < 0 {
                let left = node.left.take().unwrap();
                node.left = Some(Self::rotate_left(left));
            }

            // LEFT-LEFT CASE
            return Self::rotate_right(node);
        }

        // RIGHT HEAVY
        if balance < -1 {
            let right_balance = Self::balance_factor(&node.right);

            // RIGHT-LEFT CASE
            if right_balance > 0 {
                let right = node.right.take().unwrap();
                node.right = Some(Self::rotate_right(right));
            }

            // RIGHT-RIGHT CASE
            return Self::rotate_left(node);
        }

        node
    }

    // 12. INSERT A VALUE
    fn insert(&mut self, data: i32) {
        self.root = Self::insert_node(self.root.take(), data);
    }

    fn insert_node(
        node: Option<Box<Node>>,
        data: i32,
    ) -> Option<Box<Node>> {

        match node {
            None => Some(Self::create_node(data)),

            Some(mut current) => {
                if data < current.data {
                    current.left =
                        Self::insert_node(current.left.take(), data);
                } else if data > current.data {
                    current.right =
                        Self::insert_node(current.right.take(), data);
                } else {
                    // Ignore duplicate values.
                    return Some(current);
                }

                Some(Self::balance_node(current))
            }
        }
    }

    // 13. SEARCH
    fn search(&self, target: i32) -> bool {
        let mut current = self.root.as_deref();

        while let Some(node) = current {
            if target == node.data {
                return true;
            }

            if target < node.data {
                current = node.left.as_deref();
            } else {
                current = node.right.as_deref();
            }
        }

        false
    }

    // 14. FIND MINIMUM
    fn min_value(&self) -> Option<i32> {
        let mut current = self.root.as_deref()?;

        while let Some(left) = current.left.as_deref() {
            current = left;
        }

        Some(current.data)
    }

    // 15. FIND MAXIMUM
    fn max_value(&self) -> Option<i32> {
        let mut current = self.root.as_deref()?;

        while let Some(right) = current.right.as_deref() {
            current = right;
        }

        Some(current.data)
    }

    // 16. DELETE A VALUE
    fn delete(&mut self, data: i32) {
        self.root = Self::delete_node(self.root.take(), data);
    }

    fn delete_node(
        node: Option<Box<Node>>,
        data: i32,
    ) -> Option<Box<Node>> {

        let mut current = node?;

        if data < current.data {
            current.left =
                Self::delete_node(current.left.take(), data);

        } else if data > current.data {
            current.right =
                Self::delete_node(current.right.take(), data);

        } else {
            // CASE 1: NO LEFT CHILD
            if current.left.is_none() {
                return current.right.take();
            }

            // CASE 2: NO RIGHT CHILD
            if current.right.is_none() {
                return current.left.take();
            }

            // CASE 3: TWO CHILDREN
            // Find inorder successor from right subtree.
            let successor =
                Self::minimum_node(&current.right).unwrap();

            current.data = successor;

            current.right =
                Self::delete_node(current.right.take(), successor);
        }

        Some(Self::balance_node(current))
    }

    // 17. FIND MINIMUM NODE IN A SUBTREE
    fn minimum_node(node: &Option<Box<Node>>) -> Option<i32> {
        let mut current = node.as_deref()?;

        while let Some(left) = current.left.as_deref() {
            current = left;
        }

        Some(current.data)
    }

    // 18. INORDER TRAVERSAL
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

    // 19. PREORDER TRAVERSAL
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

    // 20. POSTORDER TRAVERSAL
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

    // 21. LEVEL ORDER TRAVERSAL
    fn level_order(&self) {
        let mut queue: std::collections::VecDeque<&Node> =
            std::collections::VecDeque::new();

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

    // 22. COUNT NODES
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

    // 23. COUNT LEAF NODES
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

    // 24. GET TREE HEIGHT
    fn height(&self) -> i32 {
        Self::node_height(&self.root)
    }

    // 25. GET ROOT VALUE
    fn root_value(&self) -> Option<i32> {
        self.root.as_ref().map(|node| node.data)
    }

    // 26. VALIDATE AVL BALANCE
    fn is_balanced(&self) -> bool {
        Self::check_balance(&self.root).is_some()
    }

    fn check_balance(node: &Option<Box<Node>>) -> Option<i32> {
        match node {
            None => Some(0),

            Some(current) => {
                let left_height =
                    Self::check_balance(&current.left)?;

                let right_height =
                    Self::check_balance(&current.right)?;

                if (left_height - right_height).abs() > 1 {
                    return None;
                }

                Some(1 + left_height.max(right_height))
            }
        }
    }

    // 27. VALIDATE BST ORDERING
    fn is_valid_bst(&self) -> bool {
        Self::validate_bst(&self.root, None, None)
    }

    fn validate_bst(
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

                Self::validate_bst(
                    &current.left,
                    min,
                    Some(current.data),
                ) && Self::validate_bst(
                    &current.right,
                    Some(current.data),
                    max,
                )
            }
            None => true,
        }
    }

    // 28. DISPLAY TREE
    fn display(&self) {
        Self::display_node(&self.root, 0);
    }

    fn display_node(node: &Option<Box<Node>>, depth: usize) {
        if let Some(current) = node {
            Self::display_node(&current.right, depth + 1);

            println!(
                "{}{} (h={})",
                "    ".repeat(depth),
                current.data,
                current.height
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

    // 31. NODE MEMORY SIZE
    fn node_memory_size() -> usize {
        size_of::<Node>()
    }
}

// ============================================================
// 32. MAIN FUNCTION
// ============================================================

fn main() {

    println!("======================================");
    println!("       RUST AVL TREE EXAMPLE");
    println!("======================================");

    // 33. CREATE EMPTY AVL TREE
    println!("\n33. CREATE EMPTY AVL TREE");

    let tree = AVLTree::new();

    println!("Is empty: {}", tree.root_value().is_none());

    // 34. LL ROTATION
    println!("\n34. LL ROTATION");

    let mut ll_tree = AVLTree::new();

    for value in [30, 20, 10] {
        ll_tree.insert(value);
    }

    println!("Inserted: 30, 20, 10");
    println!("Root after balancing: {:?}", ll_tree.root_value());
    ll_tree.display();

    // 35. RR ROTATION
    println!("\n35. RR ROTATION");

    let mut rr_tree = AVLTree::new();

    for value in [10, 20, 30] {
        rr_tree.insert(value);
    }

    println!("Inserted: 10, 20, 30");
    println!("Root after balancing: {:?}", rr_tree.root_value());
    rr_tree.display();

    // 36. LR ROTATION
    println!("\n36. LR ROTATION");

    let mut lr_tree = AVLTree::new();

    for value in [30, 10, 20] {
        lr_tree.insert(value);
    }

    println!("Inserted: 30, 10, 20");
    println!("Root after balancing: {:?}", lr_tree.root_value());
    lr_tree.display();

    // 37. RL ROTATION
    println!("\n37. RL ROTATION");

    let mut rl_tree = AVLTree::new();

    for value in [10, 30, 20] {
        rl_tree.insert(value);
    }

    println!("Inserted: 10, 30, 20");
    println!("Root after balancing: {:?}", rl_tree.root_value());
    rl_tree.display();

    // 38. INSERT MULTIPLE VALUES
    println!("\n38. INSERT MULTIPLE VALUES");

    let mut tree = AVLTree::new();

    let values = [50, 30, 70, 20, 40, 60, 80, 10, 25, 35, 45];

    for value in values {
        tree.insert(value);
    }

    println!("Inserted: {:?}", values);

    // 39. DISPLAY AVL TREE
    println!("\n39. AVL TREE STRUCTURE");

    tree.display();

    // 40. INORDER TRAVERSAL
    println!("\n40. INORDER TRAVERSAL");

    tree.inorder();

    // 41. PREORDER TRAVERSAL
    println!("\n41. PREORDER TRAVERSAL");

    tree.preorder();

    // 42. POSTORDER TRAVERSAL
    println!("\n42. POSTORDER TRAVERSAL");

    tree.postorder();

    // 43. LEVEL ORDER TRAVERSAL
    println!("\n43. LEVEL ORDER TRAVERSAL");

    tree.level_order();

    // 44. SEARCH
    println!("\n44. SEARCH");

    println!("Search 40: {}", tree.search(40));
    println!("Search 99: {}", tree.search(99));

    // 45. MINIMUM AND MAXIMUM
    println!("\n45. MINIMUM AND MAXIMUM");

    println!("Minimum: {:?}", tree.min_value());
    println!("Maximum: {:?}", tree.max_value());

    // 46. NODE COUNTS
    println!("\n46. NODE COUNTS");

    println!("Total nodes: {}", tree.count_nodes());
    println!("Leaf nodes: {}", tree.count_leaves());

    // 47. HEIGHT
    println!("\n47. TREE HEIGHT");

    println!("Height: {}", tree.height());

    // 48. VALIDATE AVL TREE
    println!("\n48. VALIDATE AVL TREE");

    println!("BST property: {}", tree.is_valid_bst());
    println!("AVL balance: {}", tree.is_balanced());

    // 49. CLONE TREE
    println!("\n49. CLONE TREE");

    let cloned_tree = tree.clone_tree();

    println!("Cloned tree inorder:");
    cloned_tree.inorder();

    // 50. DELETE A LEAF NODE
    println!("\n50. DELETE LEAF NODE");

    tree.delete(10);

    println!("After deleting 10:");
    tree.display();

    // 51. DELETE NODE WITH ONE CHILD
    println!("\n51. DELETE NODE WITH ONE CHILD");

    tree.delete(20);

    println!("After deleting 20:");
    tree.display();

    // 52. DELETE NODE WITH TWO CHILDREN
    println!("\n52. DELETE NODE WITH TWO CHILDREN");

    tree.delete(50);

    println!("After deleting 50:");
    tree.display();

    // 53. VALIDATE AFTER DELETIONS
    println!("\n53. VALIDATE AFTER DELETIONS");

    println!("BST property: {}", tree.is_valid_bst());
    println!("AVL balance: {}", tree.is_balanced());

    println!("Inorder:");
    tree.inorder();

    // 54. NODE MEMORY SIZE
    println!("\n54. NODE MEMORY SIZE");

    println!(
        "Node structure size: {} bytes",
        AVLTree::node_memory_size()
    );

    // 55. CLEAR TREE
    println!("\n55. CLEAR TREE");

    tree.clear();

    println!("Is empty: {}", tree.root_value().is_none());
    println!("Node count: {}", tree.count_nodes());

    // 56. FINAL STATUS
    println!("\n56. FINAL STATUS");

    println!("AVL Tree demonstration completed.");
}


/*
//---------------------------------------------------
Important implementation concepts:

- height is maintained in every node.
- balance_factor() detects whether a node is unbalanced.
- rotate_left() and rotate_right() restructure the tree.
- Insertion and deletion rebalance nodes while recursion returns toward the root.
- This implementation uses safe Rust ownership with Box and Option; no unsafe pointers.

//---------------------------------------------------
Time complexity:

Operation - Average / Worst

Search - O(log n)
Insert - O(log n)
Delete - O(log n)
Find minimum - O(log n)
Find maximum  -O(log n)
Traversal - O(n)

//---------------------------------------------------


//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261127_ds_AVLTree_01.rs -o avltree_example

./avltree_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

======================================
       RUST AVL TREE EXAMPLE
======================================

33. CREATE EMPTY AVL TREE
Is empty: true

34. LL ROTATION
Inserted: 30, 20, 10
Root after balancing: Some(20)
    30 (h=1)
20 (h=2)
    10 (h=1)

35. RR ROTATION
Inserted: 10, 20, 30
Root after balancing: Some(20)
    30 (h=1)
20 (h=2)
    10 (h=1)

36. LR ROTATION
Inserted: 30, 10, 20
Root after balancing: Some(20)
    30 (h=1)
20 (h=2)
    10 (h=1)

37. RL ROTATION
Inserted: 10, 30, 20
Root after balancing: Some(20)
    30 (h=1)
20 (h=2)
    10 (h=1)

38. INSERT MULTIPLE VALUES
Inserted: [50, 30, 70, 20, 40, 60, 80, 10, 25, 35, 45]

39. AVL TREE STRUCTURE
        80 (h=1)
    70 (h=2)
        60 (h=1)
50 (h=4)
            45 (h=1)
        40 (h=2)
            35 (h=1)
    30 (h=3)
            25 (h=1)
        20 (h=2)
            10 (h=1)

40. INORDER TRAVERSAL
10 20 25 30 35 40 45 50 60 70 80

41. PREORDER TRAVERSAL
50 30 20 10 25 40 35 45 70 60 80

42. POSTORDER TRAVERSAL
10 25 20 35 45 40 30 60 80 70 50

43. LEVEL ORDER TRAVERSAL
50 30 70 20 40 60 80 10 25 35 45

44. SEARCH
Search 40: true
Search 99: false

45. MINIMUM AND MAXIMUM
Minimum: Some(10)
Maximum: Some(80)

46. NODE COUNTS
Total nodes: 11
Leaf nodes: 6

47. TREE HEIGHT
Height: 4

48. VALIDATE AVL TREE
BST property: true
AVL balance: true

49. CLONE TREE
Cloned tree inorder:
10 20 25 30 35 40 45 50 60 70 80

50. DELETE LEAF NODE
After deleting 10:
        80 (h=1)
    70 (h=2)
        60 (h=1)
50 (h=4)
            45 (h=1)
        40 (h=2)
            35 (h=1)
    30 (h=3)
            25 (h=1)
        20 (h=2)

51. DELETE NODE WITH ONE CHILD
After deleting 20:
        80 (h=1)
    70 (h=2)
        60 (h=1)
50 (h=4)
            45 (h=1)
        40 (h=2)
            35 (h=1)
    30 (h=3)
        25 (h=1)

52. DELETE NODE WITH TWO CHILDREN
After deleting 50:
        80 (h=1)
    70 (h=2)
60 (h=4)
            45 (h=1)
        40 (h=2)
            35 (h=1)
    30 (h=3)
        25 (h=1)

53. VALIDATE AFTER DELETIONS
BST property: true
AVL balance: true
Inorder:
25 30 35 40 45 60 70 80

54. NODE MEMORY SIZE
Node structure size: 24 bytes

55. CLEAR TREE
Is empty: true
Node count: 0

56. FINAL STATUS
AVL Tree demonstration completed.

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/



