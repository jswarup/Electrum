// electrum.rs ----------------------------------------------------------------------------------------------------------------

#![allow(unused_variables)]
#![allow(dead_code)]

//------------------------------------------------------------------------------------------------------------------------------

use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;

//------------------------------------------------------------------------------------------------------------------------------

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

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}


impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//------------------------------------------------------------------------------------------------------------------------------

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//------------------------------------------------------------------------------------------------------------------------------

fn electrum_main() {
    let  r;

    {
        let x = 5;
        r = x;
    }

    println!("r: {}", r);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut     team_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    let         a_int = 40;
    let         a_team = String::from("Green");
    team_scores.insert( &a_team, &a_int);
    println!("{:?}", team_scores );

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let v = vec![1, 2, 3, 4, 5];

    let     third: &i32 = &v[2];
    println!("The third element is {}", third);
    //let does_not_exist = &v[100];
    //let does_not_exist = v.get(100);

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:#?} area: {} ", rect1, rect1.area());

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
//------------------------------------------------------------------------------------------------------------------------------

fn main() {
    electrum_main();
}

//------------------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

//------------------------------------------------------------------------------------------------------------------------------

#[test]
fn call_test(){
    electrum_main();
    assert!(true);
}

//------------------------------------------------------------------------------------------------------------------------------
}

//------------------------------------------------------------------------------------------------------------------------------
