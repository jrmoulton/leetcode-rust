//! Given the head of a linked list, remove the nth node from the end of the list and return its head.
//!
//! Constraints:
//! The number of nodes in the list is sz.
//! 1 <= sz <= 30
//! 0 <= Node.val <= 100
//! 1 <= n <= sz
//!
//! Follow up: Could you do this in one pass?

#![allow(dead_code)]

type Node = Option<Box<ListNode>>;
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Node,
}
impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Cursor {
    curr: Node,
}
impl Cursor {
    fn new(node: Node) -> Self {
        Self { curr: node }
    }
    // pub fn pop(&mut self) {
    //     match self.curr {
    //         None => {}
    //         Some(inner) => {
    //             let mut n = Some(Box::new(ListNode::new(5)));
    //             let item = inner.val;
    //             std::mem::swap(&mut inner.next, &mut n);
    //             self.to_next(n);
    //         }
    //     }
    // }
    fn to_next(&mut self, next: Node) {
        *self = Cursor::new(next);
    }
}

pub fn remove_nth_from_end(head: Node, _n: i32) -> Node {
    head
}
