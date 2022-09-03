//! Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer (similar to C/C++'s atoi function).
pub fn my_atoi(s: &str) -> i32 {
    let s = s.as_bytes();
    let mut final_chars = vec![];
    for ch in s {
        let adj = ch - 0x30;
        if adj < 10 {
            final_chars.push(adj);
        }
    }
    5
}
