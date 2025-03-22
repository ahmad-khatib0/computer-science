//
//
// One Away: There are three types of edits that can be performed on strings: insert
// a character, remove a character, or replace a character. Given two strings, write
// a function to check if they are one edit (or zero edits) away.
// EXAMPLE
// pale,   ple -> true
// pales,  pale -> true
// pale,   bale -> true
// pale,   bae -> false

mod question1;
mod question2;

fn main() {
    let a = "pse";
    let b = "pale";
    let res = question1::one_edit_away(a, b);

    let a = "palee";
    let b = "pale";
    let is_one_edit1 = question2::one_edit_away(a, b);
    println!("{}, {}: {}", a, b, is_one_edit1); // Output: palee, pale: true

    let c = "pale";
    let d = "pkle";
    let is_one_edit2 = question2::one_edit_away(c, d);
    println!("{}, {}: {}", c, d, is_one_edit2); // Output: pale, pkle: true

    println!("a {}, b {} is_one_edit_away?: {}", a, b, &res);
}

#[cfg(test)]
mod test {
    use super::*;

    fn run_test_case(a: &str, b: &str, expected: bool) {
        let res1 = question1::one_edit_away(a, b);
        let res2 = question2::one_edit_away(a, b);

        assert_eq!(res1, expected, "Failed for one_edit_away_a: {}, {}", a, b);
        assert_eq!(res2, expected, "Failed for one_edit_away_b: {}, {}", a, b);
    }

    #[test]
    fn test_one_edit_await() {
        let test_cases = [
            ("a", "b", true),
            ("", "d", true),
            ("d", "de", true),
            ("pale", "pse", false),
            ("acdsfdsfadsf", "acdsgdsfadsf", true),
            ("acdsfdsfadsf", "acdsfdfadsf", true),
            ("acdsfdsfadsf", "acdsfdsfads", true),
            ("acdsfdsfadsf", "cdsfdsfadsf", true),
            ("adfdsfadsf", "acdfdsfdsf", false),
            ("adfdsfadsf", "bdfdsfadsg", false),
            ("adfdsfadsf", "affdsfads", false),
            ("pale", "pkle", true),
            ("pkle", "pable", false),
        ];

        for (a, b, expected) in test_cases.iter() {
            run_test_case(a, b, *expected);
            run_test_case(b, a, *expected);
        }
    }
}
