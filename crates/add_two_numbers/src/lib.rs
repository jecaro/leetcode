#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers_rec(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    remainder: i32,
) -> Option<Box<ListNode>> {
    match (l1, l2, remainder) {
        (None, None, 0) => None,
        (None, None, _) => Some(Box::new(ListNode::new(remainder))),
        (Some(node), None, _) | (None, Some(node), _) => {
            let total = node.val + remainder;
            let units = total % 10;
            let remainder = total / 10;

            Some(Box::new(ListNode {
                val: units,
                next: add_two_numbers_rec(node.next, None, remainder),
            }))
        }
        (Some(node1), Some(node2), _) => {
            let total = node1.val + node2.val + remainder;
            let units = total % 10;
            let remainder = total / 10;

            Some(Box::new(ListNode {
                val: units,
                next: add_two_numbers_rec(node1.next, node2.next, remainder),
            }))
        }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    add_two_numbers_rec(l1, l2, 0)
}

#[cfg(test)]
mod add_two_numbers {

    #[test]
    fn example1() {
        let l1 = Some(Box::new(super::ListNode {
            val: 2,
            next: Some(Box::new(super::ListNode {
                val: 4,
                next: Some(Box::new(super::ListNode { val: 3, next: None })),
            })),
        }));
        let l2 = Some(Box::new(super::ListNode {
            val: 5,
            next: Some(Box::new(super::ListNode {
                val: 6,
                next: Some(Box::new(super::ListNode { val: 4, next: None })),
            })),
        }));
        let result = super::add_two_numbers(l1, l2);
        assert_eq!(
            result,
            Some(Box::new(super::ListNode {
                val: 7,
                next: Some(Box::new(super::ListNode {
                    val: 0,
                    next: Some(Box::new(super::ListNode { val: 8, next: None })),
                })),
            }))
        );
    }

    #[test]
    fn example2() {
        let l1 = Some(Box::new(super::ListNode { val: 0, next: None }));
        let l2 = Some(Box::new(super::ListNode { val: 0, next: None }));
        let result = super::add_two_numbers(l1, l2);
        assert_eq!(
            result,
            Some(Box::new(super::ListNode { val: 0, next: None }))
        );
    }

    #[test]
    fn example3() {
        let l1 = Some(Box::new(super::ListNode {
            val: 9,
            next: Some(Box::new(super::ListNode {
                val: 9,
                next: Some(Box::new(super::ListNode {
                    val: 9,
                    next: Some(Box::new(super::ListNode {
                        val: 9,
                        next: Some(Box::new(super::ListNode {
                            val: 9,
                            next: Some(Box::new(super::ListNode {
                                val: 9,
                                next: Some(Box::new(super::ListNode { val: 9, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        let l2 = Some(Box::new(super::ListNode {
            val: 9,
            next: Some(Box::new(super::ListNode {
                val: 9,
                next: Some(Box::new(super::ListNode {
                    val: 9,
                    next: Some(Box::new(super::ListNode { val: 9, next: None })),
                })),
            })),
        }));
        let result = super::add_two_numbers(l1, l2);
        assert_eq!(
            result,
            Some(Box::new(super::ListNode {
                val: 8,
                next: Some(Box::new(super::ListNode {
                    val: 9,
                    next: Some(Box::new(super::ListNode {
                        val: 9,
                        next: Some(Box::new(super::ListNode {
                            val: 9,
                            next: Some(Box::new(super::ListNode {
                                val: 0,
                                next: Some(Box::new(super::ListNode {
                                    val: 0,
                                    next: Some(Box::new(super::ListNode {
                                        val: 0,
                                        next: Some(Box::new(super::ListNode {
                                            val: 1,
                                            next: None,
                                        })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            }))
        );
    }
}
