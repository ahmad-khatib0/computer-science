use crate::common;

fn check_max_one_odd(arr: &Vec<i32>) -> bool {
    let mut found_odd = false;
    for a in arr {
        if a % 2 == 1 {
            if found_odd {
                return false;
            }
            found_odd = true;
        }
    }
    true
}

pub fn is_permutation_of_palindrome(phrase: &str) -> bool {
    let table = common::build_char_frequency_table(phrase);
    check_max_one_odd(&table)
}
