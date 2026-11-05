//---------------------------------------------------
// RedBlackTree - Rust Data Structures
// Date: 20261128
//---------------------------------------------------

use std::collections::VecDeque;
use std::mem::size_of;

// ============================================================
// 01. NODE COLOUR
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

// ============================================================
// 02. RED-BLACK TREE NODE
// ============================================================

#[derive(Debug, Clone)]
struct Node {
    data: i32,
    color: Color,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

// ============================================================
// 03. RED-BLACK TREE STRUCTURE
// ============================================================

#[derive(Debug, Clone)]
struct RedBlackTree {
    nodes: Vec<Node>,
    root: Option<usize>,
}

// ============================================================
// 04. IMPLEMENT RED-BLACK TREE
// ============================================================

impl RedBlackTree {

    // 05. CREATE EMPTY TREE
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root: None,
        }
    }

    // 06. CHECK WHETHER TREE IS EMPTY
    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    // 07. GET NODE COLOUR
    // A missing node is considered BLACK (NIL leaf).
    fn color_of(&self, index: Option<usize>) -> Color {
        match index {
            Some(i) => self.nodes[i].color,
            None => Color::Black,
        }
    }

    // 08. CREATE A NEW RED NODE
    fn create_node(&mut self, data: i32, parent: Option<usize>) -> usize {
        let index = self.nodes.len();

        self.nodes.push(Node {
            data,
            color: Color::Red,
            parent,
            left: None,
            right: None,
        });

        index
    }

    // 09. LEFT ROTATION
    //
    //       x                 y
    //      / \               / \
    //     T1  y     ==>     x  T3
    //        / \           / \
    //       T2 T3         T1 T2

    fn rotate_left(&mut self, x: usize) {
        let y = self.nodes[x]
            .right
            .expect("Right child required for left rotation");

        let t2 = self.nodes[y].left;

        // Move T2 to the right of x.
        self.nodes[x].right = t2;

        if let Some(t2_index) = t2 {
            self.nodes[t2_index].parent = Some(x);
        }

        // Connect y to the old parent of x.
        let old_parent = self.nodes[x].parent;
        self.nodes[y].parent = old_parent;

        match old_parent {
            None => {
                self.root = Some(y);
            }
            Some(parent) => {
                if self.nodes[parent].left == Some(x) {
                    self.nodes[parent].left = Some(y);
                } else {
                    self.nodes[parent].right = Some(y);
                }
            }
        }

        // Put x on the left of y.
        self.nodes[y].left = Some(x);
        self.nodes[x].parent = Some(y);
    }

    // 10. RIGHT ROTATION
    //
    //         y             x
    //        / \           / \
    //       x  T3  ==>    T1  y
    //      / \               / \
    //     T1 T2             T2 T3

    fn rotate_right(&mut self, y: usize) {
        let x = self.nodes[y]
            .left
            .expect("Left child required for right rotation");

        let t2 = self.nodes[x].right;

        // Move T2 to the left of y.
        self.nodes[y].left = t2;

        if let Some(t2_index) = t2 {
            self.nodes[t2_index].parent = Some(y);
        }

        // Connect x to the old parent of y.
        let old_parent = self.nodes[y].parent;
        self.nodes[x].parent = old_parent;

        match old_parent {
            None => {
                self.root = Some(x);
            }
            Some(parent) => {
                if self.nodes[parent].left == Some(y) {
                    self.nodes[parent].left = Some(x);
                } else {
                    self.nodes[parent].right = Some(x);
                }
            }
        }

        // Put y on the right of x.
        self.nodes[x].right = Some(y);
        self.nodes[y].parent = Some(x);
    }

    // 11. INSERT A VALUE
    fn insert(&mut self, data: i32) {
        let mut parent = None;
        let mut current = self.root;

        // Find the insertion position.
        while let Some(index) = current {
            parent = Some(index);

            if data < self.nodes[index].data {
                current = self.nodes[index].left;
            } else if data > self.nodes[index].data {
                current = self.nodes[index].right;
            } else {
                // Ignore duplicate values.
                return;
            }
        }

        // Create the new node as RED.
        let new_index = self.create_node(data, parent);

        // Attach the new node.
        match parent {
            None => {
                self.root = Some(new_index);
            }
            Some(parent_index) => {
                if data < self.nodes[parent_index].data {
                    self.nodes[parent_index].left = Some(new_index);
                } else {
                    self.nodes[parent_index].right = Some(new_index);
                }
            }
        }

        // Restore Red-Black Tree properties.
        self.fix_insert(new_index);
    }

    // 12. FIX RED-BLACK TREE AFTER INSERTION
    fn fix_insert(&mut self, mut z: usize) {
        while let Some(parent) = self.nodes[z].parent {
            if self.nodes[parent].color == Color::Black {
                break;
            }

            let grandparent = self.nodes[parent]
                .parent
                .expect("Red parent must have a grandparent");

            // PARENT IS LEFT CHILD OF GRANDPARENT.
            if self.nodes[grandparent].left == Some(parent) {
                let uncle = self.nodes[grandparent].right;

                // CASE 1: UNCLE IS RED.
                if self.color_of(uncle) == Color::Red {
                    self.nodes[parent].color = Color::Black;

                    if let Some(uncle_index) = uncle {
                        self.nodes[uncle_index].color = Color::Black;
                    }

                    self.nodes[grandparent].color = Color::Red;

                    z = grandparent;
                } else {
                    // CASE 2: TRIANGLE (LEFT-RIGHT).
                    if self.nodes[parent].right == Some(z) {
                        z = parent;
                        self.rotate_left(z);
                    }

                    // CASE 3: LINE (LEFT-LEFT).
                    let parent = self.nodes[z].parent.unwrap();
                    let grandparent =
                        self.nodes[parent].parent.unwrap();

                    self.nodes[parent].color = Color::Black;
                    self.nodes[grandparent].color = Color::Red;

                    self.rotate_right(grandparent);
                }
            } else {
                // PARENT IS RIGHT CHILD OF GRANDPARENT.
                let uncle = self.nodes[grandparent].left;

                // CASE 1: UNCLE IS RED.
                if self.color_of(uncle) == Color::Red {
                    self.nodes[parent].color = Color::Black;

                    if let Some(uncle_index) = uncle {
                        self.nodes[uncle_index].color = Color::Black;
                    }

                    self.nodes[grandparent].color = Color::Red;

                    z = grandparent;
                } else {
                    // CASE 2: TRIANGLE (RIGHT-LEFT).
                    if self.nodes[parent].left == Some(z) {
                        z = parent;
                        self.rotate_right(z);
                    }

                    // CASE 3: LINE (RIGHT-RIGHT).
                    let parent = self.nodes[z].parent.unwrap();
                    let grandparent =
                        self.nodes[parent].parent.unwrap();

                    self.nodes[parent].color = Color::Black;
                    self.nodes[grandparent].color = Color::Red;

                    self.rotate_left(grandparent);
                }
            }
        }

        // Root must always be BLACK.
        if let Some(root) = self.root {
            self.nodes[root].color = Color::Black;
            self.nodes[root].parent = None;
        }
    }

    // 13. SEARCH FOR A VALUE
    fn search(&self, target: i32) -> bool {
        let mut current = self.root;

        while let Some(index) = current {
            if target == self.nodes[index].data {
                return true;
            }

            if target < self.nodes[index].data {
                current = self.nodes[index].left;
            } else {
                current = self.nodes[index].right;
            }
        }

        false
    }

    // 14. FIND MINIMUM VALUE
    fn min_value(&self) -> Option<i32> {
        let mut current = self.root?;

        while let Some(left) = self.nodes[current].left {
            current = left;
        }

        Some(self.nodes[current].data)
    }

    // 15. FIND MAXIMUM VALUE
    fn max_value(&self) -> Option<i32> {
        let mut current = self.root?;

        while let Some(right) = self.nodes[current].right {
            current = right;
        }

        Some(self.nodes[current].data)
    }

    // 16. INORDER TRAVERSAL
    fn inorder(&self) {
        Self::inorder_node(&self.nodes, self.root);
        println!();
    }

    fn inorder_node(nodes: &[Node], index: Option<usize>) {
        if let Some(i) = index {
            Self::inorder_node(nodes, nodes[i].left);
            print!("{}({:?}) ", nodes[i].data, nodes[i].color);
            Self::inorder_node(nodes, nodes[i].right);
        }
    }

    // 17. PREORDER TRAVERSAL
    fn preorder(&self) {
        Self::preorder_node(&self.nodes, self.root);
        println!();
    }

    fn preorder_node(nodes: &[Node], index: Option<usize>) {
        if let Some(i) = index {
            print!("{}({:?}) ", nodes[i].data, nodes[i].color);
            Self::preorder_node(nodes, nodes[i].left);
            Self::preorder_node(nodes, nodes[i].right);
        }
    }

    // 18. POSTORDER TRAVERSAL
    fn postorder(&self) {
        Self::postorder_node(&self.nodes, self.root);
        println!();
    }

    fn postorder_node(nodes: &[Node], index: Option<usize>) {
        if let Some(i) = index {
            Self::postorder_node(nodes, nodes[i].left);
            Self::postorder_node(nodes, nodes[i].right);
            print!("{}({:?}) ", nodes[i].data, nodes[i].color);
        }
    }

    // 19. LEVEL ORDER TRAVERSAL
    fn level_order(&self) {
        let mut queue: VecDeque<usize> = VecDeque::new();

        if let Some(root) = self.root {
            queue.push_back(root);
        }

        while let Some(index) = queue.pop_front() {
            let node = &self.nodes[index];

            print!("{}({:?}) ", node.data, node.color);

            if let Some(left) = node.left {
                queue.push_back(left);
            }

            if let Some(right) = node.right {
                queue.push_back(right);
            }
        }

        println!();
    }

    // 20. COUNT NODES
    fn count_nodes(&self) -> usize {
        self.nodes.len()
    }

    // 21. GET TREE HEIGHT
    fn height(&self) -> usize {
        Self::node_height(&self.nodes, self.root)
    }

    fn node_height(nodes: &[Node], index: Option<usize>) -> usize {
        match index {
            Some(i) => {
                1 + Self::node_height(nodes, nodes[i].left)
                    .max(Self::node_height(nodes, nodes[i].right))
            }
            None => 0,
        }
    }

    // 22. GET ROOT VALUE
    fn root_value(&self) -> Option<i32> {
        self.root.map(|index| self.nodes[index].data)
    }

    // 23. GET ROOT COLOUR
    fn root_color(&self) -> Option<Color> {
        self.root.map(|index| self.nodes[index].color)
    }

    // 24. COUNT RED NODES
    fn count_red_nodes(&self) -> usize {
        self.nodes
            .iter()
            .filter(|node| node.color == Color::Red)
            .count()
    }

    // 25. COUNT BLACK NODES
    fn count_black_nodes(&self) -> usize {
        self.nodes
            .iter()
            .filter(|node| node.color == Color::Black)
            .count()
    }

    // 26. CALCULATE BLACK HEIGHT
    // NIL leaves are counted as BLACK.
    fn black_height(&self) -> Option<usize> {
        Self::calculate_black_height(&self.nodes, self.root)
    }

    fn calculate_black_height(
        nodes: &[Node],
        index: Option<usize>,
    ) -> Option<usize> {
        match index {
            None => Some(1),

            Some(i) => {
                let left_height =
                    Self::calculate_black_height(nodes, nodes[i].left)?;

                let right_height =
                    Self::calculate_black_height(nodes, nodes[i].right)?;

                if left_height != right_height {
                    return None;
                }

                let extra_black =
                    usize::from(nodes[i].color == Color::Black);

                Some(left_height + extra_black)
            }
        }
    }

    // 27. VALIDATE BST ORDERING
    fn is_valid_bst(&self) -> bool {
        Self::validate_bst(&self.nodes, self.root, None, None)
    }

    fn validate_bst(
        nodes: &[Node],
        index: Option<usize>,
        min: Option<i32>,
        max: Option<i32>,
    ) -> bool {
        match index {
            None => true,

            Some(i) => {
                let value = nodes[i].data;

                if let Some(min_value) = min {
                    if value <= min_value {
                        return false;
                    }
                }

                if let Some(max_value) = max {
                    if value >= max_value {
                        return false;
                    }
                }

                Self::validate_bst(
                    nodes,
                    nodes[i].left,
                    min,
                    Some(value),
                ) && Self::validate_bst(
                    nodes,
                    nodes[i].right,
                    Some(value),
                    max,
                )
            }
        }
    }

    // 28. VALIDATE RED-BLACK COLOUR RULES
    fn has_valid_colors(&self) -> bool {
        if let Some(root) = self.root {
            // Root must be BLACK.
            if self.nodes[root].color != Color::Black {
                return false;
            }

            Self::validate_colors(&self.nodes, Some(root))
        } else {
            true
        }
    }

    fn validate_colors(nodes: &[Node], index: Option<usize>) -> bool {
        match index {
            None => true,

            Some(i) => {
                let current = &nodes[i];

                // A RED node cannot have a RED child.
                if current.color == Color::Red {
                    if current.left.map(|j| nodes[j].color)
                        == Some(Color::Red)
                    {
                        return false;
                    }

                    if current.right.map(|j| nodes[j].color)
                        == Some(Color::Red)
                    {
                        return false;
                    }
                }

                Self::validate_colors(nodes, current.left)
                    && Self::validate_colors(nodes, current.right)
            }
        }
    }

    // 29. VALIDATE PARENT LINKS
    fn has_valid_parent_links(&self) -> bool {
        if let Some(root) = self.root {
            if self.nodes[root].parent.is_some() {
                return false;
            }
        }

        for (index, node) in self.nodes.iter().enumerate() {
            if let Some(left) = node.left {
                if self.nodes[left].parent != Some(index) {
                    return false;
                }
            }

            if let Some(right) = node.right {
                if self.nodes[right].parent != Some(index) {
                    return false;
                }
            }
        }

        true
    }

    // 30. VALIDATE COMPLETE RED-BLACK TREE
    fn is_valid(&self) -> bool {
        self.is_valid_bst()
            && self.has_valid_colors()
            && self.black_height().is_some()
            && self.has_valid_parent_links()
    }

    // 31. DISPLAY TREE STRUCTURE
    fn display(&self) {
        Self::display_node(&self.nodes, self.root, 0);
    }

    fn display_node(
        nodes: &[Node],
        index: Option<usize>,
        depth: usize,
    ) {
        if let Some(i) = index {
            Self::display_node(nodes, nodes[i].right, depth + 1);

            let color_symbol = match nodes[i].color {
                Color::Red => "R",
                Color::Black => "B",
            };

            println!(
                "{}{} [{}]",
                "    ".repeat(depth),
                nodes[i].data,
                color_symbol
            );

            Self::display_node(nodes, nodes[i].left, depth + 1);
        }
    }

    // 32. GET NODE MEMORY SIZE
    fn node_memory_size() -> usize {
        size_of::<Node>()
    }

    // 33. CLEAR TREE
    fn clear(&mut self) {
        self.nodes.clear();
        self.root = None;
    }
}

// ============================================================
// 34. MAIN FUNCTION
// ============================================================

fn main() {

    println!("======================================");
    println!("      RUST RED-BLACK TREE EXAMPLE");
    println!("======================================");

    // 35. CREATE EMPTY TREE
    println!("\n35. CREATE EMPTY TREE");

    let mut tree = RedBlackTree::new();

    println!("Is empty: {}", tree.is_empty());

    // 36. INSERT ROOT NODE
    println!("\n36. INSERT ROOT NODE");

    tree.insert(50);

    println!("Root value: {:?}", tree.root_value());
    println!("Root colour: {:?}", tree.root_color());

    // 37. INSERT MULTIPLE VALUES
    println!("\n37. INSERT MULTIPLE VALUES");

    let values = [30, 70, 20, 40, 60, 80, 10, 25, 35, 45, 55, 65];

    for value in values {
        tree.insert(value);

        println!(
            "Inserted {:>3} | Valid: {}",
            value,
            tree.is_valid()
        );
    }

    // 38. DISPLAY TREE
    println!("\n38. TREE STRUCTURE");

    tree.display();

    // 39. INORDER TRAVERSAL
    println!("\n39. INORDER TRAVERSAL");

    tree.inorder();

    // 40. PREORDER TRAVERSAL
    println!("\n40. PREORDER TRAVERSAL");

    tree.preorder();

    // 41. POSTORDER TRAVERSAL
    println!("\n41. POSTORDER TRAVERSAL");

    tree.postorder();

    // 42. LEVEL ORDER TRAVERSAL
    println!("\n42. LEVEL ORDER TRAVERSAL");

    tree.level_order();

    // 43. SEARCH
    println!("\n43. SEARCH");

    println!("Search 40: {}", tree.search(40));
    println!("Search 99: {}", tree.search(99));

    // 44. MINIMUM AND MAXIMUM
    println!("\n44. MINIMUM AND MAXIMUM");

    println!("Minimum: {:?}", tree.min_value());
    println!("Maximum: {:?}", tree.max_value());

    // 45. NODE COUNTS
    println!("\n45. NODE COUNTS");

    println!("Total nodes: {}", tree.count_nodes());
    println!("Red nodes: {}", tree.count_red_nodes());
    println!("Black nodes: {}", tree.count_black_nodes());

    // 46. TREE HEIGHT
    println!("\n46. TREE HEIGHT");

    println!("Height: {}", tree.height());

    // 47. BLACK HEIGHT
    println!("\n47. BLACK HEIGHT");

    println!("Black height: {:?}", tree.black_height());

    // 48. VALIDATE BST PROPERTY
    println!("\n48. VALIDATE BST PROPERTY");

    println!("Valid BST: {}", tree.is_valid_bst());

    // 49. VALIDATE RED-BLACK RULES
    println!("\n49. VALIDATE RED-BLACK RULES");

    println!("Valid colours: {}", tree.has_valid_colors());
    println!("Valid parent links: {}", tree.has_valid_parent_links());
    println!("Complete validation: {}", tree.is_valid());

    // 50. INSERT DUPLICATE
    println!("\n50. INSERT DUPLICATE");

    let old_count = tree.count_nodes();

    tree.insert(50);

    println!("Node count before: {}", old_count);
    println!("Node count after: {}", tree.count_nodes());

    // 51. INSERT SORTED VALUES
    println!("\n51. INSERT SORTED VALUES");

    let mut sorted_tree = RedBlackTree::new();

    for value in 1..=15 {
        sorted_tree.insert(value);
    }

    println!("Inserted values: 1 to 15");
    println!("Root: {:?}", sorted_tree.root_value());
    println!("Height: {}", sorted_tree.height());
    println!("Valid: {}", sorted_tree.is_valid());

    sorted_tree.display();

    // 52. CLONE TREE
    println!("\n52. CLONE TREE");

    let cloned_tree = tree.clone();

    println!("Cloned tree inorder:");
    cloned_tree.inorder();

    println!("Cloned tree valid: {}", cloned_tree.is_valid());

    // 53. NODE MEMORY SIZE
    println!("\n53. NODE MEMORY SIZE");

    println!(
        "Node structure size: {} bytes",
        RedBlackTree::node_memory_size()
    );

    // 54. CLEAR TREE
    println!("\n54. CLEAR TREE");

    tree.clear();

    println!("Is empty: {}", tree.is_empty());
    println!("Node count: {}", tree.count_nodes());

    // 55. FINAL STATUS
    println!("\n55. FINAL STATUS");

    println!("Red-Black Tree demonstration completed.");
}


/*
//---------------------------------------------------
A Red-Black Tree is a self-balancing Binary Search Tree (BST). It maintains balance using two techniques:
1. Recoloring – changing nodes between Red and Black.
2. Rotations – restructuring the tree using left and right rotations.
Unlike an AVL tree, which maintains node heights and balance factors, a Red-Black Tree maintains a set of colour-based rules.

//---------------------------------------------------
Red-Black Tree properties
1. Every node is either Red or Black.
2. The root is always Black.
3. All missing children (NIL leaves) are considered Black.
4. A Red node cannot have a Red parent or Red child.
5. Every path from a node to its descendant NIL leaves must contain the same number of Black nodes.

//---------------------------------------------------


//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261128_ds_RedBlackTree_01.rs -o redblacktree_example

./redblacktree_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

======================================
      RUST RED-BLACK TREE EXAMPLE
======================================

35. CREATE EMPTY TREE
Is empty: true

36. INSERT ROOT NODE
Root value: Some(50)
Root colour: Some(Black)

37. INSERT MULTIPLE VALUES
Inserted  30 | Valid: true
Inserted  70 | Valid: true
Inserted  20 | Valid: true
Inserted  40 | Valid: true
Inserted  60 | Valid: true
Inserted  80 | Valid: true
Inserted  10 | Valid: true
Inserted  25 | Valid: true
Inserted  35 | Valid: true
Inserted  45 | Valid: true
Inserted  55 | Valid: true
Inserted  65 | Valid: true

38. TREE STRUCTURE
        80 [B]
    70 [R]
            65 [R]
        60 [B]
            55 [R]
50 [B]
            45 [R]
        40 [B]
            35 [R]
    30 [R]
            25 [R]
        20 [B]
            10 [R]

39. INORDER TRAVERSAL
10(Red) 20(Black) 25(Red) 30(Red) 35(Red) 40(Black) 45(Red) 50(Black) 55(Red) 60(Black) 65(Red) 70(Red) 80(Black)

40. PREORDER TRAVERSAL
50(Black) 30(Red) 20(Black) 10(Red) 25(Red) 40(Black) 35(Red) 45(Red) 70(Red) 60(Black) 55(Red) 65(Red) 80(Black)

41. POSTORDER TRAVERSAL
10(Red) 25(Red) 20(Black) 35(Red) 45(Red) 40(Black) 30(Red) 55(Red) 65(Red) 60(Black) 80(Black) 70(Red) 50(Black)

42. LEVEL ORDER TRAVERSAL
50(Black) 30(Red) 70(Red) 20(Black) 40(Black) 60(Black) 80(Black) 10(Red) 25(Red) 35(Red) 45(Red) 55(Red) 65(Red)

43. SEARCH
Search 40: true
Search 99: false

44. MINIMUM AND MAXIMUM
Minimum: Some(10)
Maximum: Some(80)

45. NODE COUNTS
Total nodes: 13
Red nodes: 8
Black nodes: 5

46. TREE HEIGHT
Height: 4

47. BLACK HEIGHT
Black height: Some(3)

48. VALIDATE BST PROPERTY
Valid BST: true

49. VALIDATE RED-BLACK RULES
Valid colours: true
Valid parent links: true
Complete validation: true

50. INSERT DUPLICATE
Node count before: 13
Node count after: 13

51. INSERT SORTED VALUES
Inserted values: 1 to 15
Root: Some(4)
Height: 6
Valid: true
                    15 [R]
                14 [B]
                    13 [R]
            12 [R]
                11 [B]
        10 [B]
            9 [B]
    8 [R]
            7 [B]
        6 [B]
            5 [B]
4 [B]
        3 [B]
    2 [B]
        1 [B]

52. CLONE TREE
Cloned tree inorder:
10(Red) 20(Black) 25(Red) 30(Red) 35(Red) 40(Black) 45(Red) 50(Black) 55(Red) 60(Black) 65(Red) 70(Red) 80(Black)
Cloned tree valid: true

53. NODE MEMORY SIZE
Node structure size: 56 bytes

54. CLEAR TREE
Is empty: true
Node count: 0

55. FINAL STATUS
Red-Black Tree demonstration completed.

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/


