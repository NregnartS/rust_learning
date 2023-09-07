fn main() {
    println!("Guess a number!");
    println!("Please enter your nubmer: ");
    let mut guess = String::new();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");
}
