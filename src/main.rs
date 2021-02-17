// get input output from standard library
use std::io; 
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!"); 
    let secret_number = rand::thread_rng().gen_range(1, 101); 

    loop {
        println!("Input your guess:"); 
        // let creates a var
        let mut guess = String::new(); 
        // let num = 5; immutable (unchangable)
        // let mut num = 5; mutable (changable)
        // ::new - assosiated function ; static method
    
        io::stdin()
            // needs to be mutable to change guess
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // convert sting to u32 (unsigned 32-bit integer)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, 
            Err(_) => continue,
        }; 

        // {} - used for placeholding values
        println!("You guessed: {}", guess);

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