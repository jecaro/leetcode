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

fn values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    match root {
        None => vec![],
        Some(rc_node) => {
            let node = rc_node.borrow();
            let mut result = Vec::new();

            result.push(node.val);
            result.append(&mut values(node.left.clone()));
            result.append(&mut values(node.right.clone()));

            result
        }
    }
}

pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut values = values(root);
    values.sort();

    let mut diff = Vec::new();
    for i in 0..values.len() - 1 {
        diff.push(values[i + 1] - values[i]);
    }

    *diff.iter().min().unwrap_or(&i32::max_value())
}

#[cfg(test)]
mod minimum_absolute_difference_in_bst {

    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn example1() {
        let root = Some(Rc::new(RefCell::new(super::TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(super::TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(super::TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(super::TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(super::TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
        })));
        let result = super::get_minimum_difference(root);
        assert_eq!(result, 1);
    }

    #[test]
    fn example2() {
        let root = Some(Rc::new(RefCell::new(super::TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(super::TreeNode {
                val: 0,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(super::TreeNode {
                val: 48,
                left: Some(Rc::new(RefCell::new(super::TreeNode {
                    val: 12,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(super::TreeNode {
                    val: 49,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let result = super::get_minimum_difference(root);
        assert_eq!(result, 1);
    }
}
