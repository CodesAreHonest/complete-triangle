use std::io;
use std::io::Write;

fn main() {

    print!("Input variable a -> ");

    // add flush() to prevent frequently line-buffered
    io::stdout().flush().unwrap();
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect(
        "Variable A failed to read from Stdin",
    );

    let trimmed_a = a.trim();
    match trimmed_a.parse::<u32>() {
        Ok(integer) => println!("The first integer input is: {}", integer),
        Err(..) => println!("The input '{}' is not an integer. ", trimmed_a),
    };
}
