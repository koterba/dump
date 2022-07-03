use std::io::{stdin, stdout, Write};

enum TerminalError {
	IncorrectType,
	FlushError,
	InputError
}


fn main() {
	match input("Enter name: ") {
		Ok(res) => println!("Name was {res}"),
		Err(_) => println!("A terminal error occurred")	
	}
}

fn input(prompt: &str) -> Result<String, TerminalError> {
	print!("{prompt}");
	match stdout().flush() {
		Ok(_) => Ok(()),
		Err(_) => Err(TerminalError::FlushError)
	};

	let mut result = String::new();

	match stdin().read_line(&mut result) {
		Ok(_) => Ok(result),
		Err(_) => Err(TerminalError::InputError)
	}
}
