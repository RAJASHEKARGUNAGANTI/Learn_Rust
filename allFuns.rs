fn main() {
    mult(10);
    div(10);
    add(10);
    sub(10);
}

fn mult(n: i32) {
    for i in 1..=10 {
        let result = n * i;
        println!("{} * {} = {}", n, i, result);
    }
    println!();
}

fn div(n: i32) {
    for i in 1..=10 {
        let result = n / i;
        println!("{} / {} = {}", n, i, result);
    }
    println!();
}

fn add(n: i32) {
    for i in 1..=10 {
        let result = n + i;
        println!("{} + {} = {}", n, i, result);
    }
    println!();
}

fn sub(n: i32) {
    for i in 1..=10 {
        let result = n - i;
        println!("{} - {} = {}", n, i, result);
    }
    println!();
}