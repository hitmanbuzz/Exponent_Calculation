use std::io;
use std::io::{stdout, Write};

fn main() {
    let _first_nth = 0;

    // number user input
    print!("Enter Number: ");
    stdout().flush();
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Number Input: Error Here!");
    let number: u32 = number.trim().parse().unwrap();

    // nth user input
    print!("Enter power value: ");
    stdout().flush();
    let mut nth = String::new();
    io::stdin().read_line(&mut nth).expect("nth Input: Error Here!");
    let nth: i32 = nth.trim().parse().unwrap();

    if number > 0 && nth > 0 {
        for times in (0..=nth).rev() {
            let _first_nth = times;
            loop {
                let x = number.pow(_first_nth as u32);
                println!("{number} ^ {_first_nth} = {}", x);
                break;
            }
        }
    }
}
