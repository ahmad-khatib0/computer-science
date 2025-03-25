//
//
// String Rotation: Assume you have a method isSubstring which checks if one word is a
// substring of another. Given two strings, sl and s2, write code to check if s2 is a rotation
// of sl using only one call to isSubstring (e.g.,"waterbottle" is a rotation of "erbottlewat").
// Hints: \#34, \#88, \#704
//

fn is_substring(big: &str, small: &str) -> bool {
    big.contains(small)
}

fn is_rotation(s1: &str, s2: &str) -> bool {
    let len = s1.len();
    /* check that s1 and s2 are equal length and not empty */
    if len == s2.len() && len > 0 {
        /* concatenate s1 and s1 within new buffer */
        let s1s1 = format!("{}{}", s1, s1);
        return is_substring(&s1s1, s2);
    }
    false
}

fn main() {
    let pairs = [
        ("apple", "pleap"),
        ("waterbottle", "erbottlewat"),
        ("camera", "macera"),
    ];

    for &(word1, word2) in pairs.iter() {
        let is_rotation = is_rotation(word1, word2);
        println!("{}, {}: {}", word1, word2, is_rotation);
    }
}
