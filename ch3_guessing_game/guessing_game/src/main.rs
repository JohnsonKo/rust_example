use std::io;

fn main() {
    println!("Please guess a number:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Fail to read!");

    let guess: u32 = match guess.trim().parse(){
        Ok(t) => t,
        Err(e) => {
            println!("error {e}");
            continue;
        }
    };

    println!("guess = {:0}", guess);
}
