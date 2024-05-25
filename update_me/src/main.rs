use std::future::Future;


fn main() {
    update_me(main);
    update_me(|| {
        println!("Hello, world!");
    });
}



fn update_me<F>(f:F) -> impl Future<Output = ()>
where F: Fn() { 
    
    
    async move {
        loop {
            f();
        }
    }
}