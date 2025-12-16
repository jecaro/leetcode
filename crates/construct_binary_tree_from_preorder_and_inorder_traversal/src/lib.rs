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

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut preorder_itr = preorder.into_iter();
    match preorder_itr.next() {
        None => None,
        Some(root_id) => {
            let mut itr = inorder.split(|id| *id == root_id);

            // inorder is: left branch - node - right branch
            let inorder_left = itr.next().unwrap_or_default().to_vec();
            // preorder is: node - left branch - right branch
            // hence we take the exact same number of nodes for preorder
            let preorder_left = preorder_itr.by_ref().take(inorder_left.len()).collect();

            // same for the right
            let inorder_right = itr.next().unwrap_or_default().to_vec();
            let preorder_right = preorder_itr.collect();

            // recursively compute the nodes
            let left = build_tree(preorder_left, inorder_left);
            let right = build_tree(preorder_right, inorder_right);

            Some(Rc::new(RefCell::new(TreeNode {
                val: root_id,
                left: left,
                right: right,
            })))
        }
    }
}

#[cfg(test)]
mod construct_binary_tree_from_preorder_and_inorder_traversal {

    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn example1() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let result = super::build_tree(preorder, inorder);
        assert_eq!(
            result,
            Some(Rc::new(RefCell::new(super::TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(super::TreeNode {
                    val: 9,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(super::TreeNode {
                    val: 20,
                    left: Some(Rc::new(RefCell::new(super::TreeNode {
                        val: 15,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(super::TreeNode {
                        val: 7,
                        left: None,
                        right: None
                    }))),
                }))),
            })))
        );
    }

    #[test]
    fn example2() {
        let preorder = vec![-1];
        let inorder = vec![-1];
        let result = super::build_tree(preorder, inorder);
        assert_eq!(
            result,
            Some(Rc::new(RefCell::new(super::TreeNode {
                val: -1,
                left: None,
                right: None,
            })))
        );
    }

    #[test]
    fn example3() {
        let preorder = vec![1, 2];
        let inorder = vec![1, 2];
        let result = super::build_tree(preorder, inorder);
        assert_eq!(
            result,
            Some(Rc::new(RefCell::new(super::TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(super::TreeNode {
                    val: 2,
                    left: None,
                    right: None
                }))),
            })))
        );
    }
}
