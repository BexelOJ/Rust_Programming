//---------------------------------------------------
// BTree - Rust Data Structures
// Date: 20261129
//---------------------------------------------------

use std::mem::size_of;

// ============================================================
// 01. B-TREE NODE
// ============================================================

#[derive(Debug, Clone)]
struct BTreeNode {
    keys: Vec<i32>,
    children: Vec<BTreeNode>,
    is_leaf: bool,
}

// ============================================================
// 02. B-TREE STRUCTURE
// ============================================================

#[derive(Debug, Clone)]
struct BTree {
    root: BTreeNode,
    min_degree: usize,
}

// ============================================================
// 03. IMPLEMENT B-TREE
// ============================================================

impl BTreeNode {

    // 04. CREATE AN EMPTY NODE
    fn new(is_leaf: bool) -> Self {
        Self {
            keys: Vec::new(),
            children: Vec::new(),
            is_leaf,
        }
    }
}

impl BTree {

    // 05. CREATE A B-TREE
    // Minimum degree t must be at least 2.
    fn new(min_degree: usize) -> Self {
        assert!(min_degree >= 2);

        Self {
            root: BTreeNode::new(true),
            min_degree,
        }
    }

    // 06. CHECK WHETHER TREE IS EMPTY
    fn is_empty(&self) -> bool {
        self.root.keys.is_empty()
    }

    // 07. GET MINIMUM DEGREE
    fn degree(&self) -> usize {
        self.min_degree
    }

    // 08. GET MAXIMUM KEYS PER NODE
    fn max_keys(&self) -> usize {
        2 * self.min_degree - 1
    }

    // 09. GET MINIMUM KEYS PER NON-ROOT NODE
    fn min_keys(&self) -> usize {
        self.min_degree - 1
    }

    // 10. SEARCH FOR A KEY
    fn search(&self, key: i32) -> bool {
        Self::search_node(&self.root, key)
    }

    fn search_node(node: &BTreeNode, key: i32) -> bool {
        let mut index = 0;

        while index < node.keys.len() && key > node.keys[index] {
            index += 1;
        }

        if index < node.keys.len() && key == node.keys[index] {
            return true;
        }

        if node.is_leaf {
            return false;
        }

        Self::search_node(&node.children[index], key)
    }

    // 11. INSERT A KEY
    fn insert(&mut self, key: i32) {
        // Ignore duplicates.
        if self.search(key) {
            return;
        }

        // If root is full, split it first.
        if self.root.keys.len() == self.max_keys() {
            let old_root = std::mem::replace(
                &mut self.root,
                BTreeNode::new(false),
            );

            self.root.children.push(old_root);

            Self::split_child(&mut self.root, 0);
        }

        Self::insert_non_full(
            &mut self.root,
            key,
            self.min_degree,
        );
    }

    // 12. INSERT INTO A NON-FULL NODE
    fn insert_non_full(
        node: &mut BTreeNode,
        key: i32,
        degree: usize,
    ) {
        let mut index = node.keys.len();

        if node.is_leaf {
            node.keys.push(key);

            while index > 0 && node.keys[index - 1] > key {
                node.keys[index] = node.keys[index - 1];
                index -= 1;
            }

            node.keys[index] = key;
        } else {
            while index > 0 && key < node.keys[index - 1] {
                index -= 1;
            }

            if node.children[index].keys.len() == 2 * degree - 1 {
                // Split full child.
                Self::split_child(node, index);

                if key > node.keys[index] {
                    index += 1;
                }
            }

            Self::insert_non_full(
                &mut node.children[index],
                key,
                degree,
            );
        }
    }

    // 13. SPLIT A FULL CHILD NODE
    fn split_child(parent: &mut BTreeNode, index: usize) {
        let degree = (parent.children[index].keys.len() + 1) / 2;

        let full_child = &mut parent.children[index];

        let mut new_child = BTreeNode::new(full_child.is_leaf);

        // Median key moves into the parent.
        let median = full_child.keys.remove(degree - 1);

        // Keys after the median move into the new node.
        new_child.keys = full_child.keys.split_off(degree - 1);

        // Move the corresponding children.
        if !full_child.is_leaf {
            new_child.children = full_child.children.split_off(degree);
        }

        parent.keys.insert(index, median);
        parent.children.insert(index + 1, new_child);
    }

    // 14. INORDER TRAVERSAL
    fn inorder(&self) {
        Self::inorder_node(&self.root);
        println!();
    }

    fn inorder_node(node: &BTreeNode) {
        for index in 0..node.keys.len() {
            if !node.is_leaf {
                Self::inorder_node(&node.children[index]);
            }

            print!("{} ", node.keys[index]);
        }

        if !node.is_leaf {
            Self::inorder_node(&node.children[node.keys.len()]);
        }
    }

    // 15. PREORDER TRAVERSAL
    fn preorder(&self) {
        Self::preorder_node(&self.root);
        println!();
    }

    fn preorder_node(node: &BTreeNode) {
        for key in &node.keys {
            print!("{} ", key);
        }

        for child in &node.children {
            Self::preorder_node(child);
        }
    }

    // 16. POSTORDER TRAVERSAL
    fn postorder(&self) {
        Self::postorder_node(&self.root);
        println!();
    }

    fn postorder_node(node: &BTreeNode) {
        for child in &node.children {
            Self::postorder_node(child);
        }

        for key in &node.keys {
            print!("{} ", key);
        }
    }

    // 17. COUNT TOTAL KEYS
    fn count_keys(&self) -> usize {
        Self::count_node_keys(&self.root)
    }

    fn count_node_keys(node: &BTreeNode) -> usize {
        let mut count = node.keys.len();

        for child in &node.children {
            count += Self::count_node_keys(child);
        }

        count
    }

    // 18. COUNT TOTAL NODES
    fn count_nodes(&self) -> usize {
        Self::count_tree_nodes(&self.root)
    }

    fn count_tree_nodes(node: &BTreeNode) -> usize {
        let mut count = 1;

        for child in &node.children {
            count += Self::count_tree_nodes(child);
        }

        count
    }

    // 19. GET TREE HEIGHT
    fn height(&self) -> usize {
        let mut current = &self.root;
        let mut height = 0;

        while !current.is_leaf {
            height += 1;
            current = &current.children[0];
        }

        if self.is_empty() {
            0
        } else {
            height + 1
        }
    }

    // 20. FIND MINIMUM KEY
    fn min_value(&self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        let mut current = &self.root;

        while !current.is_leaf {
            current = &current.children[0];
        }

        current.keys.first().copied()
    }

    // 21. FIND MAXIMUM KEY
    fn max_value(&self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        let mut current = &self.root;

        while !current.is_leaf {
            current = current.children.last().unwrap();
        }

        current.keys.last().copied()
    }

    // 22. DELETE A KEY
    fn delete(&mut self, key: i32) -> bool {
        if !self.search(key) {
            return false;
        }

        Self::delete_node(
            &mut self.root,
            key,
            self.min_degree,
        );

        // If root becomes empty, reduce tree height.
        if self.root.keys.is_empty() && !self.root.is_leaf {
            self.root = self.root.children.remove(0);
        }

        true
    }

    // 23. DELETE FROM A NODE
    fn delete_node(
        node: &mut BTreeNode,
        key: i32,
        degree: usize,
    ) {
        let mut index = 0;

        while index < node.keys.len() && node.keys[index] < key {
            index += 1;
        }

        // KEY FOUND IN CURRENT NODE.
        if index < node.keys.len() && node.keys[index] == key {

            if node.is_leaf {
                // CASE 1: KEY IN LEAF NODE.
                node.keys.remove(index);
            } else {
                // CASE 2: KEY IN INTERNAL NODE.
                Self::delete_from_internal(node, index, degree);
            }

            return;
        }

        if node.is_leaf {
            return;
        }

        // Ensure the child has enough keys before descending.
        let child_index = index;

        if node.children[child_index].keys.len() == degree - 1 {
            Self::fill_child(node, child_index, degree);
        }

        let next_index = if child_index > node.keys.len() {
            child_index - 1
        } else {
            child_index
        };

        Self::delete_node(
            &mut node.children[next_index],
            key,
            degree,
        );
    }

    // 24. DELETE FROM INTERNAL NODE
    fn delete_from_internal(
        node: &mut BTreeNode,
        index: usize,
        degree: usize,
    ) {
        let key = node.keys[index];

        // CASE 2A: LEFT CHILD HAS AT LEAST t KEYS.
        if node.children[index].keys.len() >= degree {
            let predecessor =
                Self::get_predecessor(&node.children[index]);

            node.keys[index] = predecessor;

            Self::delete_node(
                &mut node.children[index],
                predecessor,
                degree,
            );

        // CASE 2B: RIGHT CHILD HAS AT LEAST t KEYS.
        } else if node.children[index + 1].keys.len() >= degree {
            let successor =
                Self::get_successor(&node.children[index + 1]);

            node.keys[index] = successor;

            Self::delete_node(
                &mut node.children[index + 1],
                successor,
                degree,
            );

        // CASE 2C: BOTH CHILDREN HAVE t-1 KEYS.
        } else {
            Self::merge_children(node, index);

            Self::delete_node(
                &mut node.children[index],
                key,
                degree,
            );
        }
    }

    // 25. GET PREDECESSOR
    fn get_predecessor(node: &BTreeNode) -> i32 {
        let mut current = node;

        while !current.is_leaf {
            current = current.children.last().unwrap();
        }

        *current.keys.last().unwrap()
    }

    // 26. GET SUCCESSOR
    fn get_successor(node: &BTreeNode) -> i32 {
        let mut current = node;

        while !current.is_leaf {
            current = &current.children[0];
        }

        current.keys[0]
    }

    // 27. FILL A CHILD BEFORE DESCENDING
    fn fill_child(
        parent: &mut BTreeNode,
        index: usize,
        degree: usize,
    ) {
        // Borrow from previous sibling.
        if index > 0
            && parent.children[index - 1].keys.len() >= degree
        {
            Self::borrow_from_previous(parent, index);
        }

        // Borrow from next sibling.
        else if index < parent.keys.len()
            && parent.children[index + 1].keys.len() >= degree
        {
            Self::borrow_from_next(parent, index);
        }

        // Merge with a sibling.
        else if index < parent.keys.len() {
            Self::merge_children(parent, index);
        } else {
            Self::merge_children(parent, index - 1);
        }
    }

    // 28. BORROW FROM PREVIOUS SIBLING
    fn borrow_from_previous(
        parent: &mut BTreeNode,
        index: usize,
    ) {
        let (left_slice, right_slice) =
            parent.children.split_at_mut(index);

        let sibling = &mut left_slice[index - 1];
        let child = &mut right_slice[0];

        // Parent key moves into child.
        child.keys.insert(0, parent.keys[index - 1]);

        // Sibling's largest key moves into parent.
        parent.keys[index - 1] = sibling.keys.pop().unwrap();

        // Move child pointer if internal nodes.
        if !sibling.is_leaf {
            let moved_child = sibling.children.pop().unwrap();
            child.children.insert(0, moved_child);
        }
    }

    // 29. BORROW FROM NEXT SIBLING
    fn borrow_from_next(
        parent: &mut BTreeNode,
        index: usize,
    ) {
        let (left_slice, right_slice) =
            parent.children.split_at_mut(index + 1);

        let child = &mut left_slice[index];
        let sibling = &mut right_slice[0];

        // Parent key moves into child.
        child.keys.push(parent.keys[index]);

        // Sibling's smallest key moves into parent.
        parent.keys[index] = sibling.keys.remove(0);

        // Move child pointer if internal nodes.
        if !sibling.is_leaf {
            let moved_child = sibling.children.remove(0);
            child.children.push(moved_child);
        }
    }

    // 30. MERGE TWO CHILDREN
    fn merge_children(
        parent: &mut BTreeNode,
        index: usize,
    ) {
        let separator = parent.keys.remove(index);

        let mut right = parent.children.remove(index + 1);
        let left = &mut parent.children[index];

        left.keys.push(separator);
        left.keys.append(&mut right.keys);

        left.children.append(&mut right.children);
    }

    // 31. DISPLAY TREE STRUCTURE
    fn display(&self) {
        Self::display_node(&self.root, 0);
    }

    fn display_node(node: &BTreeNode, depth: usize) {
        println!(
            "{}{:?}",
            "    ".repeat(depth),
            node.keys
        );

        for child in &node.children {
            Self::display_node(child, depth + 1);
        }
    }

    // 32. VALIDATE B-TREE
    fn is_valid(&self) -> bool {
        if self.root.keys.len() > self.max_keys() {
            return false;
        }

        if self.root.is_leaf && !self.root.children.is_empty() {
            return false;
        }

        if !self.root.is_leaf && self.root.keys.is_empty() {
            return false;
        }

        let mut leaf_depth = None;

        Self::validate_node(
            &self.root,
            self.min_degree,
            true,
            None,
            None,
            0,
            &mut leaf_depth,
        )
    }

    fn validate_node(
        node: &BTreeNode,
        degree: usize,
        is_root: bool,
        min: Option<i32>,
        max: Option<i32>,
        depth: usize,
        leaf_depth: &mut Option<usize>,
    ) -> bool {
        let key_count = node.keys.len();

        if key_count > 2 * degree - 1 {
            return false;
        }

        if !is_root && key_count < degree - 1 {
            return false;
        }

        // Keys must be strictly increasing.
        for pair in node.keys.windows(2) {
            if pair[0] >= pair[1] {
                return false;
            }
        }

        // Keys must respect ancestor bounds.
        for &key in &node.keys {
            if let Some(min_value) = min {
                if key <= min_value {
                    return false;
                }
            }

            if let Some(max_value) = max {
                if key >= max_value {
                    return false;
                }
            }
        }

        if node.is_leaf {
            if !node.children.is_empty() {
                return false;
            }

            match *leaf_depth {
                Some(existing) => {
                    if existing != depth {
                        return false;
                    }
                }
                None => {
                    *leaf_depth = Some(depth);
                }
            }

            return true;
        }

        // An internal node must have exactly key_count + 1 children.
        if node.children.len() != key_count + 1 {
            return false;
        }

        for index in 0..node.children.len() {
            let child_min = if index == 0 {
                min
            } else {
                Some(node.keys[index - 1])
            };

            let child_max = if index == key_count {
                max
            } else {
                Some(node.keys[index])
            };

            if !Self::validate_node(
                &node.children[index],
                degree,
                false,
                child_min,
                child_max,
                depth + 1,
                leaf_depth,
            ) {
                return false;
            }
        }

        true
    }

    // 33. TREE MEMORY SIZE
    fn node_descriptor_size() -> usize {
        size_of::<BTreeNode>()
    }

    // 34. CLONE TREE
    fn clone_tree(&self) -> Self {
        self.clone()
    }

    // 35. CLEAR TREE
    fn clear(&mut self) {
        self.root = BTreeNode::new(true);
    }
}

// ============================================================
// 36. MAIN FUNCTION
// ============================================================

fn main() {

    println!("======================================");
    println!("          RUST B-TREE EXAMPLE");
    println!("======================================");

    // 37. CREATE B-TREE
    println!("\n37. CREATE B-TREE");

    let mut tree = BTree::new(3);

    println!("Minimum degree: {}", tree.degree());
    println!("Maximum keys per node: {}", tree.max_keys());
    println!("Minimum keys per non-root node: {}", tree.min_keys());

    // 38. CHECK EMPTY TREE
    println!("\n38. CHECK EMPTY TREE");

    println!("Is empty: {}", tree.is_empty());

    // 39. INSERT VALUES
    println!("\n39. INSERT VALUES");

    let values = [
        10, 20, 5, 6, 12, 30, 7, 17,
        3, 2, 25, 40, 50, 60, 70, 80,
    ];

    for value in values {
        tree.insert(value);

        println!(
            "Inserted {:>3} | Valid: {}",
            value,
            tree.is_valid()
        );
    }

    // 40. DISPLAY TREE
    println!("\n40. B-TREE STRUCTURE");

    tree.display();

    // 41. INORDER TRAVERSAL
    println!("\n41. INORDER TRAVERSAL");

    tree.inorder();

    // 42. PREORDER TRAVERSAL
    println!("\n42. PREORDER TRAVERSAL");

    tree.preorder();

    // 43. POSTORDER TRAVERSAL
    println!("\n43. POSTORDER TRAVERSAL");

    tree.postorder();

    // 44. SEARCH
    println!("\n44. SEARCH");

    println!("Search 20: {}", tree.search(20));
    println!("Search 99: {}", tree.search(99));

    // 45. INSERT DUPLICATE
    println!("\n45. INSERT DUPLICATE");

    let old_count = tree.count_keys();

    tree.insert(20);

    println!("Before: {}", old_count);
    println!("After: {}", tree.count_keys());

    // 46. MINIMUM AND MAXIMUM
    println!("\n46. MINIMUM AND MAXIMUM");

    println!("Minimum: {:?}", tree.min_value());
    println!("Maximum: {:?}", tree.max_value());

    // 47. COUNT KEYS AND NODES
    println!("\n47. COUNT KEYS AND NODES");

    println!("Total keys: {}", tree.count_keys());
    println!("Total nodes: {}", tree.count_nodes());

    // 48. TREE HEIGHT
    println!("\n48. TREE HEIGHT");

    println!("Height: {}", tree.height());

    // 49. VALIDATE B-TREE
    println!("\n49. VALIDATE B-TREE");

    println!("Is valid: {}", tree.is_valid());

    // 50. CLONE TREE
    println!("\n50. CLONE TREE");

    let cloned_tree = tree.clone_tree();

    println!("Cloned tree inorder:");
    cloned_tree.inorder();

    println!("Cloned tree valid: {}", cloned_tree.is_valid());

    // 51. DELETE A LEAF KEY
    println!("\n51. DELETE LEAF KEY");

    println!("Deleted 2: {}", tree.delete(2));

    tree.display();

    println!("Valid: {}", tree.is_valid());

    // 52. DELETE AN INTERNAL KEY
    println!("\n52. DELETE INTERNAL KEY");

    println!("Deleted 30: {}", tree.delete(30));

    tree.display();

    println!("Valid: {}", tree.is_valid());

    // 53. DELETE MORE KEYS
    println!("\n53. DELETE MORE KEYS");

    for value in [5, 6, 7, 10, 12, 17, 20, 25] {
        let deleted = tree.delete(value);

        println!(
            "Delete {:>3} | Deleted: {} | Valid: {}",
            value,
            deleted,
            tree.is_valid()
        );
    }

    tree.display();

    // 54. NODE MEMORY SIZE
    println!("\n54. NODE MEMORY SIZE");

    println!(
        "BTreeNode descriptor size: {} bytes",
        BTree::node_descriptor_size()
    );

    // 55. CLEAR TREE
    println!("\n55. CLEAR TREE");

    tree.clear();

    println!("Is empty: {}", tree.is_empty());
    println!("Total keys: {}", tree.count_keys());

    // 56. FINAL STATUS
    println!("\n56. FINAL STATUS");

    println!("B-Tree demonstration completed.");
}


/*
//---------------------------------------------------
A B-Tree is a self-balancing multiway search tree. Unlike a Binary Search Tree, where each node has at most two children, a B-Tree node can contain multiple keys and multiple children.

//---------------------------------------------------
B-Trees are particularly important in:
- Database indexing (MySQL, PostgreSQL).
- Filesystems.
- Disk-based storage systems.
- Large datasets where reducing disk access is important.
1. B-Tree structure
For a B-Tree with minimum degree \(t = 3\):
- Maximum keys per node = \(2t - 1 = 5\).
- Minimum keys per non-root node = \(t - 1 = 2\).
- Maximum children per internal node = \(2t = 6\).
- Keys inside each node are sorted.

//---------------------------------------------------
Important B-Tree operations:

Operation	Description

Insert	Insert a key in sorted position
Split	Divide a full node and promote its median
Search	Find a key by following child ranges
Borrow	Transfer a key from a sibling
Merge	Combine two children and a parent separator
Delete	Remove a key while preserving B-Tree rules
Traversal	Visit keys in sorted order

//---------------------------------------------------
B-Tree vs Binary Search Tree vs AVL Tree
Feature	BST	AVL	B-Tree
Maximum children	2	2	Multiple
Balancing	No	Height-based	Node splitting/merging
Search	O(h)	O(log n)	O(log n)
Insertion	O(h)	O(log n)	O(log n)
Deletion	O(h)	O(log n)	O(log n)
Common application	General trees	In-memory indexing	Database/storage indexing

//---------------------------------------------------
Important concepts to understand:

- BTreeNode stores multiple sorted keys.
- children stores child nodes directly using Vec<BTreeNode>.
- min_degree = 3 means a node can hold up to five keys.
- split_child() promotes the median key when a node becomes full.
- borrow_from_previous() and borrow_from_next() redistribute keys during deletion.
- merge_children() combines two nodes when borrowing is not possible.

//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261129_ds_BTree_01.rs -o btree_example

./btree_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

======================================
          RUST B-TREE EXAMPLE
======================================

37. CREATE B-TREE
Minimum degree: 3
Maximum keys per node: 5
Minimum keys per non-root node: 2

38. CHECK EMPTY TREE
Is empty: true

39. INSERT VALUES
Inserted  10 | Valid: true
Inserted  20 | Valid: true
Inserted   5 | Valid: true
Inserted   6 | Valid: true
Inserted  12 | Valid: true
Inserted  30 | Valid: true
Inserted   7 | Valid: true
Inserted  17 | Valid: true
Inserted   3 | Valid: true
Inserted   2 | Valid: true
Inserted  25 | Valid: true
Inserted  40 | Valid: true
Inserted  50 | Valid: true
Inserted  60 | Valid: true
Inserted  70 | Valid: true
Inserted  80 | Valid: true

40. B-TREE STRUCTURE
[10, 20, 40]
    [2, 3, 5, 6, 7]
    [12, 17]
    [25, 30]
    [50, 60, 70, 80]

41. INORDER TRAVERSAL
2 3 5 6 7 10 12 17 20 25 30 40 50 60 70 80

42. PREORDER TRAVERSAL
10 20 40 2 3 5 6 7 12 17 25 30 50 60 70 80

43. POSTORDER TRAVERSAL
2 3 5 6 7 12 17 25 30 50 60 70 80 10 20 40

44. SEARCH
Search 20: true
Search 99: false

45. INSERT DUPLICATE
Before: 16
After: 16

46. MINIMUM AND MAXIMUM
Minimum: Some(2)
Maximum: Some(80)

47. COUNT KEYS AND NODES
Total keys: 16
Total nodes: 5

48. TREE HEIGHT
Height: 2

49. VALIDATE B-TREE
Is valid: true

50. CLONE TREE
Cloned tree inorder:
2 3 5 6 7 10 12 17 20 25 30 40 50 60 70 80
Cloned tree valid: true

51. DELETE LEAF KEY
Deleted 2: true
[10, 20, 40]
    [3, 5, 6, 7]
    [12, 17]
    [25, 30]
    [50, 60, 70, 80]
Valid: true

52. DELETE INTERNAL KEY
Deleted 30: true
[10, 20, 50]
    [3, 5, 6, 7]
    [12, 17]
    [25, 40]
    [60, 70, 80]
Valid: true

53. DELETE MORE KEYS
Delete   5 | Deleted: true | Valid: true
Delete   6 | Deleted: true | Valid: true
Delete   7 | Deleted: true | Valid: true
Delete  10 | Deleted: true | Valid: true
Delete  12 | Deleted: true | Valid: true
Delete  17 | Deleted: true | Valid: true
Delete  20 | Deleted: true | Valid: true
Delete  25 | Deleted: true | Valid: true
[50]
    [3, 40]
    [60, 70, 80]

54. NODE MEMORY SIZE
BTreeNode descriptor size: 56 bytes

55. CLEAR TREE
Is empty: true
Total keys: 0

56. FINAL STATUS
B-Tree demonstration completed.

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/



