fn main() {
    let mut s = String::from("hello world¹");
    let word = fist_word(&s);
    s.clear(); //this empties the String, makeing it equal to ""
            //word still has the value 5 here, but ther's no more string that 
            //we could meaningfully use the value 5 with. word is now totally invalid!
    println!("{}", word);
    
    let s = String::from("hello world²");

    let hello = &s[0..5];
    let world = &s[6..];
    
    println!("{},{}",hello, world);

    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2]; //equal to the one before
    

    println!("{}1",slice);

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..]; //equal to the one before 

    println!("{}2",slice);

    let slice = &s[0..len];
    let slice = &s[..]; //equal to the one before
    
    println!("{}3",slice);

    let word = fist_word_better(&s);
    
    println!("{}4", word);

    let my_string = String::from("hello world");

    // `first_word_even_better` works on slices of `String`s, whether partial or whole
    let word = first_word_even_better(&my_string[0..6]);
    println!("{}5", word);
    let word = first_word_even_better(&my_string[..]);
    println!("{}6", word);
    // `first_word_even_better` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word_even_better(&my_string);
    println!("{}7", word);

    let my_string_literal = "hello world";

    // `first_word_even_better` works on slices of string literals, whether partial or whole
    let word = first_word_even_better(&my_string_literal[0..6]);
    println!("{}8", word);
    let word = first_word_even_better(&my_string_literal[..]);
    println!("{}9", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_even_better(my_string_literal);
    println!("{}10", word);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
    
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

fn fist_word_better(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
        }
    }
    
    &s[..]
}

fn first_word_even_better(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
        }
    }
    
    &s[..]
}
