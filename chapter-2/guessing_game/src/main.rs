use std::io;

fn main() {
    println!("Gauessing Game !!");

    println!("Guess a Number....");

    //new is like static function on type string
    //mut means this variable can be mutated...
    let mut str = String::new();

    //& is refernce of variable str
    io::stdin().read_line(&mut str).expect("Expected a number!");

    println!("Your guess is {str}");
}
