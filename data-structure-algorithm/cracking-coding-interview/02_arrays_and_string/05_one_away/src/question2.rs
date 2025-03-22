pub fn one_edit_away(first: &str, second: &str) -> bool {
    // Length checks
    if (first.len() as i32 - second.len() as i32).abs() > 1 {
        return false;
    }

    // Get shorter and longer string
    let (s1, s2) = if first.len() < second.len() {
        (first, second)
    } else {
        (second, first)
    };

    let mut index1 = 0;
    let mut index2 = 0;
    let mut found_difference = false;
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();

    while index2 < s2.len() && index1 < s1.len() {
        if chars1[index1] != chars2[index2] {
            // Ensure this is the first difference found
            if found_difference {
                return false;
            }

            found_difference = true;

            // On replace, move the shorter pointer
            if s1.len() == s2.len() {
                index1 += 1;
            }
        } else {
            // If matching, move the shorter pointer
            index1 += 1;
        }
        // Always move the pointer for the longer string
        index2 += 1;
    }

    true
}
