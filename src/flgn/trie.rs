struct TrieNode {
    data: char,
    children: Vec<TrieNode>
}

impl TrieNode {
    fn new(c: char) -> Self {
        Self {
            data: c,
            children: Vec::new()
        }
    }
    
    fn is_child(&self, c: char) -> Result<usize, ()> {
        let mut i = 0;
        while i < self.children.len() {
            if self.children[i].data == c {
                return Ok(i);
            }
            i += 1;
        }
        return Err(());
    }
    
    fn add_child(&mut self, c: char) {
        self.children.push(TrieNode::new(c));
    }
}

pub struct Trie{
    roots: Vec<TrieNode>
}

impl Trie {
    pub fn new() -> Self {
        Self {
            roots: Vec::new()
        }
    }
    
    fn is_root(&self, c: char) -> Result<usize, ()> {
        let mut i = 0;
        while i < self.roots.len() {
            if self.roots[i].data == c {
                return Ok(i);
            }
            i += 1;
        }
        return Err(());
    }
    
    fn add_root(&mut self, c: char) {
        self.roots.push(TrieNode::new(c));
    }
    
    pub fn insert_word(&mut self,word: &str) {
        let mut i = 0;
        let mut current_node: &mut TrieNode = &mut TrieNode::new(' ');
        let characters: Vec<char> = word.chars().collect();
        while i < word.len() {
            if i == 0 {
                let node_result = self.is_root(characters[i]);
                match node_result {
                    Ok(node_index) => {
                        current_node = &mut self.roots[node_index];
                    }
                    Err(()) => {
                        let node_index = self.roots.len();
                        self.add_root(characters[i]);
                        current_node = &mut self.roots[node_index];
                    }
                }
            }
            else {
                let node_result = current_node.is_child(characters[i]);
                match node_result {
                    Ok(node_index) => {
                        current_node = &mut current_node.children[node_index];
                    }
                    Err(()) => {
                        let node_index = current_node.children.len();
                        current_node.add_child(characters[i]);
                        current_node = &mut current_node.children[node_index];
                    }
                }
            }
            i += 1;
        }
    }
    
    pub fn contains(&self, word: &str) -> bool {
        let mut i = 0;
        let mut current_node: &TrieNode = &TrieNode::new(' ');
        let characters: Vec<char> = word.chars().collect();
        while i < word.len() {
            if i == 0 {
                let node_result = self.is_root(characters[i]);
                match node_result {
                    Ok(node_index) => {
                        current_node = &self.roots[node_index];
                    }
                    Err(()) => {
                        return false;
                    }
                }
            }
            else {
                let node_result = current_node.is_child(characters[i]);
                match node_result {
                    Ok(node_index) => {
                        current_node = &current_node.children[node_index];
                    }
                    Err(()) => {
                        return false;
                    }
                }
            }
            i += 1;
        }
        return true;
    }
}