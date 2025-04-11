struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    pub fn new_tree(val: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if val.is_empty() {
            return None;
        }

        let mut index = 0;
        Self::build_tree(val, &mut index)
    }

    fn build_tree(val: &[Option<i32>], index: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
        if *index >= val.len() {
            return None;
        }

        let current_val = val[*index]?;
        *index += 1;

        let node = Rc::new(RefCell::new(TreeNode::new(current_val)));
        node.borrow_mut().left = Self::build_tree(val, index);
        node.borrow_mut().right = Self::build_tree(val, index);

        Some(node)
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        if let Some(node) = root {
            let node = node.borrow();
            result.extend(Solution::inorder_traversal(node.left.clone()));
            result.push(node.val);
            result.extend(Solution::inorder_traversal(node.right.clone()));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // fn test_inorder_traversal() {
    #[test]
    fn test_example_1() {
        let root = [Some(1), None, Some(2), Some(3)];
        let root = TreeNode::new_tree(&root);
        let output = vec![1, 3, 2];
        assert_eq!(Solution::inorder_traversal(root), output);
    }

    #[test]
    fn test_example_2() {
        let root = [
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            None,
            Some(8),
            None,
            None,
            Some(6),
            Some(7),
            Some(9),
        ];
        let root = TreeNode::new_tree(&root);
        let output = vec![4, 2, 6, 5, 7, 1, 3, 9, 8];
        assert_eq!(Solution::inorder_traversal(root), output);
    }

    #[test]
    fn test_example_3() {
        let root = [];
        let root = TreeNode::new_tree(&root);
        let output = vec![];
        assert_eq!(Solution::inorder_traversal(root), output);
    }

    #[test]
    fn test_example_4() {
        let root = [Some(1)];
        let root = TreeNode::new_tree(&root);
        let output = vec![1];
        assert_eq!(Solution::inorder_traversal(root), output);
    }
}
