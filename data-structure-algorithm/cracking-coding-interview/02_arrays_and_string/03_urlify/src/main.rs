fn replace_spaces(str: &mut [char], true_length: usize) {
    let mut space_count = 0;

    (0..true_length).for_each(|i| {
        if str[i] == ' ' {
            space_count += 1;
        }
    });

    // Calculate the new length of the string after replacement
    let mut index = true_length + space_count * 2;

    // the end of the string is properly terminated with a null character (\0).
    // This is a safety measure to handle cases where the array
    // has more space than needed. (see info.md)
    if true_length < str.len() {
        str[true_length] = '\0';
    }

    for i in (0..true_length).rev() {
        if str[i] == ' ' {
            str[index - 1] = '0';
            str[index - 2] = '2';
            str[index - 3] = '%';
            index -= 3;
        } else {
            str[index - 1] = str[i];
            index -= 1;
        }
    }
}

fn find_last_character(str: &[char]) -> Option<usize> {
    (0..str.len()).rev().find(|&i| str[i] != ' ')
}

fn main() {
    let input = "Mr John Smith    ";
    let mut arr: Vec<char> = input.chars().collect();

    let true_length = match find_last_character(&arr) {
        Some(index) => index + 1,
        None => {
            println!("No valid characters found.");
            return;
        }
    };

    replace_spaces(&mut arr, true_length);
    let result: String = arr.iter().collect();
    println!("{}", result.trim_end_matches("\0"))
}
