use std::io;
use std::io::Write;

fn main() {

    let (mut a, is_a_int) = obtain_input("a");
    let (mut b, is_b_int) = obtain_input("b");
    let (mut c, is_c_int) = obtain_input("c");

}

// the function check whether the input is number or string, then return number
fn obtain_input(var: &str) -> (u64, bool) {

    loop {
        print!("Input number for variable {} -> ", var);

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


        if is_num == true {
            let converted_value = trimmed_input.parse().unwrap();
            return (converted_value, is_num);
        }


    }


}
