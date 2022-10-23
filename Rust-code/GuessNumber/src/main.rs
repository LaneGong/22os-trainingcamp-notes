use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess Number Start!");
    let secret_number = rand::thread_rng().gen_range(1..=100);//rng => random number generator
    loop {
        println!("Please input a number:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("read line failed");
        let guess: u32 =match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("The number you input is {guess}.");
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
