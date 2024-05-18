use std::result;

fn main() {
    println!("Hello, world!");

    another_function();
    another_another_function(5);
    print_labeled_measurment(5, 'h');
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let ℕ = plus_one(5);
    println!("The value of ℕ is: {ℕ}");

    // control flow
    let number = 8;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    if number != 0 {
        println!("number was something other than zero");
    }
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };


    println!("The result is: {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count is: {count}");
        let mut remaining = 10; 
        loop {
            println!("remaining is: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    println!("End count {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("☣the value is: {}",a[index]);

        index += 1;
    }
    // now the better version
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}

fn another_function() {
    println!("Another function.");
}

fn another_another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurment(value: i32, unit_lable: char) {
    println!("The measurment is: {value}{unit_lable}"); 
}
/* wrong code
fn main() {
    let x = (let y = 6);
}
*/

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}