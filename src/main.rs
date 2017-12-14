use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;

// multiple producer, single consumer.
use std::sync::mpsc;

// use time crate
extern crate time;
use time::PreciseTime;


fn main() {


    let a = obtain_input("a");
    let b = obtain_input("b");
    let c = obtain_input("c");

    let bln_valid_triangle = form_triangle(a, b, c);
    println!("Validity of forming triangle ({})", bln_valid_triangle);

    if bln_valid_triangle == true {

        let sequential_count = classify_triangle(a, b, c);
        println!(
            "The number of triangle can be generated are {} (Sequential)",
            sequential_count
        );

        let concurrent_count = concurrency_classify(a, b, c);
        println!(
            "The number of triangle can be generated are {} (Multi-Channel)",
            concurrent_count
        );
    }

}

// function use to determine whether the inputs can form valid triangle
fn form_triangle(a: u64, b: u64, c: u64) -> bool {
    println!("a = {}, b = {}, c = {}", a, b, c);

    if a + b > c && b + c > a && c + a > b {
        return true;
    } else {
        return false;
    }
}

fn concurrency_classify(a: u64, b: u64, c: u64) -> u64 {

    let start = PreciseTime::now();

    // transmitter and receiver over the channel
    let (equilateral_tx, equilateral_rx) = mpsc::channel();
    let (scalene_tx, scalene_rx) = mpsc::channel();
    let (isosceles_tx, isosceles_rx) = mpsc::channel();

    thread::spawn(move || {
        let equilateral_count = equilateral(a, b, c);
        equilateral_tx.send(equilateral_count).unwrap();
    });

    thread::spawn(move || {
        let scalene_count = scalene(a, b, c);
        scalene_tx.send(scalene_count).unwrap();
    });

    thread::spawn(move || {
        let isosceles_count = isosceles(a, b, c);
        isosceles_tx.send(isosceles_count).unwrap();
    });

    let equilateral_receive = equilateral_rx.recv().unwrap();
    let scalene_receive = scalene_rx.recv().unwrap();
    let isosceles_receive = isosceles_rx.recv().unwrap();

    let count = equilateral_receive + scalene_receive + isosceles_receive;

    pythagorean_theorem(a, b, c);
    let end = PreciseTime::now();



    println!(
        "{} seconds on classify triangle concurrently. ",
        start.to(end)
    );

    return count;
}

/*
* 	This function is use to classify the triangle whether there are:
*		1. Scalene (all sides are different)
*		2. Isosceles (two sides are equal)
*		3. Equilateral (all three sides are equal)
*/
fn classify_triangle(a: u64, b: u64, c: u64) -> u64 {

    let start = PreciseTime::now();

    let equilateral_count = equilateral(a, b, c);
    let scalene_count = scalene(a, b, c);
    let isosceles_count = isosceles(a, b, c);
    let count = equilateral_count + scalene_count + isosceles_count;

    pythagorean_theorem(a, b, c);

    let end = PreciseTime::now();

    println!(
        "{} seconds on classify triangle sequentially. ",
        start.to(end)
    );




    return count;

}

// the function determine whether the triangle is isosceles
fn isosceles(a: u64, b: u64, c: u64) -> u64 {

    thread::sleep(Duration::from_secs(2));
    let mut count: u64 = 0;

    if (a == b && a != c && b != c) || (a == c && a != b && b != c) ||
        (b == c && a != b && a != c)
    {
        count = count + 1;
        println!("The triangle is Isosceles (two sides are equal).");
    }

    return count;

}

// the function determine whether the triangle is equilateral
fn equilateral(a: u64, b: u64, c: u64) -> u64 {

    thread::sleep(Duration::from_secs(2));
    let mut count: u64 = 0;

    if a == b && b == c && c == a {
        count = count + 1;
        println!("The triangle is Equilateral (all three sides are equal).");
    }

    return count;
}

// the function determine whether the triangle is Scalene
fn scalene(a: u64, b: u64, c: u64) -> u64 {

    thread::sleep(Duration::from_secs(2));
    let mut count: u64 = 0;

    if a != b && b != c && c != a {
        count = count + 1;
        println!("The triangle is Scalene (all sides are different).");
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

// determine whether the triangle are acute, right and obtuse triangle
fn pythagorean_theorem(a: u64, b: u64, c: u64) {

    let a_square = a * a;
    let b_square = b * b;
    let c_square = c * c;

    println!(
        "The square of A = {}, B = {}, C = {}",
        a_square,
        b_square,
        c_square
    );


    if a_square + b_square == c_square {
        println!("The triangle is Right triangle.");
    } else if a_square + b_square > c_square {
        println!("The triangle is Acute triangle.");
    } else if a_square + b_square < c_square {
        println!("The triangle is Obtuse triangle.");
    }
}
