use std::io::{self};

fn main() {
    println!("Printing a diamond. Please enter a digit between 1 and 9:");
    // get the diamond size
    let n: u32;
    match getdigit() {
        Ok(d) => n = d,
        Err(_) => {
            println!("Please enter a non-zero digit.");
            return;
        },
    }
    // print out the diamond
    for i in 0..n {
        for _ in 0..n-i-1 {
            print!(" ");
        }
        for _ in 0..2*i+1 {
            print!("*");
        }
        println!();
    }
    for i in (0..n-1).rev() {
        for _ in 0..n-i-1 {
            print!(" ");
        }
        for _ in 0..2*i+1 {
            print!("*");
        }
        println!();
    }
}

fn getdigit() -> Result<u32,()> {
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("Read failed");
    
    for c in input_text.chars() {
        // c is a digit but cant be 0
        if c == '0' {
            return Err(());
        }
        if c.is_digit(10) {
            return Ok(c.to_digit(10).unwrap());
        }
    }
    // throw an error
    Err(())
}