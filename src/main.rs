use std::io;
use std::io::Write;

fn main() {

    let mut a = obtain_input("a");
    let mut b = obtain_input("b");
    let mut c = obtain_input("c");

    println!("a = {}, b = {}, c = {}", a, b, c);

}

// the function check whether the input is number or string, then return number
fn obtain_input(var: &str) -> u64 {

    loop {
        print!(
            "Input number for variable {} (between 1 to 1000000) -> ",
            var
        );

        // add flush() to prevent frequently line-buffered
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("readline error");

        let trimmed_input = input.trim();

        // check if the input is a number or floats
        let is_num = trimmed_input.parse::<u64>().is_ok();

        // display the message to check whether the inputs are numerical
        match trimmed_input.parse::<u64>() {	
            Ok(..) => println!("The input ({}) is a integer. ", trimmed_input),
            Err(..) => println!("The input ({}) is not a integer. ", trimmed_input),
        };

        let input: u64;

        if is_num == true {
            let converted_value = trimmed_input.parse().unwrap();
            input = converted_value;
        } else {
            input = 0;
        }

        // check whether the number input are among 1 to 1000000
        if input >= 1 && input <= 1000000 {
            let valid_num = true;

            if is_num == true && valid_num {
                //            let converted_value = trimmed_input.parse().unwrap();
                return input;
            }
        } else {
            println!("Your input is not between 1 to 1000000, please type again");
        }

    }
}
