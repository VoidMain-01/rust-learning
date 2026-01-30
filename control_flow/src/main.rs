fn main() {
    let num = 7;
    let number = 6;
    
    if num < 5 {
        println!("The condition is true.");
    }
    else {
        println!("The condition is false.");
    }
    
    if number % 4 == 0 {
        println!("The number is divisible by 4.");
    }
    else if number % 3 == 0 {
        println!("The number is divisible by 3.");
    }
    else if number % 2 == 0 {
        println!("The number is divisible by 2.");
    }
    else {
        println!("The number is not divisible by 4, 3 or 2.");
    }
    
    let condition = true;
    let result = if condition {5} else {6};
    
    println!("The value of result is: {result}");
}
