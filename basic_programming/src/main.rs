use std::io;

fn main() {
    println!("Welcome to utility store...");
	println!("Menu:\n1. Convert Fahrenheit to Celsius\n2. Celsius to Fahrenheit\n3. Get nth Fibonacci Number\n4. Print a Christmas carol\n5. Display an insertion sort\n6. Exit");	
	
	loop{
		let mut input_menu_option = String::new();
		
		println!("Enter a menu option");
		
		io::stdin()
		.read_line(&mut input_menu_option).expect("Enter a valid option");
		
		let input_menu_option: u32 = match input_menu_option.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		
		if input_menu_option > 6 {
			println!("Menu option not valid. Re-enter menu option");			
		}else if input_menu_option == 1 {
			let mut celsius_input = String::new();
			println!("Enter the celsius temperature");
			io::stdin()
			.read_line(&mut celsius_input).expect("Enter a number");
			let celsius_input: u32 = match celsius_input.trim().parse(){
				Ok(num) => num,
				Err(_) => continue,
			};
			fahrenheit_to_celsius(celsius_input);			
		}else if input_menu_option == 3 {
			let mut nth_instance = String::new();
			println!("Enter the nth instance");
			io::stdin()
			.read_line(&mut nth_instance).expect("Enter a number");
			let nth_instance: u32 = match nth_instance.trim().parse(){
				Ok(num) => num,
				Err(_) => continue,
			};
			get_nth_fibonacci(nth_instance);
		}else if input_menu_option == 6 {
			println!("Good bye..");
			break;
		}else {
			println!("Work in progress, try an another menu");
		}
	}	
}

fn fahrenheit_to_celsius(temperature: u32) {
	let celsius = (temperature - 32) * 5 / 9;
	println!("Fahrenheit Temperature is: {} F", celsius);
}

fn get_nth_fibonacci(n: u32){
	let mut current_value: u64 = 1;
	let mut previous_value: u64 = 0;
	
	let mut temp_store: u64 = 0;
	for _counter in 1..n {
		temp_store = current_value;
		current_value = current_value + previous_value; 
		previous_value = temp_store;
	}
	println!("The nth Fibonacci is: {}",current_value);
}

