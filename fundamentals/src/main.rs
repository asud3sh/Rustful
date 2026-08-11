mod an_eg_module;
use an_eg_module::say_haribol;

fn main() {
    println!("======= Type Annotation ======");
    explicit_type_annotate();
    println!("======= Control Flow ======");
    eg_control_flow();
    println!("======= Tuple ======");
    eg_tuple();
    println!("======= Array ======");
    eg_array();
    println!("======= Struct ======");
    eg_struct();
    println!("======= Enum ======");
    eg_enum();
    println!("======= Module ======");
    say_haribol();
    println!("======= Trait ======");
    eg_traits();
    println!("======= Generics ======");
    eg_generics();
    println!("======= Exceptions ======");
    eg_exceptions();
    println!("======= Ownership ======");
    eg_own_and_borrow();
}

fn eg_tuple() {
    let person = ("mnpr", 30, true); // tuple of (&str, i32, bool)
    let name = person.0; // access tuple element by index 0,..
    let age = person.1;
    let is_student = person.2;
    println!("Name : {name}, Age {age}, Student {is_student}");
    // Destructuring a tuple
    let (name, age, student_status) = person;
    println!("Name : {name}, Age {age}, Student {student_status}");
}

fn eg_array() {
    let numbers = [1, 2, 3, 4, 5]; // array of 5 i32s, type inferred as [i32; 5]
    let first_number = numbers[0]; // access array element by index 0,..
    println!("{first_number}");
    // initialize array with the same value
    let all_zeros = [0; 5]; // array of 5 i32s, all initialized to 0
    println!("{all_zeros:?}");
    // Arrays are fixed-size and allocated on the stack by default.
}

fn explicit_type_annotate() {
    let days: i32 = 32;
    let price: f64 = 99.99;
    let name: &str = "Rust";
    println!("Days : {days}, Price : {price}, Name : {name}");
}

fn eg_control_flow() {
    let number: i32 = 5;
    if number < 5 {
        println!("Less than 5");
    } else if number == 7 {
        println!("It's seven!");
    } else {
        println!("Greater than or equal to 5 and not seven");
    }
    // if is an expression, so it returns a value
    let result = if number < 5 {
        "Less than 5"
    } else {
        "It's seven!"
    };
    println!("{result}");

    let mut counter = 0;
    loop {
        counter += 1;
        println!("Counter in loop : {counter}");
        if counter == 5 {
            break;
        }
    }

    let mut number = 3;
    while number != 0 {
        println!("Number in while: {number}");
        number -= 1;
    }
    // iterate over a range (inclusive)
    for i in 0..10 {
        println!("Counter in for: {i}");
        if i == 5 {
            break;
        }
    }

    let an_array = [10, 20, 30, 40, 50];
    for element in an_array {
        println!("For an Element in an array: {element}");
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    const fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn eg_struct() {
    let square = Rectangle {
        width: 5,
        height: 5,
    };
    println!("Area of square is : {}", square.area());
}

#[allow(dead_code)] // allow dead code for now
enum Directions {
    North,
    East,
    West,
    South,
}

fn eg_enum() {
    let direction = Directions::North;
    match direction {
        Directions::North => println!("Go North"),
        Directions::South => println!("Go South"),
        Directions::East => println!("Go East"),
        Directions::West => println!("Go West"),
    }
}

trait Summary {
    fn summarize(&self) -> String; // trait method signature
}

struct NewsArticle {
    headline: String,
    author: String,
    content: String,
}

// implement the Summary trait for the NewsArticle struct
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, {}: {}", self.headline, self.author, self.content)
    }
}

fn eg_traits() {
    let article = NewsArticle {
        headline: String::from("Rust is awesome!"),
        author: String::from("experience so far"),
        content: String::from("Rust is a great programming language"),
    };

    println!("Summary: {}", article.summarize());
}

// generic struct
struct Point<T> {
    x: T,
    y: T,
}

fn eg_generics_struct() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.5, y: 2.5 };
    println!(
        "Integer point coordinates: ({}, {})",
        integer_point.x, integer_point.y
    );
    println!(
        "Float point coordinates: ({}, {})",
        float_point.x, float_point.y
    );
}

// generic function
// works for any type T passed into it
fn wrap_in_tuple<T>(item: T) -> (T, T)
where
    T: Clone,
{
    (item.clone(), item)
}

fn eg_generics_function() {
    let numbers = wrap_in_tuple(5);
    let words = wrap_in_tuple("phew");
    println!("numbers: {numbers:?}");
    println!("words: {words:?}");
}

fn eg_generics() {
    eg_generics_struct();
    eg_generics_function();
}

use std::fs;
use std::io::Error;

fn read_file_content(filename: &str) -> Result<String, Error> {
    fs::read_to_string(filename)
}

fn eg_exceptions() {
    match read_file_content("imp_note_na.txt") {
        Ok(content) => {
            println!("======== File Content =======");
            println!("{content}");
        }
        Err(error) => {
            println!("======== Caught Exception =======");
            println!("{error}");
        }
    }
}

// s is a reference to a String not ownership
const fn calculate_length(s: &str) -> usize {
    s.len() // can access string data through reference
}

fn change_string(s: &mut String) {
    // s is a mutable reference, allowing modification
    s.push_str(" and practice");
}

fn eg_own_and_borrow() {
    let mut my_string = String::from("learn Rust");
    let length = calculate_length(&my_string);
    println!("Length of my_string : {length}");
    change_string(&mut my_string);
    println!("Modified string : {my_string}");
}
