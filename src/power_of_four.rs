//! Given an integer n, return true if it is a power of four. Otherwise, return false.
//! An integer n is a power of four, if there exists an integer x such that n == 4^x.
//!
//! Example 1:
//! Input: n = 16
//! Output: true
//!
//! Example 2:
//! Input: n = 5
//! Output: false
//!
//! Example 3:
//! Input: n = 1
//! Output: true
//!
//! Constraints:
//! -2^31 <= n <= 2^31 - 1
//!
//! Follow up: Could you solve it without loops/recursion?

pub fn is_power_of_four(n: i32) -> bool {
    let mask = 0b1010101010101010101010101010101;
    (n & mask).count_ones() == 1 && (n & !mask).count_ones() == 0
}
pub fn with_help_is_power_of_four(n: i32) -> bool {
    (n > 0) && (n & 0b1010101010101010101010101010101 == n) && (n & (n - 1) == 0)
}
