fn main() {
    // let x: u8 = 256; overflow case
    let x = 3.0;  // f64
    let y: f32 = 2.0; // f32
    println!("{x}");
    println!("{y}");

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -20 / 3; // results in negative

    // remainder
    let remainder = 43 % 5;

    println!("{sum}");
    println!("{difference}");
    println!("{product}");
    println!("{quotient}");
    println!("{truncated}");
    println!("{remainder}");

    // boolean type
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // character type
    let _ch = 'z';
    let _ch2: char = 'A'; // with explicit type annotation
    let smiley_face = '😊';

    println!("{smiley_face}");

    // combined types (tuples and arrays)
    let tup: (i32, f64, u8) = (324, 4.5, 1);
    let first_value = tup.0;
    println!("{first_value}");
    let (_a, b, _c) = tup;
    println!("The value of b is: {b}");

    // arrays
    let _arr = [1, 2, 3, 4, 5];
    let months = ["Jan",  "Feb", "Mar", "Apr", "May", "June", "July", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let first = months[0];
    println!("{first}");

    let _brr: [i32; 5] = [1, 2, 3, 4, 5];
    let _crr = [3; 5]; // same as crr = [3, 3, 3, 3, 3];
}
