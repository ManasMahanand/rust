fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    {
        let mut x = x;
        x = x + 1;
        println!("The value of x is: {x}");
    }
    println!("The value of x is: {x}");
}
