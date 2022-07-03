use std::io::{self, Write, stdout};

pub fn input(prompt: &str) -> String {
	print!("{}", prompt);
	stdout().flush();

	let mut user_input: String = String::new();
	io::stdin()
	.read_line(&mut user_input)
	.expect("Could not read stdin. (User input)");
	print!("");

	user_input.trim().to_string()
}

pub fn i32_input(prompt: &str) -> i32 {
	let result = input(prompt);
	let i32_result: i32 = result.trim().parse().expect("Input was not a valid integer");

	i32_result
}
