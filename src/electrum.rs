// electrum.rs ----------------------------------------------------------------------------------------------------------------

#![allow(unused_variables)]
#![allow(dead_code)]

//------------------------------------------------------------------------------------------------------------------------------

use std::io;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display; 
use std::io::Read;
use std::ops::Deref;
use std::sync::{Mutex, Arc, mpsc};
use std::thread;  
use rand::Rng;

//------------------------------------------------------------------------------------------------------------------------------

pub trait Draw {
    fn draw(&self);
}

//------------------------------------------------------------------------------------------------------------------------------

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

//------------------------------------------------------------------------------------------------------------------------------

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

//------------------------------------------------------------------------------------------------------------------------------

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

//------------------------------------------------------------------------------------------------------------------------------

fn test_gui() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

//------------------------------------------------------------------------------------------------------------------------------

fn test_thread() {
    let         counter = Arc::new( Mutex::new(0));
    let mut     handles = vec![];
    let mut     rx_channels = vec![];

    for i in 0..10 {
        let     counter = Arc::clone( &counter);
        let     (tx, rx) = mpsc::channel();
        rx_channels.push( rx);
        let     handle = thread::spawn( move || {
            let mut num = counter.lock().unwrap();
            let     val = i;
            tx.send(val).unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for i in 0..10 {
        let    rx = &rx_channels[ i];
        println!("{:?}", rx.recv().unwrap());
    }
    
    
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}

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

fn read_contents( filename: String) -> Result< Vec<u8>, Box<dyn std::error::Error>> 
{ 
    let mut     file = std::fs::File::open( filename)?;
    let         len = file.metadata().map(|m| m.len() as usize + 1).unwrap_or(0);
    let mut     bytes = Vec::with_capacity( len); 
    let         fsize = file.read_to_end( &mut bytes)?;
    Ok(bytes)
}

//------------------------------------------------------------------------------------------------------------------------------

struct Cacher<T> where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

//------------------------------------------------------------------------------------------------------------------------------

struct MyBox<T>( T);


impl<T> MyBox<T> {
    fn  new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
//------------------------------------------------------------------------------------------------------------------------------

fn electrum_main( args: &Vec<String>) {
    println!("args: {:#?}", args);

    test_gui();
    test_thread();

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    let     b = Box::new(5);
    println!("b = {}", *b);

    let     bytes = read_contents( String::from( "Cargo1.toml")).unwrap_or_else(  |err| { 
                println!( "Fail {}", err); 
                std::process::exit(1);
            });
    let     r;
    {
        let x = 5;
        r = x;
    }

    println!("r: {}", r);

    let     number_list = vec![34, 50, 25, 100, 65];

    let     result = largest(&number_list);
    println!("The largest number is {}", result);

    let     char_list = vec!['y', 'm', 'a', 'q'];

    let     result = largest(&char_list);
    println!("The largest char is {}", result);

    let     tweet = Tweet {
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
    let args: Vec<String> = std::env::args().collect();
    electrum_main( &args);
}

//------------------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

//------------------------------------------------------------------------------------------------------------------------------

#[test]
fn call_test(){
    let         args: Vec<String>  = Vec::new(); 
    electrum_main( &args);
    assert!(true);
}

//------------------------------------------------------------------------------------------------------------------------------

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 1);
}

//------------------------------------------------------------------------------------------------------------------------------
}

//------------------------------------------------------------------------------------------------------------------------------
