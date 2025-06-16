
// Functions that returns value


fn plus_one(x: i32) -> i32 {
    x + 1
}
fn main() {
    let x: i32 = plus_one(5);
    
    println!("x is {}", x);
}