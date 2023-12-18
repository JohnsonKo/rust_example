use std::io;

fn main() {
    println!("Please guess a number:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Fail to read!");

    println!("guess = {}", guess);
}
