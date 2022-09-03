//! Given a string s, return the longest palindromic substring in s.

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
struct Palindrome {
    /// start of repeated chars
    start_idx: usize,
    /// end of repea chars
    end_idx: usize,
    /// num of chars in palindrome after end_idx
    length: usize,
    /// length of the String
    str_len: usize,
}
impl PartialOrd for Palindrome {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            ((self.end_idx + self.length) - (self.start_idx - self.length))
                .cmp(&((other.end_idx + other.length) - (other.start_idx - other.length))),
        )
    }
}
impl Palindrome {
    fn new(str_len: usize) -> Self {
        Self {
            str_len,
            ..Default::default()
        }
    }
    fn next(&mut self) {
        self.end_idx += 1;
        self.start_idx = self.end_idx;
        self.length = 0;
    }

    fn at_final_pos(&self) -> bool {
        self.start_idx + self.length >= self.str_len - 1
    }

    fn end_in_bounds(&self) -> bool {
        self.end_idx < self.str_len - 1
    }

    fn length_in_bounds(&self) -> bool {
        self.length < self.start_idx && self.end_idx + self.length < self.str_len - 1
    }

    fn next_value_is_same(&self, s: &[u8]) -> bool {
        s[self.start_idx] == s[self.end_idx + 1]
    }

    fn next_outer_values_are_eq(&self, s: &[u8]) -> bool {
        s[self.start_idx - self.length - 1] == s[self.end_idx + self.length + 1]
    }

    fn to_string(self, s: &[u8]) -> String {
        String::from_utf8(s[self.start_idx - self.length..=self.end_idx + self.length].to_owned())
            .unwrap()
    }
}

/// Returns the longest palindrome subsequence in the string
pub fn longest_palindrome(s: &str) -> String {
    let s = s.as_bytes();
    let mut final_palindrome = Palindrome::default();
    let mut curr_palindrome = Palindrome::new(s.len());
    while !curr_palindrome.at_final_pos() {
        while curr_palindrome.end_in_bounds() && curr_palindrome.next_value_is_same(s) {
            curr_palindrome.end_idx += 1;
        }
        while curr_palindrome.length_in_bounds() && curr_palindrome.next_outer_values_are_eq(s) {
            curr_palindrome.length += 1;
        }
        if curr_palindrome > final_palindrome {
            final_palindrome = curr_palindrome;
        }
        curr_palindrome.next();
    }
    final_palindrome.to_string(s)
}
