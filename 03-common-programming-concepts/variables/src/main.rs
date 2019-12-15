/*
By default variables are immutable
*/

fn main() {
    // let x = 5; // will error
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    /* Variables vs Constants
    - constants are ALWAYS immutable
    - const keyword, and type MUST be annotated
    - constants may be set only to a constant expression,
        not the result of a function call or any other value
        that could only be computed at runtime
    */
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS: {}", MAX_POINTS);

    /* Shadowing
    - you can declare a new variable with the same name as a previous variable
    - We can shadow a variable by using the same variable’s name and
        repeating the use of the let keyword
    - vs marking a variable as mut
        -Get compile-time error if we accidentally try to reassign to this variable
        without using the let keyword.
        -By using let, we can perform a few transformations on a value but have
        the variable be immutable after those transformations have been completed.
    */
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    /*
    Because we’re effectively creating a new variable when we use the
    let keyword again, we can change the type of the value but
    reuse the same name.
    */
    let spaces = "   ";
    // let mut spaces = "   ";  ERROR: can't mutate var type
    let spaces = spaces.len();
}