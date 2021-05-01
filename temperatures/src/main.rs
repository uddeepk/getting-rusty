use std::io;

fn main() {
    loop {
	// Loopin until valied input
	println!("Convert Temperature");

	println!("1. Farenheit to Celsius");
	println!("2. Celsius to Farenheit");
	println!("Please enter your choice!");
	
	let mut choice = String::new();

	io::stdin()
	    .read_line(&mut choice)
	    .expect("Failed to read line");

	let choice: i32 = match choice.trim().parse() {
	    Ok(num) => num,
	    Err(_) => continue,
	};

	println!("Enter the temperature: ");
	
	let mut temperature = String::new();

	io::stdin()
	    .read_line(&mut temperature)
	    .expect("Failed to read line");

	let temperature: f64 = match temperature.trim().parse() {
	    Ok(num) => num,
	    Err(_) => continue,
	};

	// Now calling methods
	if choice == 1 {
	    let temperature_celsius = farenheit_to_celsius(temperature);
	    println!("{} Farenheit is {} Celsius", temperature, temperature_celsius);
	}else  if choice == 2 {
	    let temperature_farenheit = celsius_to_farenheit(temperature);
	    println!("{} Celsius is {} Farenheit", temperature, temperature_farenheit);
	} else {
	    println!("Invalid Choice!\n");
	    continue;
	}
	// When input is ok
	break;
    }


}

fn farenheit_to_celsius(t: f64) -> f64 {
    (t - 32.0) * 5.0 / 9.0
}

fn celsius_to_farenheit(t: f64) -> f64 {
    t * 9.0 / 5.0 + 32.0
}
