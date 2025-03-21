use crate::common::get_char_number;

fn toggle(bit_vector: i32, index: i32) -> i32 {
    // If index is negative, return the bit_vector unchanged (no toggling needed).
    if index < 0 {
        return bit_vector;
    }

    // A mask is created by shifting 1 to the left by index positions. For example:
    // If index = 2, mask becomes 0b100 (binary) or 4 (decimal).
    let mask = 1 << index;
    // The expression bit_vector & mask isolates the bit at the index position:
    // . If the result is 0, the bit was 0.
    // . If the result is non-zero, the bit was 1.
    if (bit_vector & mask) == 0 {
        // Use the bitwise OR (|) operator to set the bit at the index position to 1.
        bit_vector | mask
    } else {
        // o set the bit at the index position to 0
        bit_vector & !mask
    }
}

fn create_bit_vector(str: &str) -> i32 {
    // in hex: 0x00000000,  in bit: 0b00000000000000000000000000000000
    let mut bit_vector = 0;
    for c in str.chars() {
        let x = get_char_number(c);
        bit_vector = toggle(bit_vector, x);
    }
    bit_vector
}

/// verify if the bit_vector has at most one bit set to 1
fn check_at_most_one_bit_set(bit_vector: i32) -> bool {
    // . The expression bit_vector & (bit_vector - 1) clears the lowest set bit in bit_vector.
    // . If the result is 0, it means there was at most one bit set to 1.
    // For example:
    //     If bit_vector = 0b1000, then bit_vector - 1 = 0b0111,
    //     and bit_vector & (bit_vector - 1) = 0b0000.
    //     If bit_vector = 0b1010, then bit_vector - 1 = 0b1001,
    //     and bit_vector & (bit_vector - 1) = 0b1000 (non-zero).
    (bit_vector & (bit_vector - 1)) == 0
}

pub fn is_permutation_of_palindrome(str: &str) -> bool {
    let bit_vector = create_bit_vector(str);
    check_at_most_one_bit_set(bit_vector)
}
