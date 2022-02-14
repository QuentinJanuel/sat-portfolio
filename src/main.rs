extern crate sat_portfolio;
extern {
    fn say_hello();
}

fn main() {
    println!("Hello, world!");
    unsafe {
        say_hello();
    }
}
