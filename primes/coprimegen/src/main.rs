use std::io;

fn main() {
    println!("Please input your number.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let max = guess.trim().parse::<i64>().unwrap();


    let vector = 2..max;
    let vector2 = 2..max;

    let vector = vector.map(|x| vector2.filter(|y| x != *y && x % *y == 1)).
}

fn coprime(a:i64,b:i64) -> (a,b) {

}

