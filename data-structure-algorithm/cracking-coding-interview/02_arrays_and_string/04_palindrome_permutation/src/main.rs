mod common;
mod question1;
mod question2;
mod question3;

fn main() {
    let mut pali = String::from("Rats live on no evil star");

    let mut res = question1::is_permutation_of_palindrome(&pali);
    println!("the phrase: {}, is palindrome? {}", pali, res);

    pali = String::from("Ratzs live on no evil starz");
    res = question2::is_permutation_of_palindrome(&pali);
    println!("the phrase: {}, is palindrome? {}", pali, res);

    pali = String::from("Zeus was deified, saw Suez");
    res = question2::is_permutation_of_palindrome(&pali);
    println!("the phrase: {}, is palindrome? {}", pali, res);

    res = question3::is_permutation_of_palindrome(&pali);
    println!("the phrase: {}, is palindrome? {}", pali, res);
}
