use std::io;
use std::io::Write;

fn main() {

    let a = obtain_input("a");
    let b = obtain_input("b");
    let c = obtain_input("c");

    let bln_valid_triangle = form_triangle(a, b, c);
    println!("Validity of forming triangle ({})", bln_valid_triangle);

    let triangle_count = classify_triangle(a, b, c);
    println!(
        "The number of triangle can be generated are {}",
        triangle_count
    );

}

fn form_triangle(a: u64, b: u64, c: u64) -> bool {
    println!("a = {}, b = {}, c = {}", a, b, c);

    if a + b > c && b + c > a && c + a > b {
        return true;
    } else {
        return false;
    }
}

/*
* 	This function is use to classify the triangle whether there are:
*		1. Scalene (all sides are different)
*		2. Isosceles (two sides are equal)
*		3. Equilateral (all three sides are equal)
*/
fn classify_triangle(a: u64, b: u64, c: u64) -> u64 {

    let mut count: u64 = 0;


    if a == b && b == c && c == a {
        count = count + 1;
        println!("The triangle is equilateral.");
    } else if a != b && b != c && c != a {
        count = count + 1;
        println!("The triangle is Scalene.");
    } else if (a == b && a != c && b != c) || (a == c && a != b && b != c) ||
               (b == c && a != b && a != c)
    {
        count = count + 1;
        println!("The triangle is Isosceles.");
    }

    return count;

}

// the function check whether the input is number or string, then return number
fn obtain_input(var: &str) -> u64 {

    loop {
        print!("Input number for variable {} -> ", var);

        // add flush() to prevent frequently line-buffered
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("readline error");

        // trimmed the input to remove empty space
        let trimmed_input = input.trim();

        // check if the input is a number or floats
        let is_num = trimmed_input.parse::<u64>().is_ok();

        // display the message to check whether the inputs are numerical
        match trimmed_input.parse::<u64>() {	
            Ok(..) => println!("The input ({}) is a integer. ", trimmed_input),
            Err(..) => println!("The input ({}) is not a integer. ", trimmed_input),
        };

        let input: u64;

        // check whether the input is number, convert string to integer if its number
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
                return input;
            }

        } else {
            println!("Your input is not between 1 to 1000000, please type again");
        }

    }
}
