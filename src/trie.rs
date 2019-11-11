use std::collections::HashMap;
use std::ptr::NonNull;

type Node<T> = NonNull<TrieNode<T>>;
#[derive(Debug)]
pub struct TrieNode<T> {
    key: usize,
    value: Option<T>,
    level: usize,
    right: Option<NonNull<TrieNode<T>>>,
    left: Option<NonNull<TrieNode<T>>>,
    is_desc_left: bool,
    is_desc_right: bool,
}

#[derive(Debug)]
pub struct Xfast<T=String> {
    nr_levels: usize,
    level_maps: Vec<HashMap<usize, NonNull<TrieNode<T>>>>,
}

impl<T> TrieNode<T> {
    
    pub fn new(key: usize, value: T, level: usize) -> Box<Self> {
        Box::new(TrieNode{
            key: key,
            value: Some(value),
            level: level,
            right: None,
            left: None,
            is_desc_right: true,
            is_desc_left: true,
        })
    }

    // constructor for internal nodes
    pub fn new_internal(level: usize) -> Box<Self> {
        Box::new(TrieNode{
            key: 0,
            value: None,
            level,
            right: None,
            left: None,
            is_desc_left: true,
            is_desc_right: true,
        })
    }

    // return the rightmost node for @cur_node as parent
    // @max_level: max possible height of the trie
    fn get_rightmost_node(max_level: usize, mut cur_node: *mut TrieNode<T>) -> Option<Node<T>> {
        unsafe {
            while (*cur_node).level != max_level {
                match (*cur_node).right {
                    Some(right_node) => {
                        cur_node = right_node.as_ptr();
                    }
                    None => {
                        (*cur_node).left.map(|left_node| {
                            cur_node = left_node.as_ptr();
                        });
                    }
                }
            }
            NonNull::new(cur_node as *mut TrieNode<T>)
        }
    }

    // return the leftmost node for @cur_node as parent
    // @max_level: max possible height of the trie
    fn get_leftmost_node(max_level: usize, mut cur_node: *mut TrieNode<T>) -> Option<Node<T>> {
        unsafe {
            while (*cur_node).level != max_level {
                match (*cur_node).left {
                    Some(left_node) => {
                        cur_node = left_node.as_ptr();
                    }
                    None => {
                        (*cur_node).right.map(|right_node| {
                            cur_node = right_node.as_ptr();
                        });
                    }
                }
            }
            NonNull::new(cur_node as *mut TrieNode<T>)
        }
    }
}

impl<T> Xfast<T> {
    
    pub fn new(range: usize) -> Self {
        let nr_levels = Xfast::<T>::get_levels_count(range);
        let level_maps = Xfast::create_map_list(nr_levels+1);
        let mut new_trie = Xfast {
            nr_levels,
            level_maps,
        };

        // insert the root node in the trie at level 0
        let root_node = TrieNode::new_internal(0);
        let root_node = Box::into_raw_non_null(root_node);
        new_trie.level_maps[0].insert(0, root_node);
        new_trie
    }

    // levels => height of the trie
    fn get_levels_count(mut range: usize) -> usize {
        let mut levels = 0;
        while range > 0 {
            range >>= 1;
            levels += 1;
        }
        levels
    }

    // helper fn for populating a vector list of hashmaps
    fn create_map_list(nr_levels: usize) -> Vec<HashMap<usize, Node<T>>> {
        let mut map_list: Vec<HashMap<usize, Node<T>>> = Vec::new();
        for _level in 0..nr_levels {
            let level_hash: HashMap<usize, Node<T>> = HashMap::new();
            map_list.push(level_hash);
        }
        map_list
    }
    fn find_lowest_common_ancestor(&self, key: usize) -> Option<*mut TrieNode<T>> {
        let mut low = 0;
        let mut high = self.nr_levels;
        let mut ancestor_node: Option<*mut TrieNode<T>> = None;

        while high >= low {
            let mid = (low + high)/2;
            let prefix = key >> (self.nr_levels - mid);
            //check the presence of an internal node with the keyed as `prefix` in hashmap at the `mid` level 
            match self.level_maps[mid].get(&prefix) {
                Some(&value) => {
                    low = mid + 1;
                    ancestor_node = Some(value.as_ptr());
                }
                None => {
                    // prevent out of bound subtraction of a usize
                    if mid == 0 {
                        break;
                    }
                    high = mid - 1;
                }
            }
        }
        ancestor_node
    }

    fn find_successor(&self, key: usize) -> Option<&TrieNode<T>> {
        // find the lowest common ancestor- a node which shares maximum common prefix with the key
        let successor_node: Option<*mut TrieNode<T>> = self.find_lowest_common_ancestor(key);
        match successor_node {
            Some(mut node) => unsafe {
                // successor of a key already present is the key itself
                if (*node).level == (self.nr_levels) {
                    return Some(&(*node));
                }

                //right subtree of an internal node can have the successor
                if (key >> (self.nr_levels - (*node).level -1 ) & 1) != 0 {
                    (*node).right.map(|right_node| {
                        node = right_node.as_ptr();
                    });
                }
                else {
                    //left subtree of the internal node has the successor
                    (*node).left.map(|left_node| {
                        node = left_node.as_ptr();
                    });
                }
                                
                // in case the key of the successor node (leaf node) above calculated has lower key than the currently searched key
                // navigate using the right and left pointer of the leaf node to find the smallest node which has a key >= the key being searched
                if (*node).key < key {
                    let mut temp_node = None;
                    (*node).right.map(|right_node| {
                        temp_node = Some(&(*right_node.as_ptr()));
                    });
                    return temp_node;
                }
                return Some(&(*node));
            }
            None => {
                return None;
            }
        }
    }

    fn find_predecessor(&self, key: usize) -> Option<&TrieNode<T>> {
        // find the lowest common ancestor- a node which shares maximum common prefix with the key
        let predecessor_node: Option<*mut TrieNode<T>> = self.find_lowest_common_ancestor(key);
        match predecessor_node {
            Some(mut node) => unsafe {
                // predecessor of a key already present is the key itself
                if (*node).level == (self.nr_levels) {
                    return Some(&(*node));
                }

                if (key>>(self.nr_levels - (*node).level -1) &1) != 0 {
                    (*node).right.map(|right_node| {
                        node = right_node.as_ptr();
                    });
                }
                else {
                    (*node).left.map(|left_node| {
                        node = left_node.as_ptr();
                    });
                }

                if (*node).key > key {
                    let mut temp_node = None;
                    (*node).left.map(|left_node| {
                        temp_node = Some(&(*left_node.as_ptr()));
                    });
                    return temp_node;
                }
                return Some(&(*node));
            }
            None => {
                return None;
            }
        }
    }

    fn populate_internal_nodes(&mut self, key: usize) {
        let mut level = 1;
        let max_levels = self.nr_levels;
        while level < max_levels {
            let prefix = key >> (max_levels - level);
            if let None = self.level_maps[level].get(&prefix) {
                let temp_node = TrieNode::new_internal(level);
                let temp_node = Box::into_raw_non_null(temp_node);
                self.level_maps[level].insert(prefix, temp_node);
                // add to the right child if the bit is 1 at that index else make it the left child
                if (prefix & 1) != 0 {
                    let temp_prefix = prefix >> 1;
                    self.level_maps[level-1].get(&temp_prefix).map(|&value| unsafe{
                        (*value.as_ptr()).right = Some(temp_node);
                        (*value.as_ptr()).is_desc_right = false;
                    });
                }
                else {
                    let temp_prefix = prefix >> 1;
                    self.level_maps[level-1].get(&temp_prefix).map(|&value| unsafe{
                        (*value.as_ptr()).left = Some(temp_node);
                        (*value.as_ptr()).is_desc_left = false;
                    }); 
                }
            }
            level += 1;
        }
    }

    fn update_descendant_ptr(&mut self, key: usize) {
        let mut prefix = key;
        let mut level = self.nr_levels - 1;

        while level > 0 {
            prefix = prefix >> 1;
            // find an internal node prefixed as `prefix` at `level` in the level_map
            self.level_maps[level].get(&prefix).map(|&value| unsafe {
                //check if this node has a left child
                match (*value.as_ptr()).left {
                    //the internal node doesn't have a left child
                    None => {
                        //An internal node is inserted in a trie only when it has one its children
                        //Therefore, this node has a right child which is used to find its descendant ptr
                        (*value.as_ptr()).right.map(|right_node| {
                            (*value.as_ptr()).left = TrieNode::get_leftmost_node(self.nr_levels, right_node.as_ptr());
                            (*value.as_ptr()).is_desc_left = true;
                        });
                    },
                    // Left child is present
                    Some(left_ptr) => {
                        //this internal node can have a right child or not
                        match (*value.as_ptr()).right {
                            // the right child is not present
                            None => {
                                (*value.as_ptr()).right = TrieNode::get_rightmost_node(self.nr_levels, left_ptr.as_ptr());
                                (*value.as_ptr()).is_desc_right = true;
                            }
                            // right child is also present
                            Some(right_ptr)=> {
                                // if any of the left or the right child is associated with a descendant pointer then update with the latest descendant pointer. 
                                // At any given instance only one descendant ptr can be present
                                if (*value.as_ptr()).is_desc_right {
                                    (*value.as_ptr()).right = TrieNode::get_rightmost_node(self.nr_levels, left_ptr.as_ptr());
                                }
                                else if (*value.as_ptr()).is_desc_left {
                                    (*value.as_ptr()).left = TrieNode::get_leftmost_node(self.nr_levels, right_ptr.as_ptr());
                                }
                            }
                        }
                    }
                }
            });
            level -= 1;
        }

        // update the descendant ptr for the root node
        self.level_maps[0].get(&0).map(|&value| unsafe {
            let is_left_descendant = (*value.as_ptr()).is_desc_left;
            let is_right_descendant = (*value.as_ptr()).is_desc_right;
            if is_left_descendant {
                (*value.as_ptr()).right.map(|right_node| {
                    (*value.as_ptr()).left = TrieNode::get_leftmost_node(self.nr_levels, right_node.as_ptr());
                });
            }
            if is_right_descendant {
                println!("editing root");
                (*value.as_ptr()).left.map(|left_node| {
                    (*value.as_ptr()).right = TrieNode::get_rightmost_node(self.nr_levels, left_node.as_ptr());
                });
            }
        });
    }

    pub fn insert_key(&mut self, key: usize, value: T) {
        //create a new node with key and val
        let new_node = TrieNode::new(key, value, self.nr_levels);
        let new_node = Some(Box::into_raw_non_null(new_node));
        //find predecessor and successor for the new node
        let predecessor = self.find_predecessor(key);
        let successor = self.find_successor(key);
        
        //update the right and left pointers of the new node to refer to its successors and predecessors resp.
        //update the right ptr in the predecessor ,and left ptr in the successor with the new_node.
        predecessor.map(|pred_node| unsafe{
            //FIXME
            let pred_node = &(*pred_node) as *const TrieNode<T> as *mut TrieNode<T>;
            new_node.map(|node| {
                (*node.as_ptr()).right = (*pred_node).right;
                (*node.as_ptr()).left = NonNull::new(pred_node);
            });
            (*pred_node).right = new_node;
        });

        successor.map(|suc_node| unsafe{
            let suc_node = &(*suc_node) as *const TrieNode<T> as *mut TrieNode<T>;
            new_node.map(|node| {
                (*node.as_ptr()).left = (*suc_node).left;
                (*node.as_ptr()).right = NonNull::new(suc_node);
            });
            (*suc_node).left = new_node;
        });

        //populate intermediate iternal nodes on the path down the new_node
        self.populate_internal_nodes(key);
        
        //insert the new_node at the last level and update the ptr of its parent node using the prefix bit
        self.level_maps[self.nr_levels].insert(key, new_node.unwrap());
        let temp_key = key >> 1;
        self.level_maps[self.nr_levels-1].get(&temp_key).map(|&value| unsafe {
            if (key & 1) != 0 {
                (*value.as_ptr()).right= new_node;
                (*value.as_ptr()).is_desc_right = false;
            }
            else {
                (*value.as_ptr()).left = new_node;
                (*value.as_ptr()).is_desc_left = false;
            }
        });

        // update descendant ptrs
        self.update_descendant_ptr(key);
    }

}