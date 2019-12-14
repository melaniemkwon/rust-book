use std::io;
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

    println!("Please input your guess.");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

}
