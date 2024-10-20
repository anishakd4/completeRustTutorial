fn main() {
    let ans = is_even(34545);
    println!("{}", ans);

    let ans2 = fib(4);
    println!("{}", ans2);
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
