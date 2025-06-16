fn main() {
    let a:[i32;5] = [10, 20, 30, 40, 50];

    let mut index : usize = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    for i in a {
        println!(" The value is : {}", i)
    }

    for number in (1..4).rev(){
        println!("{number}")
    }
}