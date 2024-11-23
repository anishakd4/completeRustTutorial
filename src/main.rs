use std::fs::read_to_string;

fn main() {

    //variables loops and function
    variables_loop_function();

    //structs
    structs_demo();

    //Attaching function to the struct
    attach_function_to_struct();

    //enums let you enumerate over various types of a value. Like if something has some certain amount of types
    enums_demo();

    //options enum. It lets you return null/none/nil from a function.
    options_enum_demo();

    //the Result enum lets you either return Ok value or Err value. The Result enum is how you do Error handling in Rust.
    result_enum_demo();

    //Adding new crate to the package
    print_current_time()
}

fn variables_loop_function() {
    let ans = is_even(34545);
    println!("{}", ans);

    let ans2 = fib(4);
    println!("{}", ans2);

    let name = String::from("Anish Kumar");
    println!("{}", get_str_len(name));
}

fn is_even(x: i64) -> bool {
    if x % 2 == 0 {
        true
    } else {
        return false;
    }
}

fn fib(num: u32) -> u32 {
    let mut first = 0; //mutable variables
    let mut second = 1;

    if num == 0 {
        return first;
    }

    if num == 1 {
        return second;
    }

    //as we are not using i we get warning so let us replace it with underscore
    // for i in 1..num-2{
    //     let temp = second;
    //     second = second + first;
    //     first = temp
    // }

    for _ in 1..num - 1 {
        let temp = second;
        second = second + first;
        first = temp
    }
    return second;
}

fn get_str_len(str: String) -> usize {
    str.chars().count() //implicit return from the function
}

#[derive(Debug)] // use this to print whole struct. We can't simple print the whole struct, we need to apply debug trait.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn structs_demo() {
    let user1 = User {
        active: true,
        username: String::from("anishkumar3232"),
        email: String::from("aads@gmail.com"),
        sign_in_count: 1,
    };
    println!("{:?}", user1); //println macro doesn't know on its own how to print a struct
}

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 { // here self is the current struct
        self.width * self.height
    }

    fn perimeter(&self, num: u32) -> u32 { //additional parameters can be passed
        (2 * (self.height + self.width)) + num
    }

    fn debug() -> u32 { // this is like a static function in java class
        7
    }
}

fn attach_function_to_struct() {
    let rect = Rect {
        width: 30,
        height: 50,
    }; //this rect will automatically get access to the area function
    println!("{}", rect.area());
    println!("{}", rect.perimeter(5));
    println!("{}", Rect::debug());
}

//enums let you enumerate over various types of a value
//enums take less space than strings. Also they are type safe, compile time safe.
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

//enum with associated data
enum Shape {
    Circle(f64), //variant with with associated data
    Square(f64),
    Rectangle(f64, f64),
}

fn enums_demo() {
    let my_direction = Direction::East;
    print_direction(my_direction);

    let my_circle = Shape::Circle(3.14);
    calculate_area(my_circle);
    let my_square = Shape::Square(3.14);
    calculate_area(my_square);
    let my_rectangle = Shape::Rectangle(3.14, 3.14);
    calculate_area(my_rectangle);
    // println!("{}", my_circle); //`Shape` does not implement `Display`
    // println!("{}", my_square); //`Shape` does not implement `Display`
    // println!("{}", my_rectangle); //`Shape` does not implement `Display`
}

fn print_direction(my_direction: Direction) {
    println!("{:?}", my_direction);
}

//Pattern matching
fn calculate_area(shape: Shape) -> f64 {
    let area = match shape {
        Shape::Circle(a) => a * a,
        Shape::Rectangle(a, b) => a * b,
        _ => 0.0
    };

    println!("area: {}", area);

    return area;
}

//The option enum lets you return some value or none value
fn options_enum_demo() {
    let ind = find_first_a(String::from("pritam"));
    match ind {
        Some(x) => println!("first a at : {}", x),
        None => println!("a not found"),
    }
    let ind = find_first_a(String::from("ulllullu"));
    match ind {
        Some(x) => println!("first a at : {}", x),
        None => println!("a not found"),
    }
}

fn find_first_a(s: String) -> Option<i32> {
    for (index, c) in s.chars().enumerate() {
        if c == 'a' {
            return Some(index as i32);
        }
    }

    return None;
}

fn result_enum_demo() {
    let result = read_to_string("a.txt");

    match result {
        Ok(data) => println!("{}", data),
        Err(err) => println!("error: {}", err),
    }
}
use chrono::Local;
fn print_current_time() {
    let now = Local::now();
    println!("current time is: {}", now);
}

