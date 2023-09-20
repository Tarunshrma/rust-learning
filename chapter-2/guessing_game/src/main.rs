use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Gauessing Game !!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //    println!("Secret number is {secret_number}");
    println!("Guess a Number....");

    loop {
        //Note: new is like static function on type string
        //Note: mut means this variable can be mutated...
        let mut guess = String::new();

        //Note: & is refernce of variable str
        io::stdin()
            .read_line(&mut guess)
            .expect("Expected a number!");

        //NOTE: guess variable can be redeclaredwith the different type this feature is called "Shadowing"
        //NOTE: parse will convert the string to expected type.. e.g. u32 in this case
        //NOTE: is string can not be converted to number, error can be handled
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //NOTE: Compare & Match
        //NOTE: Looks like switch case
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
