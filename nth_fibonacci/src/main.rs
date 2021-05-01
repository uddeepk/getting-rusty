use std::io;

fn main() {
    
    loop {
	println!("Nth Fibonacci number ( n starting at 0)");
	println!("Enter a number: ");

	 let mut n = String::new();

	io::stdin()
	    .read_line(&mut n)
	    .expect("Failed to read line");

	//let n: i32 = n.trim().parse().unwrap();
	let n: i32 = match n.trim().parse() {
	    Ok(num) => num,
	    Err(_) => continue,
	};

	let nth_fib = fib(n);

	println!("The {}th Fibonacci number is {}", n, nth_fib);

	break;
    }
   
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
