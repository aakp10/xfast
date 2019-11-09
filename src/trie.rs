use std::collections::HashMap;
use std::ptr::NonNull;

type Node<T> = NonNull<TrieNode<T>>;
#[derive(Debug)]
struct TrieNode<T> {
    key: usize,
    value: Option<T>,
    level: usize,
    right: Option<NonNull<TrieNode<T>>>,
    left: Option<NonNull<TrieNode<T>>>,
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

