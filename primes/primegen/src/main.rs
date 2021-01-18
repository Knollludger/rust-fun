use std::io;

fn main() {
    println!("Please input your number.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let max = guess.trim().parse::<i64>().unwrap();

    let vector = 1..max;

    let result = vector.filter(|x| !is_prime(*x)).collect::<Vec<i64>>();

    
    println!("Primes :  {:?}", result);

    println!("Value Printed To : {}.", max.to_string());
}

fn is_prime(num:i64) -> bool{
    let vector = 2..(num/2);

    let not_prime = vector.map(|x| num % x == 0).fold(false, |x,y| x || y);

    return not_prime;
}
