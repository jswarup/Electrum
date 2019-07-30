
#![allow(unused_variables)]
#![allow(dead_code)]


use std::io;
use rand::Rng;
use std::cmp::Ordering;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
 
fn main() {

    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:#?}", rect1);

    let user2 = User {
            email: String::from("another@example.com"),
            username: String::from("anotherusername567"),
            active: true,
            sign_in_count: 1,
        };

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Guess the number!");
    loop {
        println!("Please input your guess.");
    
        let mut     guess = String::new();
        let         r = io::stdin().read_line(&mut guess);
        r.expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
//            .expect("Please type a number!");

        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

