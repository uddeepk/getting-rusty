use std::io;

fn main() {
    println!("Nth Fibonacci number");
    println!("Enter a number: ");

    let mut n = String::new();

    io::stdin()
	.read_line(&mut n)
	.expect("Failed to read line");

    let n: i32 = n.trim().parse().unwrap();
   /* let n: i32 = match n.trim().parse() {
	Ok(num) => num,
	Err(_) => ,
    };*/

    let nth_fib = fib(n);
    //    let nth_fib

    println!("The {}th Fibonacci number is {}", n, nth_fib);
}

fn fib(n: i32) -> i32 {
    if n == 0 {
	return 0;
    }
    if n == 1  {
	return 1;
    }
    // else
    return fib(n - 1) + fib ( n - 2);
}
