use std::io;
use std::io::Write;

fn main() {

    let (a, is_num) = obtain_input("a");
    println!("The variable A is {} and is a number ({})", a, is_num);

}

// the function check whether the input is number or string, then return number
fn obtain_input(var: &str) -> (f64, bool) {

    print!("Input number for variable {} -> ", var);

    // add flush() to prevent frequently line-buffered
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("readline error");

    let trimmed_input = input.trim();

    // check if the input is a number or floats
    let is_num = trimmed_input.parse::<f64>().is_ok();

    // display the message to check whether the inputs are numerical
    match trimmed_input.parse::<f64>() {
        Ok(..) => println!("The input ({}) is a number. ", trimmed_input),
        Err(..) => println!("The input ({}) is not a number. ", trimmed_input),
    };


    if is_num == true {
        let converted_value = trimmed_input.parse().unwrap();
        return (converted_value, is_num);
    } else {
        let default_value = 0.0;
        return (default_value, is_num);
    }



}