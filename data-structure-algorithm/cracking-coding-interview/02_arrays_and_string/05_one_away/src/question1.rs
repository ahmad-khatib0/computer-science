// This is one of those problems where it's helpful to think about the "meaning" of each of
// these operations. What does it mean for two strings to be one insertion, replacement,
// or removal away from each other?
//
// . Replacement: Consider two strings, such as bale and pale, that are one replacement
//   away. Yes, that does mean that you could replace a character in bale to make pale.
//   But more precisely, it means that they are different only in one place.
// . Insertion: The strings apple and aple are one insertion away. This means that if you
//   compared the strings, they would be identical-except for a shift at some point in
//   the strings.
// Removal: The strings apple and aple are also one removal away, since removal is just
//   the inverse of insertion.
//

fn one_edit_replace(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut found_diff = false;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            if found_diff {
                return false;
            }
            found_diff = true;
        }
    }
    true
}

fn one_edit_insert(s1: &str, s2: &str) -> bool {
    let mut idx1 = 0;
    let mut idx2 = 0;

    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    while idx2 < s2.len() && idx1 < s1.len() {
        if s1_chars[idx1] != s2_chars[idx2] {
            if idx1 != idx2 {
                return false;
            }

            idx2 += 1;
        } else {
            idx1 += 1;
            idx2 += 1;
        }
    }

    true
}

pub fn one_edit_away(s1: &str, s2: &str) -> bool {
    if s1.len() == s2.len() {
        return one_edit_replace(s1, s2); // replace
    } else if s1.len() + 1 == s2.len() {
        return one_edit_insert(s1, s2); // insert
    } else if s1.len() - 1 == s2.len() {
        return one_edit_insert(s2, s1); // remove
    }
    false
}
