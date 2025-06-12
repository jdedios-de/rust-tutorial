use std::io;

fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
    
    
    println!("The sum is {}.", sum);
    println!("The difference is {}.", difference);
    println!("The product is {}.", product);
    println!("The quotient is {}.", quotient);
    println!("The truncated quotient is {}.", truncated);
    println!("The remainder is {}.", remainder);

    let t = true;

    let f: bool = false; // with explicit type annotation
    
    println!("t is {}.", t);

    println!("f is {}.", f);

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    
    println!("c is {}.", c);
    println!("z is {}.", z);
    println!("The heart-eyed cat is {}.", heart_eyed_cat);
    
    // Tuple

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    println!("The value of tup is: {:?}", tup);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    
    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);

    let a = [1, 2, 3, 4, 5];
    
    println!("The value of a is: {:?}", a);

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    
    println!("The value of months is: {:?}", months);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("The value of a is: {:?}", a);

    let aa = [3; 5];
    println!("The value of a is: {:?}", aa);

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    
    let second = a[1];
    
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);
    println!("The value of a is: {:?}", a);

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
