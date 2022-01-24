use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Secret number: {}",secret_number);
    println!("Guess the number!");

    loop {
        println!("Please type your guess. ('q' to quit)");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

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
