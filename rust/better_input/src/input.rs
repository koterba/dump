use std::io::{self, Write, stdout};

pub fn print_prompt(prompt: &str) {
	print!("{prompt}");
	stdout().flush().expect("Could not flush stdout");
}

pub fn input(prompt: &str) -> String {
	print_prompt(prompt);

	let mut result = String::new();
	io::stdin().read_line(&mut result).expect("Could not read stdin");
	result
}

pub fn input_trim(prompt: &str) -> String {
	let result = input(prompt);

	result.trim().to_string()
}

pub fn input_i32(prompt: &str) -> i32 {
	let result = input_trim(prompt); // using the function above instead of re-writing
	
	let result_i32: i32 = result.parse().expect("Input was not an integer");
	result_i32
}
