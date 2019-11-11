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

    fn find_successor(&self, key: usize) -> Option<&TrieNode<T>> {
        let mut low = 0;
        let mut high = self.nr_levels;
        let mut successor_node: Option<*mut TrieNode<T>> = None;
        
        // find the lowest common ancestor- a node which shares maximum common prefix with the key
        while high >= low {
            let mid = (low + high)/2;
            let prefix = key >> (self.nr_levels - mid);
            //check the presence of an internal node with the keyed as `prefix` in hashmap at the `mid` level 
            match self.level_maps[mid].get(&prefix) {
                Some(&value) => {
                    low = mid + 1;
                    successor_node = Some(value.as_ptr());
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
        let mut low = 0;
        let mut high = self.nr_levels;
        let mut predecessor_node: Option<*mut TrieNode<T>> = None;
        
        // find the lowest common ancestor- a node which shares maximum common prefix with the key
        while high >= low {
            let mid = (low + high)/2;
            let prefix = key >> (self.nr_levels - mid);
            //check the presence of an internal node with the keyed as `prefix` in hashmap at the `mid` level 
            match self.level_maps[mid].get(&prefix) {
                Some(&value) => {
                    println!("mid{}", mid);
                    low = mid+1;
                    predecessor_node = Some(value.as_ptr());
                }
                None => {
                    // prevent out of bound subtraction of a usize
                    if mid == 0 {
                        break;
                    }
                    high = mid-1;
                }
            }
        }

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

}