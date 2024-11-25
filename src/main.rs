use chrono::Local;
use std::collections::HashMap;
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
    print_current_time();

    //Moving demo
    moving_demo();
    moving_demo2();

    //Reference rule demo
    reference_rule_demo();

    //vector_demo
    vector_demo();

    //hashmap_demo
    hashmap_demo();

    //tuples
    tuples_demo();

    //iterator demo
    println!("");
    println!("iterator demo");
    iterator_demo();

    //mutable iterator demo
    println!("");
    println!("mutable iterator demo");
    mutable_iterator_demo();

    //iterator demo using next
    println!("");
    println!("iterator demo using next");
    iterator_demo_using_next();

    //Into iterator demo
    println!("");
    println!("into iterator demo");
    into_iterator_demo();

    //consuming adaptors
    println!("");
    println!("consuming adapters demo");
    consuming_adaptors_demo();

    //iterator adaptors
    println!("");
    println!("iterator adapters demo");
    iterator_adaptors_demo();

    //iterator in hashmaps
    println!("");
    println!("iterator in hashmap demo");
    hashmap_iterator_demo();

    //string operations
    println!("");
    println!("string operations demo");
    string_operations_demo();

    //slice demo
    println!("");
    println!("slice demo");
    slice_demo();
    slice_demo2();

    //string types
    println!("");
    println!("string types demo");
    string_types();

    //array slice
    println!("");
    println!("array slice demo");
    array_slice_demo();

    //generics demo
    println!("");
    println!("generics demo");
    generics_demo();
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
    fn area(&self) -> u32 {
        // here self is the current struct
        self.width * self.height
    }

    fn perimeter(&self, num: u32) -> u32 {
        //additional parameters can be passed
        (2 * (self.height + self.width)) + num
    }

    fn debug() -> u32 {
        // this is like a static function in java class
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
        _ => 0.0,
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

//The Result enum lets you return either ok value or Err value. The result enum is how you can do error handling in Rust
fn result_enum_demo() {
    let result = read_to_string("a.txt"); //read_to_string comes from the Rust standard library

    match result {
        Ok(data) => println!("{}", data),
        Err(err) => println!("error: {}", err),
    }
}

fn print_current_time() {
    let now = Local::now();
    println!("current time is: {}", now);
}

fn moving_demo() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);
    //println!("{}", s1); //error as s2 is the current owner. Value used after being moved
}

fn moving_demo2() {
    let s1 = String::from("hello");
    moving_demo3_print_str(s1); //this is similar to doing let s3 = s1
                                //println!("{}", s1); //error as s2 is the current owner. Value used after being moved
}

fn moving_demo3_print_str(s3: String) {
    println!("{}", s3);
}

fn reference_rule_demo() {
    let mut s1 = String::from("hello");
    let mut s2 = &mut s1;
    reference_rule_demo_print_str(&mut s1);
    //println!("{} {}", s1, s2); //cannot borrow `s1` as mutable more than once at a time
}

fn reference_rule_demo_print_str(s1: &mut String) {
    println!("{}", s1);
}

//vectors allow you to store more than 1 value in a single data structure that puts all values next to each other in memory
// as we can add elements in it and remove from it, it is stored in heap and a variable in stack is there pointing it on heap.
fn vector_demo() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("vec {:?}", vec); //vector is struct and it implements debug trait thats why we are able to print whole vector using ?:
    println!("even_filter {:?}", even_filter(vec));

    let mut vec2 = Vec::new();
    vec2.push(1);
    vec2.push(2);
    vec2.push(3);
    println!("even_filter2 {:?}", even_filter2(&vec2));

    let mut vec3 = Vec::new();
    vec3.push(1);
    vec3.push(2);
    vec3.push(3);
    println!("even_filter3 {:?}", even_filter3(&mut vec3));

    let numbers = vec![1, 2, 3, 4, 5]; //vector initialize using a macro
    println!("numbers {:?}", numbers);
}

fn even_filter(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec: Vec<i32> = Vec::new();
    for v in vec {
        if v % 2 == 0 {
            new_vec.push(v);
        }
    }
    return new_vec;
}

fn even_filter2(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec: Vec<i32> = Vec::new();
    for v in vec {
        if v % 2 == 0 {
            new_vec.push(*v);
        }
    }
    return new_vec;
}

fn even_filter3(vec: &mut Vec<i32>) {
    let mut i = 0;
    while i < vec.len() {
        if vec[i] % 2 != 0 {
            vec.remove(i);
        } else {
            i += 1;
        }
    }
}

fn hashmap_demo() {
    let mut users: HashMap<String, u32> = HashMap::new();
    users.insert("Anish Kumar".to_string(), 42);
    users.insert("renu Kumar".to_string(), 43);

    let first_user_age = users.get("Anish Kumar");

    match first_user_age {
        Some(val) => println!("The first user age is {}", val),
        None => println!("No user age found"),
    }
}

fn tuples_demo() {
    let input_vec = vec![(String::from("anish"), 32), (String::from("renu"), 33)];
    let hm = group_by_values(input_vec);
    println!("hm: {:?}", hm);
}

fn group_by_values(input_vec: Vec<(String, u32)>) -> HashMap<String, u32> {
    let mut hm = HashMap::new();
    for (key, value) in input_vec {
        hm.insert(key, value);
    }
    return hm;
}

fn iterator_demo() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    //the iter method in rust provides a way to iterate over the elements of a collection by borrowing
    //them. v1_iter doesn't become owner of these values. We can't mutate these values as iter has immuatble
    //reference to these values

    for val in v1_iter {
        println!("val is {}", val);
    }

    println!("{:?}", v1);
}

fn mutable_iterator_demo() {
    let mut v1 = vec![1, 2, 3];
    let v1_iter = v1.iter_mut();

    for val in v1_iter {
        *val += 1;
    }

    println!("{:?}", v1);
}

fn iterator_demo_using_next() {
    let mut v1 = vec![1, 2, 3, 4];
    let mut v1_iter = v1.iter_mut();

    //for loop hides some of the complexity of using iterator.
    while let Some(val) = v1_iter.next() {
        println!("val is {}", val);
    }

    println!("{:?}", v1);
}

//the into iterator trait is used to convert a collection into an iterator that takes the ownership
// of the collection
fn into_iterator_demo() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.into_iter();
    for val in v1_iter {
        println!("val is {}", val);
    }

    //println!("{:?}", v1); //ownership gets transferred in case of into_iter
}

fn consuming_adaptors_demo() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    println!("total is {}", total);

    //Value used after being moved [E0382]. ownership got moved to sum function
    // for val in v1_iter {
    //
    // }
}

fn iterator_adaptors_demo() {
    let v1 = vec![1, 2, 3, 4];
    let v1_iter = v1.iter();

    let v1_iter2 = v1_iter.map(|x| x + 1);

    for val in v1_iter2 {
        println!("val is {}", val);
    }

    let v2 = vec![1, 2, 3, 4];
    let v2_iter = v2.iter();

    let v2_iter2 = v2_iter.filter(|x| *x % 2 == 0);

    for val in v2_iter2 {
        println!("val is {}", val);
    }

    let v3 = vec![1, 2, 3, 4, 5];
    let v3_iter = v3.iter();

    let v3_iter2 = v3_iter.filter(|x| *x % 2 == 1).map(|x| x * 2);

    let v3_new: Vec<i32> = v3_iter2.collect();
    println!("v3: {:?}", v3_new);
}

fn hashmap_iterator_demo() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Green"), 30);

    for (key, value) in scores.iter() {
        println!("{} is scores {}", key, value);
    }

    for (key, value) in scores.iter_mut() {
        *value += 10;
        println!("{} is scores {}", key, value);
    }
}

fn string_operations_demo() {
    let mut name = String::from("Anish");
    name.push_str(" Kumar");
    println!("name is {}", name);

    name.replace_range(8..name.len(), "");
    println!("name is {}", name);
}

fn slice_demo() {
    let mut word = String::from("hello world");
    let word2 = &word[0..5];

    //word.clear(); //cannot borrow `word` as mutable because it is also borrowed as immutable

    println!("word2 is {}", word2);
}

fn slice_demo2() {
    let word = String::from("kumar anish");
    let word2 = find_first_word(&word);

    println!("word2 is {}", word2);
}

fn find_first_word(word: &String) -> &str {
    let mut index = 0;

    for (_, i) in word.chars().enumerate() {
        if i == ' ' {
            break;
        }
        index = index + 1;
    }

    return &word[0..index];
}

fn string_types() {
    let name = String::from("Anish"); //String type
    let name2 = &name; //has a view from original string/is a reference
    let name3 = "manish"; //literal is also an &str but it points directly to an address in the binary.
}

fn array_slice_demo(){
    let arr = [1, 2, 3];
    println!("arr is {:?}", arr);

    let array_slice = &arr[0..1];
    println!("array_slice is {:?}", array_slice);
}

fn generics_demo(){
    let bigger = largest(5, 2);
    let biggest = largest('a', 'g');

    println!("bigger is {}", bigger);
    println!("biggest is {}", biggest);
}

fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}