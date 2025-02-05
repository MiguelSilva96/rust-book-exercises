use std::io;
use std::collections::HashMap;

fn main() {
    let mut left: Vec<_> = Vec::new();
    let mut right: HashMap<i32, i32> = HashMap::new();

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
        if right.contains_key(&n2) {
            let tmp = right.get(&n2).unwrap();
            right.insert(n2, tmp + 1);
        } 
        else {
            right.insert(n2, 1);
        }
    }
    
    let mut res = 0;
    for n in 0..left.len() {
        if right.contains_key(&left[n]) {
            res += left[n] * right[&left[n]];
        }
    }
    println!("{}", res);
}
