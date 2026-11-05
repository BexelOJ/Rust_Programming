//---------------------------------------------------
// Trie - Rust Data Structures
// Date: 20261130
//---------------------------------------------------

use std::collections::HashMap;
use std::mem::size_of;

// ============================================================
// 01. TRIE NODE
// ============================================================

#[derive(Debug, Clone)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

// ============================================================
// 02. TRIE DATA STRUCTURE
// ============================================================

#[derive(Debug, Clone)]
struct Trie {
    root: TrieNode,
    word_count: usize,
}

impl Trie {
    // ========================================================
    // 03. CREATE A NEW TRIE
    // ========================================================

    fn new() -> Self {
        Self {
            root: TrieNode::new(),
            word_count: 0,
        }
    }

    // ========================================================
    // 04. CHECK WHETHER TRIE IS EMPTY
    // ========================================================

    fn is_empty(&self) -> bool {
        self.word_count == 0
    }

    // ========================================================
    // 05. GET NUMBER OF WORDS
    // ========================================================

    fn len(&self) -> usize {
        self.word_count
    }

    // ========================================================
    // 06. INSERT A WORD
    // ========================================================

    fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;

        for character in word.chars() {
            current = current
                .children
                .entry(character)
                .or_insert_with(TrieNode::new);
        }

        if !current.is_end_of_word {
            current.is_end_of_word = true;
            self.word_count += 1;
        }
    }

    // ========================================================
    // 07. SEARCH FOR A COMPLETE WORD
    // ========================================================

    fn search(&self, word: &str) -> bool {
        let mut current = &self.root;

        for character in word.chars() {
            match current.children.get(&character) {
                Some(node) => current = node,
                None => return false,
            }
        }

        current.is_end_of_word
    }

    // ========================================================
    // 08. CHECK WHETHER A PREFIX EXISTS
    // ========================================================

    fn starts_with(&self, prefix: &str) -> bool {
        let mut current = &self.root;

        for character in prefix.chars() {
            match current.children.get(&character) {
                Some(node) => current = node,
                None => return false,
            }
        }

        true
    }

    // ========================================================
    // 09. GET NODE FOR A PREFIX
    // ========================================================

    fn get_node(&self, prefix: &str) -> Option<&TrieNode> {
        let mut current = &self.root;

        for character in prefix.chars() {
            current = current.children.get(&character)?;
        }

        Some(current)
    }

    // ========================================================
    // 10. COUNT ALL NODES
    // ========================================================

    fn count_nodes(&self) -> usize {
        fn count(node: &TrieNode) -> usize {
            let mut total = 1;

            for child in node.children.values() {
                total += count(child);
            }

            total
        }

        count(&self.root)
    }

    // ========================================================
    // 11. COUNT WORDS UNDER A NODE
    // ========================================================

    fn count_words_from(node: &TrieNode) -> usize {
        let mut total = usize::from(node.is_end_of_word);

        for child in node.children.values() {
            total += Self::count_words_from(child);
        }

        total
    }

    // ========================================================
    // 12. COUNT WORDS WITH A GIVEN PREFIX
    // ========================================================

    fn count_prefix_words(&self, prefix: &str) -> usize {
        match self.get_node(prefix) {
            Some(node) => Self::count_words_from(node),
            None => 0,
        }
    }

    // ========================================================
    // 13. COLLECT ALL WORDS
    // ========================================================

    fn collect_words(node: &TrieNode, current: &mut String, result: &mut Vec<String>) {
        if node.is_end_of_word {
            result.push(current.clone());
        }

        let mut characters: Vec<char> =
            node.children.keys().copied().collect();

        characters.sort_unstable();

        for character in characters {
            if let Some(child) = node.children.get(&character) {
                current.push(character);

                Self::collect_words(child, current, result);

                current.pop();
            }
        }
    }

    // ========================================================
    // 14. GET ALL STORED WORDS
    // ========================================================

    fn all_words(&self) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();

        Self::collect_words(&self.root, &mut current, &mut result);

        result
    }

    // ========================================================
    // 15. GET WORDS WITH A PREFIX
    // ========================================================

    fn words_with_prefix(&self, prefix: &str) -> Vec<String> {
        let mut result = Vec::new();

        if let Some(node) = self.get_node(prefix) {
            let mut current = prefix.to_string();

            Self::collect_words(node, &mut current, &mut result);
        }

        result
    }

    // ========================================================
    // 16. AUTOCOMPLETE SUGGESTIONS
    // ========================================================

    fn autocomplete(&self, prefix: &str, limit: usize) -> Vec<String> {
        self.words_with_prefix(prefix)
            .into_iter()
            .take(limit)
            .collect()
    }

    // ========================================================
    // 17. FIND LONGEST PREFIX THAT IS A COMPLETE WORD
    // ========================================================

    fn longest_word_prefix(&self, input: &str) -> Option<String> {
        let mut current = &self.root;
        let mut prefix = String::new();
        let mut longest = None;

        for character in input.chars() {
            match current.children.get(&character) {
                Some(node) => {
                    prefix.push(character);
                    current = node;

                    if current.is_end_of_word {
                        longest = Some(prefix.clone());
                    }
                }
                None => break,
            }
        }

        longest
    }

    // ========================================================
    // 18. DELETE A WORD
    // ========================================================

    fn delete(&mut self, word: &str) -> bool {
        let characters: Vec<char> = word.chars().collect();

        let deleted = Self::delete_recursive(
            &mut self.root,
            &characters,
            0,
        );

        if deleted {
            self.word_count -= 1;
        }

        deleted
    }

    // ========================================================
    // 19. RECURSIVE DELETE HELPER
    // ========================================================

    fn delete_recursive(
        node: &mut TrieNode,
        characters: &[char],
        depth: usize,
    ) -> bool {
        if depth == characters.len() {
            if !node.is_end_of_word {
                return false;
            }

            node.is_end_of_word = false;
            return true;
        }

        let character = characters[depth];

        let deleted = match node.children.get_mut(&character) {
            Some(child) => Self::delete_recursive(
                child,
                characters,
                depth + 1,
            ),
            None => false,
        };

        if deleted {
            let should_remove = node.children
                .get(&character)
                .map(|child| {
                    !child.is_end_of_word && child.is_leaf()
                })
                .unwrap_or(false);

            if should_remove {
                node.children.remove(&character);
            }
        }

        deleted
    }

    // ========================================================
    // 20. DELETE ALL WORDS WITH A PREFIX
    // ========================================================

    fn delete_prefix(&mut self, prefix: &str) -> usize {
        if prefix.is_empty() {
            let deleted = self.word_count;
            self.root = TrieNode::new();
            self.word_count = 0;
            return deleted;
        }

        let characters: Vec<char> = prefix.chars().collect();

        let mut current = &mut self.root;

        for character in &characters[..characters.len() - 1] {
            match current.children.get_mut(character) {
                Some(node) => current = node,
                None => return 0,
            }
        }

        let last_character = *characters.last().unwrap();

        let removed = current.children.remove(&last_character);

        match removed {
            Some(node) => {
                let deleted = Self::count_words_from(&node);
                self.word_count -= deleted;
                deleted
            }
            None => 0,
        }
    }

    // ========================================================
    // 21. CHECK WHETHER A NODE IS A LEAF
    // ========================================================

    fn is_prefix_leaf(&self, prefix: &str) -> bool {
        match self.get_node(prefix) {
            Some(node) => node.is_leaf(),
            None => false,
        }
    }

    // ========================================================
    // 22. GET WORD LENGTH IN CHARACTERS
    // ========================================================

    fn word_length(word: &str) -> usize {
        word.chars().count()
    }

    // ========================================================
    // 23. GET LONGEST STORED WORD
    // ========================================================

    fn longest_word(&self) -> Option<String> {
        self.all_words()
            .into_iter()
            .max_by_key(|word| word.chars().count())
    }

    // ========================================================
    // 24. GET SHORTEST STORED WORD
    // ========================================================

    fn shortest_word(&self) -> Option<String> {
        self.all_words()
            .into_iter()
            .min_by_key(|word| word.chars().count())
    }

    // ========================================================
    // 25. CHECK WHETHER ALL WORDS HAVE A PREFIX
    // ========================================================

    fn all_words_have_prefix(&self, prefix: &str) -> bool {
        self.all_words()
            .iter()
            .all(|word| word.starts_with(prefix))
    }

    // ========================================================
    // 26. DISPLAY TRIE WORDS
    // ========================================================

    fn display(&self) {
        println!("Stored words:");

        for word in self.all_words() {
            println!("  {}", word);
        }
    }

    // ========================================================
    // 27. DISPLAY TRIE STRUCTURE
    // ========================================================

    fn display_structure(&self) {
        fn display_node(
            node: &TrieNode,
            prefix: &str,
            depth: usize,
        ) {
            let mut characters: Vec<char> =
                node.children.keys().copied().collect();

            characters.sort_unstable();

            for character in characters {
                if let Some(child) = node.children.get(&character) {
                    let new_prefix = format!("{}{}", prefix, character);

                    println!(
                        "{}{}{}",
                        "  ".repeat(depth),
                        character,
                        if child.is_end_of_word { " *" } else { "" }
                    );

                    display_node(child, &new_prefix, depth + 1);
                }
            }
        }

        println!("ROOT");

        display_node(&self.root, "", 0);
    }

    // ========================================================
    // 28. CLEAR TRIE
    // ========================================================

    fn clear(&mut self) {
        self.root = TrieNode::new();
        self.word_count = 0;
    }

    // ========================================================
    // 29. CLONE TRIE
    // ========================================================

    fn clone_trie(&self) -> Self {
        self.clone()
    }

    // ========================================================
    // 30. MEMORY SIZE OF TRIE DESCRIPTOR
    // ========================================================

    fn memory_size(&self) -> usize {
        size_of::<Self>()
    }
}

// ============================================================
// 31. MAIN FUNCTION
// ============================================================

fn main() {

    // --------------------------------------------------------
    // 32. CREATE A TRIE
    // --------------------------------------------------------

    println!("\n32. CREATE A TRIE");

    let mut trie = Trie::new();

    println!("Trie created successfully");
    println!("Is empty: {}", trie.is_empty());

    // --------------------------------------------------------
    // 33. INSERT WORDS
    // --------------------------------------------------------

    println!("\n33. INSERT WORDS");

    trie.insert("apple");
    trie.insert("application");
    trie.insert("apply");
    trie.insert("app");
    trie.insert("banana");
    trie.insert("ball");
    trie.insert("bat");
    trie.insert("cat");
    trie.insert("car");
    trie.insert("care");

    println!("Words inserted successfully");

    // --------------------------------------------------------
    // 34. DISPLAY ALL WORDS
    // --------------------------------------------------------

    println!("\n34. DISPLAY ALL WORDS");

    trie.display();

    // --------------------------------------------------------
    // 35. DISPLAY TRIE STRUCTURE
    // --------------------------------------------------------

    println!("\n35. DISPLAY TRIE STRUCTURE");

    trie.display_structure();

    // --------------------------------------------------------
    // 36. SEARCH COMPLETE WORDS
    // --------------------------------------------------------

    println!("\n36. SEARCH COMPLETE WORDS");

    println!("apple       : {}", trie.search("apple"));
    println!("app         : {}", trie.search("app"));
    println!("application : {}", trie.search("application"));
    println!("apples      : {}", trie.search("apples"));
    println!("ban         : {}", trie.search("ban"));

    // --------------------------------------------------------
    // 37. CHECK PREFIX
    // --------------------------------------------------------

    println!("\n37. CHECK PREFIX");

    println!("app : {}", trie.starts_with("app"));
    println!("ban : {}", trie.starts_with("ban"));
    println!("xyz : {}", trie.starts_with("xyz"));

    // --------------------------------------------------------
    // 38. GET WORD COUNT
    // --------------------------------------------------------

    println!("\n38. GET WORD COUNT");

    println!("Total words: {}", trie.len());

    // --------------------------------------------------------
    // 39. COUNT NODES
    // --------------------------------------------------------

    println!("\n39. COUNT NODES");

    println!("Total nodes: {}", trie.count_nodes());

    // --------------------------------------------------------
    // 40. COUNT WORDS WITH PREFIX
    // --------------------------------------------------------

    println!("\n40. COUNT WORDS WITH PREFIX");

    println!("Prefix app : {}", trie.count_prefix_words("app"));
    println!("Prefix ba  : {}", trie.count_prefix_words("ba"));
    println!("Prefix ca  : {}", trie.count_prefix_words("ca"));

    // --------------------------------------------------------
    // 41. GET WORDS WITH PREFIX
    // --------------------------------------------------------

    println!("\n41. GET WORDS WITH PREFIX");

    println!("app words: {:?}", trie.words_with_prefix("app"));
    println!("ba words : {:?}", trie.words_with_prefix("ba"));
    println!("ca words : {:?}", trie.words_with_prefix("ca"));

    // --------------------------------------------------------
    // 42. AUTOCOMPLETE
    // --------------------------------------------------------

    println!("\n42. AUTOCOMPLETE");

    println!("Suggestions for 'app': {:?}", trie.autocomplete("app", 3));
    println!("Suggestions for 'ba' : {:?}", trie.autocomplete("ba", 2));

    // --------------------------------------------------------
    // 43. LONGEST WORD PREFIX
    // --------------------------------------------------------

    println!("\n43. LONGEST WORD PREFIX");

    println!(
        "Input: applepie, Longest word prefix: {:?}",
        trie.longest_word_prefix("applepie")
    );

    println!(
        "Input: applicationX, Longest word prefix: {:?}",
        trie.longest_word_prefix("applicationX")
    );

    // --------------------------------------------------------
    // 44. DELETE A WORD
    // --------------------------------------------------------

    println!("\n44. DELETE A WORD");

    println!("Delete apple: {}", trie.delete("apple"));
    println!("Search apple: {}", trie.search("apple"));

    // Other words sharing the prefix remain.
    println!("Search app: {}", trie.search("app"));
    println!("Search application: {}", trie.search("application"));

    // --------------------------------------------------------
    // 45. DELETE A NON-EXISTING WORD
    // --------------------------------------------------------

    println!("\n45. DELETE A NON-EXISTING WORD");

    println!("Delete mango: {}", trie.delete("mango"));

    // --------------------------------------------------------
    // 46. LONGEST AND SHORTEST WORD
    // --------------------------------------------------------

    println!("\n46. LONGEST AND SHORTEST WORD");

    println!("Longest word : {:?}", trie.longest_word());
    println!("Shortest word: {:?}", trie.shortest_word());

    // --------------------------------------------------------
    // 47. CHECK PREFIX LEAF
    // --------------------------------------------------------

    println!("\n47. CHECK PREFIX LEAF");

    println!("apple : {}", trie.is_prefix_leaf("apple"));
    println!("banana: {}", trie.is_prefix_leaf("banana"));

    // --------------------------------------------------------
    // 48. WORD LENGTH
    // --------------------------------------------------------

    println!("\n48. WORD LENGTH");

    println!("Length of application: {}", Trie::word_length("application"));

    // --------------------------------------------------------
    // 49. DUPLICATE INSERTION
    // --------------------------------------------------------

    println!("\n49. DUPLICATE INSERTION");

    let count_before = trie.len();

    trie.insert("banana");

    println!("Word count before: {}", count_before);
    println!("Word count after : {}", trie.len());

    // --------------------------------------------------------
    // 50. DELETE ALL WORDS WITH PREFIX
    // --------------------------------------------------------

    println!("\n50. DELETE ALL WORDS WITH PREFIX");

    let deleted = trie.delete_prefix("ba");

    println!("Deleted words: {}", deleted);
    println!("Remaining words:");

    trie.display();

    // --------------------------------------------------------
    // 51. ALL WORDS HAVE PREFIX
    // --------------------------------------------------------

    println!("\n51. ALL WORDS HAVE PREFIX");

    println!(
        "All words start with 'a': {}",
        trie.all_words_have_prefix("a")
    );

    // --------------------------------------------------------
    // 52. CLONE TRIE
    // --------------------------------------------------------

    println!("\n52. CLONE TRIE");

    let cloned_trie = trie.clone_trie();

    println!("Original count: {}", trie.len());
    println!("Cloned count  : {}", cloned_trie.len());

    // --------------------------------------------------------
    // 53. MEMORY SIZE
    // --------------------------------------------------------

    println!("\n53. MEMORY SIZE");

    println!(
        "Trie descriptor size: {} bytes",
        trie.memory_size()
    );

    println!(
        "TrieNode descriptor size: {} bytes",
        size_of::<TrieNode>()
    );

    println!("Note: These sizes exclude heap allocations.");

    // --------------------------------------------------------
    // 54. UNICODE SUPPORT
    // --------------------------------------------------------

    println!("\n54. UNICODE SUPPORT");

    let mut unicode_trie = Trie::new();

    unicode_trie.insert("കേരളം");
    unicode_trie.insert("നമസ്കാരം");
    unicode_trie.insert("こんにちは");
    unicode_trie.insert("नमस्ते");

    println!("Search കേരളം: {}", unicode_trie.search("കേരളം"));
    println!("Search नमस्ते: {}", unicode_trie.search("नमस्ते"));

    unicode_trie.display();

    // --------------------------------------------------------
    // 55. CLEAR TRIE
    // --------------------------------------------------------

    println!("\n55. CLEAR TRIE");

    trie.clear();

    println!("Is empty: {}", trie.is_empty());
    println!("Word count: {}", trie.len());
    println!("Node count: {}", trie.count_nodes());

    // --------------------------------------------------------
    // 56. FINAL STATUS
    // --------------------------------------------------------

    println!("\n56. FINAL STATUS");

    println!("Trie demonstration completed successfully.");
}


/*
//---------------------------------------------------
A Trie is a tree-based data structure used to store and search strings efficiently. It is commonly used in:

- Autocomplete and search suggestions.
- Dictionary applications.
- Spell checkers.
- Prefix-based searching.
- Contact name searches.
- Word games and spell-checking systems.

//---------------------------------------------------


//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261130_ds_Trie_01.rs -o trie_example

./trie_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

32. CREATE A TRIE
Trie created successfully
Is empty: true

33. INSERT WORDS
Words inserted successfully

34. DISPLAY ALL WORDS
Stored words:
  app
  apple
  application
  apply
  ball
  banana
  bat
  car
  care
  cat

35. DISPLAY TRIE STRUCTURE
ROOT
a
  p
    p *
      l
        e *
        i
          c
            a
              t
                i
                  o
                    n *
        y *
b
  a
    l
      l *
    n
      a
        n
          a *
    t *
c
  a
    r *
      e *
    t *

36. SEARCH COMPLETE WORDS
apple       : true
app         : true
application : true
apples      : false
ban         : false

37. CHECK PREFIX
app : true
ban : true
xyz : false

38. GET WORD COUNT
Total words: 10

39. COUNT NODES
Total nodes: 28

40. COUNT WORDS WITH PREFIX
Prefix app : 4
Prefix ba  : 3
Prefix ca  : 3

41. GET WORDS WITH PREFIX
app words: ["app", "apple", "application", "apply"]
ba words : ["ball", "banana", "bat"]
ca words : ["car", "care", "cat"]

42. AUTOCOMPLETE
Suggestions for 'app': ["app", "apple", "application"]
Suggestions for 'ba' : ["ball", "banana"]

43. LONGEST WORD PREFIX
Input: applepie, Longest word prefix: Some("apple")
Input: applicationX, Longest word prefix: Some("application")

44. DELETE A WORD
Delete apple: true
Search apple: false
Search app: true
Search application: true

45. DELETE A NON-EXISTING WORD
Delete mango: false

46. LONGEST AND SHORTEST WORD
Longest word : Some("application")
Shortest word: Some("app")

47. CHECK PREFIX LEAF
apple : false
banana: true

48. WORD LENGTH
Length of application: 11

49. DUPLICATE INSERTION
Word count before: 9
Word count after : 9

50. DELETE ALL WORDS WITH PREFIX
Deleted words: 3
Remaining words:
Stored words:
  app
  application
  apply
  car
  care
  cat

51. ALL WORDS HAVE PREFIX
All words start with 'a': false

52. CLONE TRIE
Original count: 6
Cloned count  : 6

53. MEMORY SIZE
Trie descriptor size: 64 bytes
TrieNode descriptor size: 56 bytes
Note: These sizes exclude heap allocations.

54. UNICODE SUPPORT
Search കേരളം: true
Search नमस्ते: true
Stored words:
  नमस्ते
  കേരളം
  നമസ്കാരം
  こんにちは

55. CLEAR TRIE
Is empty: true
Word count: 0
Node count: 1

56. FINAL STATUS
Trie demonstration completed successfully.

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/



