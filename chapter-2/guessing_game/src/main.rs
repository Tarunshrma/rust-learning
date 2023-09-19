use std::io;

fn main() {
    println!("Gauessing Game !!");

    println!("Guess a Number....");

    let mut str = String::new();

    io::stdin().read_line(&mut str).expect("Expected a number!");

    println!("Your guess is {str}");
}
