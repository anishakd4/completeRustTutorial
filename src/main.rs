#[derive(Debug)] // use this to print whole struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 { // here self is the current struct
        self.width * self.height
    }

    fn perimeter(&self, num: u32) -> u32 { //addition parameters can passed
        (2 * (self.height + self.width)) + num
    }

    fn debug() -> u32 { // this is like a static function in java class
        7
    }
}

fn main() {
    let ans = is_even(34545);
    println!("{}", ans);

    let ans2 = fib(4);
    println!("{}", ans2);

    let name = String::from("Anish Kumar");
    println!("{}", get_str_len(name));

    let user1 = User {
        active: true,
        username: String::from("anishkumar3232"),
        email: String::from("aads@gmail.com"),
        sign_in_count: 1,
    };
    println!("{:?}", user1); //println macro doesn't know on its own how to print a struct

    //Attaching function to the struct
    let rect = Rect {
        width: 30,
        height: 50,
    }; //this rect will automatically get access to the area function
    println!("{}", rect.area());
    println!("{}", rect.perimeter(5));
    println!("{}", Rect::debug());
}

fn is_even(x: i64) -> bool {
    if x % 2 == 0 {
        return true;
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
    str.chars().count()
}

