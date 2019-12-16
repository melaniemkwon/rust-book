fn main() {
    println!("Hello, world!");

    another_function(5, 6);
    statements_vs_expressions();

    let z = five();
    println!("The value of z is: {}", z);

    let y = plus_one(99);
    println!("The value of y is: {}", y);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn statements_vs_expressions() {
    // Statements vs Expressions

    // Statements are instructions that perform some action
    // and do not return a value.
    let a = 6;
    println!("The value of a is: {}", a);

    // {..} is an Expression
    // Expressions evaluate to something.
    let b = {       // this block evaluates to 4
        let a = 3;
        a + 1       // Expressions do not include ending semicolons.
    };              // If you add a semicolon to the end of an expression,
                    // you turn it into a statement,
                    // which will then not return a value.
    println!("The value of b is: {}", b);
}

// Functions with Return Values
fn five() -> i32 {
    5 // no semicolon
}

fn plus_one(x: i32) -> i32 {
    x + 1;      // ERROR with semicolon. statement means no return.
    // x + 1
}