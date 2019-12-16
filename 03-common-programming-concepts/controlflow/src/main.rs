fn main() {
    let number = 3;

    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }

    // condition MUST be boolean type. no auto conversions
    // if number {
    //     println!("will this compile?");
    // }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if is an expressions, so you can use it in a let statement
    let condition = true;
    let number = if condition {
        5
    } else {
        // "six"   // ERROR: type mismatch
        6
    };
    println!("The value of number is: {}", number);

    // ### LOOP
    // loop { // runs forever
    //     println!("again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            // pass the result of that operation to
            // the rest of your code after 'break'
            break counter * 2;
        }
    };
    println!("The loop result is {}", result);

    // ### WHILE
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // ### FOR
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // use rev() to reverse the range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
