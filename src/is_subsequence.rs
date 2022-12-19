//! Given two strings s and t, return true if s is a subsequence of t, or false otherwise.
//!
//! A subsequence of a string is a new string that is formed from the original string by
//! deleting some (can be none) of the characters without disturbing the relative positions of
//! the remaining characters. (i.e., "ace" is a subsequence of "abcde" while "aec" is not).
//!
//! Constraints:
//! 0 <= s.length <= 100
//! 0 <= t.length <= 104
//! s and t consist only of lowercase English letters.
//!
//! Follow up: Suppose there are lots of incoming s, say s1, s2, ..., sk where k >= 109, and you
//! want to check one by one to see if t has its subsequence. In this scenario, how would you
//! change your code?

/// Check if s is a subsequence of t
pub fn is_subsequence(s: String, t: String) -> bool {
    let t_bytes = t.as_bytes();
    let mut idx = 0;
    for ch in s.chars() {
        while idx < t.len() && t_bytes[idx] as char != ch {
            idx += 1;
        }
        if idx >= t.len() {
            return false;
        }
        if t_bytes[idx] as char == ch {
            idx += 1;
        }
    }
    true
}
