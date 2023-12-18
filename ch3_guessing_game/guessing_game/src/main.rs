use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::Colorize;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 50);

    println!("secret number is {}", secret_number);

    loop{
        println!("Please guess a number:");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read!");
    
        let guess: u32 = match guess.trim().parse(){
            Ok(t) => t,
            Err(e) => {
                println!("{} {e}", "error:".red());
                continue;
            }
        };

        // println!("guess = {:0}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}", "Too small!".on_red()),
            Ordering::Greater => println!("{}", "Too big!".on_red()),
            Ordering::Equal => {
                println!("{}", "You win!".on_green());
                break;
            }
        }
    }
}
