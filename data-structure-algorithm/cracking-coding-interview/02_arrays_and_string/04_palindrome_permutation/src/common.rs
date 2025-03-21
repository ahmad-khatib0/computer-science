pub fn get_char_number(c: char) -> i32 {
    let a = 'a' as i32;
    let z = 'z' as i32;

    let val = c as i32;
    if a <= val && val <= z {
        // Mapping to 0 to 25:
        // To map these numeric values to a range of 0 to 25, we subtract the numeric
        // value of 'a' (97) from the numeric value of the character. For example:
        // 'a' → 97 - 97 = 0, 'b' → 98 - 97 = 1,
        return val - a;
    }

    -1
}

pub fn build_char_frequency_table(phrase: &str) -> Vec<i32> {
    let mut table = vec![0; ('z' as usize) - ('a' as usize) + 1];
    for c in phrase.chars() {
        let x = get_char_number(c);
        if x != -1 {
            table[x as usize] += 1;
        }
    }
    table
}
