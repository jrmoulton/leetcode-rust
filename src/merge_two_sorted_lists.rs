//! You are given the heads of two sorted linked lists list1 and list2.
//!
//! Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.
//!
//! Return the head of the merged linked list.
//!
//! Example 1:
//! Input: list1 = \[1,2,4\], list2 = \[1,3,4\]
//! Output: \[1,1,2,3,4,4\]
//!
//! Example 2:
//! Input: list1 = \[\], list2 = \[\]
//! Output: \[\]
//!
//! Example 3:
//! Input: list1 = \[\], list2 = \[0\]
//! Output: \[0\]
//!
//! Constraints:
//! The number of nodes in both lists is in the range \[0, 50\].
//! -100 <= Node.val <= 100
//! Both list1 and list2 are sorted in non-decreasing order.

/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
fn push(head: Box<ListNode>, new_node: Box<ListNode>) {
    let mut temp = head;
    while let Some(next) = temp.next {
        temp = next;
    }
    temp.next = Some(new_node);
}

/// Merges two sorted lists into a single sorted list
// Have a return list. Maintian list1 and list2 as pointers to the pos in each list.
// while both not None:
//      compare pointers. Add the smaller. Move poionter
#[allow(unreachable_code, unused_variables)]
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    unimplemented!();
    let base = if list1.is_some() {
        list1
    } else if list2.is_some() {
        list2
    } else {
        return None;
    };
    let list1_head = &list1;
    let list2_head = &list2;
    match (list1_head, list2_head) {
        (None, None) => {
            return list1;
        }
        (None, Some(inner_list2)) => {
            push(
                base.expect("base case already handled"),
                inner_list2.clone(),
            );
        }
        (Some(inner_list1), None) => {
            push(
                base.expect("base case already handled"),
                inner_list1.clone(),
            );
        }
        (Some(_), Some(_)) => todo!(),
    };
    todo!()
}
