pub fn main_logic() {
    let s = String::from("hellow world");

    //let word =

    let word = first_word(&s);
    println!("the fist word is : {word}")
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
