use std::io;

fn main() {
    println!("Enter the number");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line☣");
    let n: u64 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let result = n_th_fibo(n);

    println!("The nth Fibonacci nubmer is {}", result)
}

fn n_th_fibo(n: u64) -> u64 {
    let mut fiba = 0;
    let mut fibb = 1;
    let mut i =1;
    while i < n {
        let tmp = fibb;
        fibb += fiba;
        fiba = tmp;
        i += 1
        }
        return fiba  
    
}
