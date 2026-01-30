fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");
    // Shadowing
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x is: {x}");
    }
    println!("The value of the x is: {x}");
}
