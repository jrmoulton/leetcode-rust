#![cfg(test)]

use super::*;
use pretty_assertions::assert_eq;

#[test]
#[cfg(feature = "active")]
fn test_reverse_wods_in_a_string_iii() {
    use reverse_wods_in_a_string_iii::reverse_words;
    assert_eq!(
        reverse_words("Let's take LeetCode contest".into()),
        "s'teL ekat edoCteeL tsetnoc".to_string()
    );
    assert_eq!(reverse_words("God Ding".into()), "doG gniD".to_string());
}

#[test]
#[cfg(feature = "broken")]
fn test_vertical_traversal() {
    use vertical_order_traversal::TreeNode;

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    })));
    assert_eq!(
        vertical_order_traversal::vertical_traversal(root),
        vec![vec![9], vec![3, 15], vec![20], vec![7]]
    );
}

#[test]
#[cfg(feature = "complete")]
fn test_fizz_buzz() {
    assert_eq!(
        fizz_buzz::fizz_buzz(15),
        vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz"
        ]
        .iter()
        .map(|val| String::from(*val))
        .collect::<Vec<String>>()
    )
}

#[test]
#[cfg(feature = "complete")]
fn test_rotate_image() {
    let mut a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    rotate_image::rotate(&mut a);
    assert_eq!(a, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);

    let mut b = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];
    rotate_image::rotate(&mut b);
    assert_eq!(
        b,
        vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11]
        ]
    );
}

#[test]
#[cfg(feature = "complete")]
fn test_power_of_two() {
    assert!(power_of_two::is_power_of_two(2));
    assert!(power_of_two::is_power_of_two(16));
    assert!(!power_of_two::is_power_of_two(3));
}

#[test]
#[cfg(feature = "complete")]
fn test_power_of_four() {
    assert!(power_of_four::is_power_of_four(16));
    assert!(!power_of_four::is_power_of_four(5));
    assert!(power_of_four::is_power_of_four(1));
    assert!(!power_of_four::is_power_of_four(-2147483647));
}

#[test]
#[cfg(feature = "broken")]
fn test_remove_nth_from_end() {
    use remove_nth_from_end::ListNode;
    let head_1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));
    let sol_1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 5, next: None })),
            })),
        })),
    }));
    assert_eq!(remove_nth_from_end::remove_nth_from_end(head_1, 2), sol_1);
}

#[test]
#[cfg(feature = "complete")]
fn test_longest_palindrome() {
    assert_eq!(
        palindrome_sub::longest_palindrome("babad"),
        "bab".to_string()
    );
    assert_eq!(palindrome_sub::longest_palindrome("cbbd"), "bb".to_string());
    assert_eq!(palindrome_sub::longest_palindrome("azwdzwmwcqzgcobeeiphemqbjtxzwkhiqpbrprocbppbxrnsxnwgikiaqutwpftbiinlnpyqstkiqzbggcsdzzjbrkfmhgtnbujzszxsycmvipjtktpebaafycngqasbbhxaeawwmkjcziybxowkaibqnndcjbsoehtamhspnidjylyisiaewmypfyiqtwlmejkpzlieolfdjnxntonnzfgcqlcfpoxcwqctalwrgwhvqvtrpwemxhirpgizjffqgntsmvzldpjfijdncexbwtxnmbnoykxshkqbounzrewkpqjxocvaufnhunsmsazgibxedtopnccriwcfzeomsrrangufkjfzipkmwfbmkarnyyrgdsooosgqlkzvorrrsaveuoxjeajvbdpgxlcrtqomliphnlehgrzgwujogxteyulphhuhwyoyvcxqatfkboahfqhjgujcaapoyqtsdqfwnijlkknuralezqmcryvkankszmzpgqutojoyzsnyfwsyeqqzrlhzbc"), "sooos".to_string());
    assert_eq!(palindrome_sub::longest_palindrome("ac"), "a".to_string());
    assert_eq!(palindrome_sub::longest_palindrome("bb"), "bb".to_string());
    assert_eq!(palindrome_sub::longest_palindrome("abb"), "bb".to_string());
}

#[test]
#[cfg(feature = "complete")]
fn test_roman_to_int() {
    assert_eq!(roman_to_integer::roman_to_int("III"), 3);
    assert_eq!(roman_to_integer::roman_to_int("LVIII"), 58);
    assert_eq!(roman_to_integer::roman_to_int("IV"), 4);
    assert_eq!(roman_to_integer::roman_to_int("MCMXCIV"), 1994);
}

#[test]
#[cfg(feature = "broken")]
fn test_my_atoi() {
    assert_eq!(atoi::my_atoi("42"), 42);
    assert_eq!(atoi::my_atoi("   -42"), -42);
    assert_eq!(atoi::my_atoi("4193 with words"), 4193);
}
