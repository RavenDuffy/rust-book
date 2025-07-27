fn main() {
    string_properties();

    let string_to_pass = String::from("multiple words");
    let first = first_word(&string_to_pass);
    println!("{first}");
}

fn string_properties() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    let world2 = &s[6..=10];

    println!("{hello}{world}{world2}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
