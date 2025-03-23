//
//
// 1.6 String Compression: Implement a method to perform basic string compression
// using the counts of repeated characters. For example, the string aabcccccaaa would
// become a2blc5a3. If the "compressed" string would not become smaller than the original
// string, your method should return the original string. You can assume the string has
// only uppercase and lowercase letters (a - z).
// Hints #92, #110
//

mod question1;
mod question2;

fn main() {
    let mut str = "aaabbbccca";
    println!(
        "compressed version of {} is: {}",
        &str,
        question1::string_compression_bad(str)
    );

    str = "abc";
    println!(
        "compressed version of {} is: {}",
        &str,
        question2::string_compression(str)
    );
}
