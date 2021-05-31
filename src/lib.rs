pub fn fizzbuzzify(input_string: &str) -> String {
    let words: Vec<&str> = input_string.split_whitespace().collect();
    let mut fizzbuzzified_words: Vec<&str> = Vec::new();

    for (i, word) in words.iter().enumerate() {
        fizzbuzzified_words.push(fizzbuzzify_word(i, word));
    }

    return fizzbuzzified_words.join(" ");
}

fn fizzbuzzify_word(i: usize, word: &str) -> &str {

    if (i+1) % 15 == 0 {
        return "fizzbuzz"
    }
    if (i+1) % 3 == 0 {
        return "fizz"
    }
    if (i+1) % 5 == 0 {
        return "buzz"
    }

    word
}


#[test]
pub fn run_tests() {
    println!("Running fizzbuzzify tests!");

    assert_eq!(fizzbuzzify(""), "");
    assert_eq!(fizzbuzzify("1"), "1");
    assert_eq!(fizzbuzzify("1 2 3"), "1 2 fizz");
    assert_eq!(fizzbuzzify("1 2 3 4 5"), "1 2 fizz 4 buzz");
    assert_eq!(fizzbuzzify("1 2 3 4 5 6 7 8 9 10 11 12 13 14 15"),
               "1 2 fizz 4 buzz fizz 7 8 fizz buzz 11 fizz 13 14 fizzbuzz");
}