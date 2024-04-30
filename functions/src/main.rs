fn main() {
    test_function(how_to_return(), 'h');
    ifs();
    loop_and_break();
    loop_in_loop();
    while_loop();
    for_and_while_collections();
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

fn loop_and_break() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

fn loop_in_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
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
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("baloezzzz!!!");
}

fn for_and_while_collections() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("[while] the value is: {}", a[index]);
        index += 1;
    }
    
    for element in a {
        println!("[for] the value is: {}", element);
    }

    for number in (1..10).rev() {
        println!("{number}");
    }
    println!("baloeeeezzz!");
}