extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn guess() -> u32 {

    //mutable String
    let mut guess = String::new();

    let mut guess_u32 = 0;

    loop {

        println!("Please to guess a number > 0...");

        //read the string
        io::stdin().read_line(&mut guess)
            .ok()
            //if it's not ok, return the following line
            .expect("Failed to read your number...");

        //transform and return the string to a 32 bits integer
        guess_u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to transform the string into an integer...");
                continue
            }
        };

        break;
    }

    guess_u32

}

fn new_rand_u32() -> u32 {

    //return new random number
    rand::thread_rng().gen_range(1, 101)

}

fn main() {

    //new random number
    let rand_u32 : u32 = new_rand_u32();

    //infinite loop
    loop {

        let guess = guess();

        //print the guess number...
        println!("Your number is {}", guess);

        //match the guess number with the random number
        match guess.cmp(&rand_u32) {
            //if guess is lower than rand_u32...
            Ordering::Less => println!("Too small..."),
            //if guess is greater than rand_u32...
            Ordering::Greater => println!("Too big..."),
            //if guess is equals to rand_u32, we quit!
            Ordering::Equal => {
                println!("Great job!");
                break;
            }
        }
    }
}
