fn main() {
    let s = String::from("Hello Warld");
    let s1 = first_word(&s);
    println!("{}", s1);

    let my_string = String::from("hello world");

    // first word works on slices of Strings
    let word = first_word(&my_string[..6]);
    let word = first_word(&my_string[..]);
    let word = first_word(&my_string);

    let my_string_literal = "hello world";
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i , &item) in bytes.iter().enumerate() {
        if item == b' ' {
        return &s[0..i];
        }
    }
    &s[..]
}
