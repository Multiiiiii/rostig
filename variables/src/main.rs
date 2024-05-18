

fn main() {
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "     " ;
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    let remainder = 43 % 5;
    println!("The value of remainder is: {remainder}");
    
    let z = 'ℤ';
    println!("The value of z is: {z}");

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    println!("The value of the second element is: {}", tup.1);
}  