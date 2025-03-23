pub fn string_compression_bad(str: &str) -> String {
    let mut compressed = String::with_capacity(str.len());
    let mut count_consecutive = 0;
    let chars: Vec<char> = str.chars().collect();

    for i in 0..chars.len() {
        count_consecutive += 1;

        // If next character is different than current, append this char to result
        // it means the current sequence of consecutive characters has ended.
        if i + 1 >= chars.len() || chars[i] != chars[i + 1] {
            compressed.push(chars[i]);
            compressed.push_str(&count_consecutive.to_string());
            count_consecutive = 0;
        }
    }

    if compressed.len() < str.len() {
        compressed
    } else {
        str.to_string()
    }
}
