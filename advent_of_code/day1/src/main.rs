use std::io;

fn main() {
    let mut left: Vec<_> = Vec::new();
    let mut right: Vec<_> = Vec::new();

    loop {
        let mut numbers = String::new();
        let _ = io::stdin().read_line(&mut numbers);
        if numbers.len() <= 2 {
            break;
        }
        let numbers_vec: Vec<_> = numbers.trim().split("   ").collect();
        let n1: i32 = numbers_vec[0].parse().expect("NaN1");
        let n2: i32 = numbers_vec[1].parse().expect("NaN2");

        left.push(n1);
        right.push(n2);
    }

    left.sort();
    right.sort();
    
    let mut res = 0;
    for n in 0..left.len() {
        res = res + (left[n] - right[n]).abs();
    }
    println!("{}", res);
}
