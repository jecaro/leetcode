use std::cell::RefCell;
use std::rc::Rc;

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
}

pub fn right_side_view(op_root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    match op_root {
        None => vec![],
        Some(root) => {
            let mut result = Vec::new();

            let mut stack = Vec::new();
            stack.push(root);

            while !stack.is_empty() {
                let current_level: Vec<_> = stack.drain(..).collect();

                for (i, node) in current_level.iter().enumerate() {
                    // The first one is the right most node
                    if i == 0 {
                        result.push(node.borrow().val);
                    }
                    // Push first the right node, it'll be popped first
                    if let Some(right) = node.borrow().right.clone() {
                        stack.push(right);
                    }
                    // Fallback on the left node, if there
                    if let Some(left) = node.borrow().left.clone() {
                        stack.push(left);
                    }
                }
            }

            result
        }
    }
}

#[cfg(test)]
mod binary_tree_right_side_view {

    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn example1() {
        let root = Some(Rc::new(RefCell::new(super::TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(super::TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(super::TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(super::TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(super::TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let result = super::right_side_view(root);
        assert_eq!(result, vec![1, 3, 4]);
    }

    #[test]
    fn example2() {
        let root = Some(Rc::new(RefCell::new(super::TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(super::TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(super::TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(super::TreeNode {
                        val: 5,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(super::TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        let result = super::right_side_view(root);
        assert_eq!(result, vec![1, 3, 4, 5]);
    }

    #[test]
    fn example3() {
        let root = Some(Rc::new(RefCell::new(super::TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(super::TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        let result = super::right_side_view(root);
        assert_eq!(result, vec![1, 3]);
    }

    #[test]
    fn example4() {
        let root = None;
        let result = super::right_side_view(root);
        assert_eq!(result, vec![]);
    }
}
