use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number");

    let random_number = rand::thread_rng().gen_range(0..=100);

    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error Reading line");

        let user_input: i32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        match user_input.cmp(&random_number) {
            std::cmp::Ordering::Less => {
                println!("Too small")
            }
            std::cmp::Ordering::Equal => {
                println!("You guessed right");
                break;
            }
            std::cmp::Ordering::Greater => {
                println!("Too Large")
            }
        }
    }
}
