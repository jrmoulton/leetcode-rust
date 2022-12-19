pub mod atoi;
pub mod find_pivot_index;
pub mod fizz_buzz;
pub mod is_subsequence;
pub mod isomorphic_strings;
pub mod merge_two_sorted_lists;
pub mod odd_even_jumps;
pub mod palindrome_sub;
pub mod power_of_four;
pub mod power_of_two;
pub mod remove_nth_from_end;
pub mod reverse_wods_in_a_string_iii;
pub mod roman_to_integer;
pub mod rotate_image;
pub mod running_sum_1d_array;
pub mod vertical_order_traversal;

pub mod linked_list {
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        pub fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }

    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct LinkedList {
        pub head: Option<Box<ListNode>>,
    }
    impl LinkedList {
        pub fn new(head: Option<Box<ListNode>>) -> Self {
            Self { head }
        }
    }
    impl From<Vec<i32>> for LinkedList {
        fn from(vec: Vec<i32>) -> Self {
            let mut root = None;
            for val in vec.into_iter().rev() {
                root = Some(Box::new(ListNode { val, next: root }));
            }
            LinkedList::new(root)
        }
    }
}

pub mod reverse_linked_list {
    use crate::linked_list::LinkedList;

    pub fn reverse_list(list: LinkedList) -> LinkedList {
        let head = list.head;
        let mut prev = None;
        let mut curr = head;
        while let Some(mut inner) = curr.take() {
            let temp = inner.next.take();
            inner.next = prev.take();
            prev = Some(inner);
            curr = temp;
        }
        LinkedList::new(prev)
    }
}

#[cfg(test)]
mod tests;
