use crate::common;

pub fn is_permutation_of_palindrome(str: &str) -> bool {
    let mut count_odd = 0;
    let mut table = vec![0; ('z' as usize) - ('a' as usize) + 1];
    for c in str.chars() {
        let x = common::get_char_number(c);
        if x != -1 {
            table[x as usize] += 1;
            if table[x as usize] % 2 == 1 {
                count_odd += 1;
            } else {
                count_odd -= 1;
            }
        }
    }

    count_odd <= 1
}
