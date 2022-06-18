use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number : u32 = rand::thread_rng().gen_range(1..101); // or (1..=100)
    //println!("The secret number is {}",secret_number);
    loop{
        println!("Guess The Number!: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!!");
        println!("You guessed {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number){
            Ordering::Less => println!("The number is smaller!"),
            Ordering::Greater => println!("The number is greater"),
            Ordering::Equal => {
                println!("The number is equal, You Win!!");
                break;
            }
        }
    }
}
