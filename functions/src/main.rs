fn main() {
    test_function(how_to_return(), 'h');
    ifs();
}

fn test_function(number: i32, unit: char) {
    let y = {
        let x = 1;
        x + number
    };
    println!("The number is: {y}{unit}");
}

// the return value is the tail of the function or the value after return statement
// if we add semicolon, there is no tail
fn how_to_return() -> i32 {
    5
}

fn ifs() {
    let x = if true { 10 } else { 0 };
    if x > 1 {
        println!("{x} is greater than 1");
    } else {
        println!("meh");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn loops() {
    
}