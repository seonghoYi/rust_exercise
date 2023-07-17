use std::io;

fn fibonacci(n: u32) -> u32 {

    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    
    let mut number = String::new();

    io::stdin().read_line(&mut number).unwrap();


    let number: u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Wrong input!!!!"),
    };

    let fibo = fibonacci(number);

    println!("fibonacci: {}", fibo);
}
