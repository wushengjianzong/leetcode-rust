#![allow(dead_code, unused_imports)]

struct Solution;

// Definition for singly-linked list.
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

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut curr: &mut Option<Box<ListNode>> = &mut head;

        let mut state = (l1, l2, 0, 0);
        loop {
            state = match state {
                (None, None, _, carry) => {
                    if carry == 0 {
                        break;
                    } else {
                        (None, None, carry, 0)
                    }
                },
                (Some(node), None, _, carry) | (None, Some(node), _, carry) => {
                    let sum: i32 = node.val + carry;
                    (node.next, None, sum % 10, sum / 10)
                },
                (Some(l1), Some(l2), _, carry) => {
                    let sum: i32 = l1.val + l2.val + carry;
                    (l1.next, l2.next, sum % 10, sum / 10)
                }
            };

            *curr = Some(Box::new(ListNode::new(state.2)));
            curr = &mut curr.as_mut().unwrap().next;
        }
        return head;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let l1: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: None
                }))
            })),
        }));
        let l2: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: None
                }))
            })),
        }));
        let mut curr = Solution::add_two_numbers(l1, l2);
        while let Some(_node) = curr {
            print!("{}", _node.val);
            curr = _node.next;
        }
    }
}
