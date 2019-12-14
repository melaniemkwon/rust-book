use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    // rand::thread_rng
    //   gives us the random number generator
    //   that is local to the current thread of execution
    //   and seeded by the operating system
    // gen_range
    //   method is defined by the Rng trait that we brought
    //   into scope with use rand::Rng
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        /*
        Rust allows us to shadow the previous value of guess
        with a new one. This feature is often used in situations
        in which you want to convert a value from one type to
        another type.

        colon (:) after guess tells Rust we’ll annotate
        the variable’s type to unsigned, 32-bit integer

        Trim
        removes whitespace from the string
        Parse
        https://doc.rust-lang.org/std/primitive.str.html#method.parse
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, //ignore non number guess instead of crashing
        };

        println!("You guessed: {}", guess);

        /*
        use a match expression to decide what to do next based on
        which variant of Ordering was returned from the call to
        cmp with the values in guess and secret_number

        A match expression is made up of arms.
        An arm consists of a pattern and the code that
        should be run if the value given to the beginning of
        the match expression fits that arm’s pattern.
        */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }//end loop
}
