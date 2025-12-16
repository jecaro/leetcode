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

pub fn max_depth_rec(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(node_rc) => {
            let node = node_rc.borrow();

            1 + std::cmp::max(max_depth_rec(&node.left), max_depth_rec(&node.right))
        }
    }
}

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    max_depth_rec(&root)
}

#[cfg(test)]
mod maximum_depth_of_binary_tree {

    use std::cell::RefCell;
    use std::rc::Rc;

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
        let result = super::max_depth(root);
        assert_eq!(result, 3);
    }

    #[test]
    fn example2() {
        let root = Some(Rc::new(RefCell::new(super::TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(super::TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        })));
        let result = super::max_depth(root);
        assert_eq!(result, 2);
    }
}
