fn main() {
    // ERROR: Rust is a statically typed language.
    // let guess = "42".parse().expect("Not a number!");
    let guess: u32 = "42".parse().expect("Not a number!");

    // ### SCALAR TYPES

    // integer and float
    let int_type: i32 = 99; // i32
    let x = 2.0;        // f64
    let y: f32 = 3.0;   // f32

    // numeric operations
    let sum = 5 + 10;
    let diff = 95.5 - 4.3;
    let prod = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    // boolean type
    let t = true;
    let f: bool = false; // with explicit annot

    // character type:
    //  four bytes in size and represents a Unicode Scalar Value
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    // ### COMPOUND TYPES

    // tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x1, y1, z1) = tup;
    println!("The value of y is: {}", y);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("First tuple value is: {}", five_hundred);

    // array type
    // arrays in Rust have a fixed length, like tuples
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    println!("{:?}", a);

    let a = [1, 2, 3, 4, 5];
    let second = a[1];
    println!("second array element: {}", second);

    // compilation didn’t produce any errors, but the program resulted in
    // a runtime error and didn’t exit successfully.
    let index = 10;
    let element = a[index];
    println!("The value of element is: {}", element);
}
