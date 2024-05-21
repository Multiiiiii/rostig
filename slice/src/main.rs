fn main() {
    let string = String::from("hello ☣☣ erne");
    let word = fist_word(&string);
    println!("{}", word);
}

fn fist_word(s: &String) -> usize {
    let bytes =s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}