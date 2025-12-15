#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1.clone(), list2.clone()) {
        (None, None) => None,
        (Some(_), None) => list1,
        (None, Some(_)) => list2,
        (Some(node1), Some(node2)) => {
            if node2.val < node1.val {
                let node1_node2 = ListNode {
                    val: node2.val,
                    next: Some(node1),
                };
                merge_two_lists(Some(Box::new(node1_node2)), node2.next)
            } else {
                let node1_next_node2 = merge_two_lists(node1.next, Some(node2));
                let node1 = ListNode {
                    val: node1.val,
                    next: node1_next_node2,
                };
                Some(Box::new(node1))
            }
        }
    }
}

#[cfg(test)]
mod merge_two_sorted_lists {

    #[test]
    fn example1() {
        let list1 = Some(Box::new(super::ListNode {
            val: 1,
            next: Some(Box::new(super::ListNode {
                val: 2,
                next: Some(Box::new(super::ListNode { val: 4, next: None })),
            })),
        }));
        let list2 = Some(Box::new(super::ListNode {
            val: 1,
            next: Some(Box::new(super::ListNode {
                val: 3,
                next: Some(Box::new(super::ListNode { val: 4, next: None })),
            })),
        }));
        let result = super::merge_two_lists(list1, list2);
        assert_eq!(
            result,
            Some(Box::new(super::ListNode {
                val: 1,
                next: Some(Box::new(super::ListNode {
                    val: 1,
                    next: Some(Box::new(super::ListNode {
                        val: 2,
                        next: Some(Box::new(super::ListNode {
                            val: 3,
                            next: Some(Box::new(super::ListNode {
                                val: 4,
                                next: Some(Box::new(super::ListNode { val: 4, next: None })),
                            })),
                        })),
                    })),
                })),
            }))
        );
    }

    #[test]
    fn example2() {
        let list1 = None;
        let list2 = None;
        let result = super::merge_two_lists(list1, list2);
        assert_eq!(result, None);
    }

    #[test]
    fn example3() {
        let list1 = None;
        let list2 = Some(Box::new(super::ListNode { val: 0, next: None }));
        let result = super::merge_two_lists(list1, list2);
        assert_eq!(
            result,
            Some(Box::new(super::ListNode { val: 0, next: None }))
        );
    }
}
