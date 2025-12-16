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

pub fn average_of_levels(op_root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
    match op_root {
        None => vec![],
        Some(root) => {
            let mut result = Vec::new();

            let mut stack = Vec::new();
            stack.push(root);

            while !stack.is_empty() {
                let current_level: Vec<Rc<RefCell<TreeNode>>> = stack.drain(..).collect();
                let nb_nodes = current_level.len();
                let mut total = 0.;
                for op_node in current_level {
                    let node = op_node.borrow();

                    if let Some(left_node) = node.left.clone() {
                        stack.push(left_node);
                    }
                    if let Some(right_node) = node.right.clone() {
                        stack.push(right_node);
                    }

                    total += node.val as f64;
                }

                result.push(total / nb_nodes as f64);
            }
            result
        }
    }
}

#[cfg(test)]
mod average_of_levels_in_binary_tree {

    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::average_of_levels;

    #[test]
    fn example1() {
        let root = Some(Rc::new(RefCell::new(super::TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(super::TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(super::TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(super::TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(super::TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let result = average_of_levels(root);
        assert_eq!(result, vec![3.00000, 14.50000, 11.00000]);
    }

    #[test]
    fn example2() {
        let root = Some(Rc::new(RefCell::new(super::TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(super::TreeNode {
                val: 9,
                left: Some(Rc::new(RefCell::new(super::TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(super::TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(super::TreeNode {
                val: 20,
                left: None,
                right: None,
            }))),
        })));
        let result = average_of_levels(root);
        assert_eq!(result, vec![3.00000, 14.50000, 11.00000]);
    }
}
