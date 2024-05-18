use std::io;
fn main() {
    loop {
        println!("Enter a temperature in Fahrenheit: ");
        let mut fahrenheit = String::new();
        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line☣");
        let fahrenheit: f64 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
        println!("The temperature in Celsius is: {}", celsius);
    }
}
