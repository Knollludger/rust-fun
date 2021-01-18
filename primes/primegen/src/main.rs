use std::io;

fn main() {
    println!("Please input your number.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let max = guess.trim().parse::<i32>().unwrap();

    let vector = 1..max;

    let result = vector.map(|x| x * 2).map(|x| x * 2).collect::<Vec<i32>>();

    println!("Mapped: {:?}", result);

    println!("Max Value : {}", max.to_string());
}
