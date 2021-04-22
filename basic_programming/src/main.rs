use std::io;

fn main() {
    println!("Welcome to utility store...");
	println!("Menu:\n1. Convert Fahrenheit to Celsius\n2. Celsius to Fahrenheit\n3. Get nth Fibonacci Number\n4. Print a Christmas carol\n5. Display an insertion sort\n6. Get nth Fibonacci Number v2\n7. Exit");	
	
	loop{
		let mut input_menu_option = String::new();
		
		println!("Enter a menu option");
		
		io::stdin()
		.read_line(&mut input_menu_option).expect("Enter a valid option");
		
		let input_menu_option: u32 = match input_menu_option.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		
		if input_menu_option > 7 {
			println!("Menu option not valid. Re-enter menu option");			
		}else if input_menu_option == 1 {
			fahrenheit_to_celsius(util_get_input_u64("Enter the celsius temperature".to_string()));			
		}else if input_menu_option == 3 {
			get_nth_fibonacci(util_get_input_u64("Enter the nth instance".to_string()));
		}else if input_menu_option == 6 {
			get_nth_fibonacci_v2();
		}else if input_menu_option == 7 {
			println!("Good bye..");
			break;
		}else {
			println!("Work in progress, try an another menu");
		}
	}	
}

fn fahrenheit_to_celsius(temperature: u64) {
	let celsius: f64 = ((temperature - 32) * 5 / 9) as f64;
	println!("Fahrenheit Temperature is: {} F", celsius);
}

fn get_nth_fibonacci(n: u64){
	let mut current_value: u128 = 1;
	let mut previous_value: u128 = 0;
	
	let mut temp_store: u128 = 0;
	for _counter in 1..n {
		temp_store = current_value;
		current_value = current_value + previous_value; 
		previous_value = temp_store;
	}
	println!("The nth Fibonacci is: {}",current_value);
}

fn get_nth_fibonacci_v2(){	
	let nth_sequence = util_get_input_u64("Enter the nth instance".to_string());		
	
	let current_value: i128 = get_fibonacci_for(nth_sequence.into());
	
	println!("The nth Fibonacci is: {}",current_value);
}

fn get_fibonacci_for(n: i128) -> i128{
	if n > 2 {
		get_fibonacci_for(n-1) + get_fibonacci_for(n-2)
	}else if n == 2 {
		1
	} else {
		0
	}
}

fn util_get_input_u64(help_text: String) -> u64 {
	let mut user_input = String::new();
	println!("{}",help_text);
	io::stdin()
		.read_line(&mut user_input).expect("Enter a number");
	//let user_input: u64 = 
	match user_input.trim().parse(){
		Ok(num) => num,
		Err(_) => 0,
	}
}