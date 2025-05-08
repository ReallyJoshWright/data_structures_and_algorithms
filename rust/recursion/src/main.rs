fn factorial(n: i32) -> i32 {
    if n < 0 {
        0
    } else if n == 0 || n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn factorial_tail(n: i32, a: i32) -> i32 {
    if n < 0 {
        0
    } else if n == 0 {
        1
    } else if n == 1 {
        a
    } else {
        factorial_tail(n - 1, n * a)
    }
}

fn main() {
    let n = 6;
    let mut result = factorial(n);
    println!("Result: {}", result);

    let a = 1;
    result = factorial_tail(n, a);
    println!("Result: {}", result);
}
