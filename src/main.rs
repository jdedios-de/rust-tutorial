fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");

    let temp: f32 = "34.5".parse().expect("Not a number!");

    println!("The guess is {}", guess);
    println!("The temp is {}", temp);
}
