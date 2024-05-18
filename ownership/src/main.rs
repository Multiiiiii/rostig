fn main() {
    let s = String::from("hello¹");
    println!("{}", s);
    {                      // s is not valid here, it’s not yet declared
        let s = "hello²";   // s is valid from this point forward
        println!("{}", s);
    }                      // this scope is now over, and s is no longer valid
    
    

    let mut s = String::from("hello");
    s.push_str(", world! ☣"); // push_str() appends a literal to a String
    println!{"{}", s};

    {
        let s = String::from("hello³"); // s is valid from this point forward
        println!("{}", s);
     
    } // this scope is now over, and s is no longer valid

    let x = 5; //pushed to stack
    let y = x; //pushed to stack

    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello⁴"); // ptr, len, capacity pushed to stack
    let s2 = s1; // ptr, len, capacity copied to s2 and stored on the stack, s1 is invalidated, so s1 got moved to s2
    // println!("{}, world!", s1); // error: value borrowed here after move
    println!("{}, world!", s2);

    let s1 = String::from("hello⁵");
    let s2 = s1.clone(); // heap data gets copied, s1 is still valid
    println!("s1 = {}, s2 = {}", s1, s2);

    {
    let s = String::from("hello⁶"); // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
    }// Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

    {
        let s1 = gives_ownership();         // gives_ownership moves its return value into s1

        let s2 = String::from("hello⁷"); // s2 comes into scope

        let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s3
        println!{"{} {}", s1, s3};
    } // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was moved, so nothing happens. s1 goes out of scope and is dropped.

    let s1 = String::from("hello⁸");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

    let s1 = String::from("hello⁹");
    let len = calculate_length_with_pointer(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello¹⁰");

    change(&mut s);

    println!("{}",s);

    let s = String::from("hello¹¹");
    let len = calculate_length_with_pointer(&s);
    println!("The length of '{}' is {}.", s, len);

    let mut s = String::from("hello¹²");

    {
        let r1 = &mut s;
        change(r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;
    change(r2);
    println!("{}", s);

    let mut s = String::from("hello¹³");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2); //variables r1 and r2 will not be used after this point
    let r3 = &mut s; //no problem
    change(r3);
    println!("{}", r3);

    let something = no_dangle();
    println!("{}", something);
   
    
} 

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn gives_ownership() -> String {             // gives_ownership will move its return value into the function that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); //len() returns the length of a String
    (s, length)
}

fn calculate_length_with_pointer(s: &String) -> usize { //s is a reference to a String
    s.len() 
} // Here, s goes out of scrope. But because it does not have ownership of what it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(",world☣");
}

fn no_dangle() -> String { 
    let s = String::from("hello¹⁴");
    s
}