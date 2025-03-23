//
// This wil l be more opti mal i n cases where we don't have a large n umber of
// re peating characters. It wil l avoid us having to create a stri ng that we never use.
// The downside of this is that it causes a second loop through the characters and also
// adds nearly duplicated code.
//

fn count_compression(str: &str) -> usize {
    let mut compressed_length = 0;
    let mut count_consecutive = 0;
    let chars: Vec<char> = str.chars().collect();

    for i in 0..chars.len() {
        count_consecutive += 1;

        // If next character is different than current, append this char to result
        // it means the current sequence of consecutive characters has ended.
        if i + 1 >= chars.len() || chars[i] != chars[i + 1] {
            compressed_length += 1;
            compressed_length += count_consecutive.to_string().len();
            count_consecutive = 0;
        }
    }
    compressed_length
}

pub fn string_compression(str: &str) -> String {
    let final_length = count_compression(str);

    // If the compressed string won't be shorter, return the original string
    if final_length >= str.len() {
        return str.to_string();
    }

    let mut compressed = String::with_capacity(final_length);
    let mut count_consecutive = 0;
    let chars: Vec<char> = str.chars().collect();
    for i in 0..chars.len() {
        count_consecutive += 1;

        // If next character is different than current, append this char to result
        if i + 1 >= chars.len() || chars[i] != chars[i + 1] {
            compressed.push(chars[i]);
            compressed.push_str(&count_consecutive.to_string());
            count_consecutive = 0;
        }
    }

    compressed
}
