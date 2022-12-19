//! Given two strings s and t, determine if they are isomorphic.
//!
//! Two strings s and t are isomorphic if the characters in s can be replaced to get t.
//! All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character, but a character may map to itself.
//!
//! Example 1:
//! Input: s = "egg", t = "add"
//! Output: true
//!
//! Example 2:
//! Input: s = "foo", t = "bar"
//! Output: false
//!
//! Example 3:
//! Input: s = "paper", t = "title"
//! Output: true
//!
//! Constraints:
//! 1 <= s.length <= 5 * 104
//! t.length == s.length
//! s and t consist of any valid ascii character.

use std::collections::HashMap;

/// Check if a characters can be mapped to other characters to make other string
pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut map = HashMap::new();
    for (s_ch, t_ch) in s.chars().zip(t.chars()) {
        match map.get(&s_ch) {
            None => {
                // if t_ch already been stored and it didn't get matched with s_ch
                if map.values().any(|value| *value == t_ch) {
                    return false;
                } else {
                    map.insert(s_ch, t_ch);
                }
            }
            Some(val) => {
                if *val != t_ch {
                    return false;
                }
            }
        }
    }
    true
}
