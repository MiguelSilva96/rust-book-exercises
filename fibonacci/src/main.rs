use std::io;


fn main() {
    println!("yay let's calculate fibonacci!");
    println!("which position do you want to know?");
    let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

    let n: u64 = n.trim().parse().expect("it should be a number!!");
    let result = fibonacci(n);
    println!("The result is {result}")
}


fn fibonacci(n: u64) -> u64 {
    if n <= 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}