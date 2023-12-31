fn main() {
    let str = String::from("Hello world");
    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&str[0..6]);
    let word = first_word(&str[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&str);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    // Slices also work on other types of collections than String
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    // &String is a normal pointer, &str is a fat pointer
    println!(
        "&String={} &str={}",
        std::mem::size_of::<&String>(),
        std::mem::size_of::<&str>(),
    );
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    // Equivalent to &s[0..s.len()];
    &s[..]
}