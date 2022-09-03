//! Given a roman numeral, convert it to an integer.

pub fn roman_to_int(s: &str) -> i32 {
    let mut final_int = 0;
    let mut prev_ch = 'a';
    for ch in s.chars() {
        match ch {
            'I' => {
                final_int += 1;
            }
            'V' => {
                if prev_ch == 'I' {
                    final_int += 3;
                } else {
                    final_int += 5;
                }
            }
            'X' => {
                if prev_ch == 'I' {
                    final_int += 8;
                } else {
                    final_int += 10;
                }
            }
            'L' => {
                if prev_ch == 'X' {
                    final_int += 30;
                } else {
                    final_int += 50;
                }
            }
            'C' => {
                if prev_ch == 'X' {
                    final_int += 80;
                } else {
                    final_int += 100;
                }
            }
            'D' => {
                if prev_ch == 'C' {
                    final_int += 300;
                } else {
                    final_int += 500;
                }
            }
            'M' => {
                if prev_ch == 'C' {
                    final_int += 800;
                } else {
                    final_int += 1000;
                }
            }
            _ => {}
        }
        prev_ch = ch;
    }
    final_int
}
