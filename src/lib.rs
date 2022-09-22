pub mod atoi;
pub mod fizz_buzz;
pub mod palindrome_sub;
pub mod power_of_four;
pub mod power_of_two;
pub mod remove_nth_from_end;
pub mod roman_to_integer;
pub mod rotate_image;
pub mod vertical_order_traversal;

mod tests;

mod reverse_wods_in_a_string_iii {
    //! Given a string s, reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.
    //!
    //! Example 1:
    //! Input: s = "Let's take LeetCode contest"
    //! Output: "s'teL ekat edoCteeL tsetnoc"
    //!
    //! Example 2:
    //! Input: s = "God Ding"
    //! Output: "doG gniD"
    //!
    //! Constraints:
    //! 1 <= s.length <= 5 * 104
    //! s contains printable ASCII characters.
    //! s does not contain any leading or trailing spaces.
    //! There is at least one word in s.
    //! All the words in s are separated by a single space.

    /// reverse each individual word
    ///
    /// Process:
    /// split string on white space -> vec<String>.
    /// for string in vec<string>:
    ///     reverse string in place sort of (see about this)
    ///     add white space back
    ///     concat vec<string> -> String
    ///     trim last whitespace
    ///     return string
    pub fn reverse_words(s: String) -> String {
        let ret_str: String = s
            .split_whitespace()
            .map(|sub_str| {
                let mut s = sub_str.to_string().chars().rev().collect::<String>();
                s.push(' ');
                s
            })
            .collect::<String>()
            .trim()
            .to_string();
        ret_str
    }
}
