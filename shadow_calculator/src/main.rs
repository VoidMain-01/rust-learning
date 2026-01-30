fn main() {
    let x = "5";
    let x: i32 = x.parse().unwrap();
    let result = x * 50;
    
    println!("{result}");
}
