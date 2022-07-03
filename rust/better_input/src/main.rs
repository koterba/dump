mod input;
use input::input_trim;

fn main() {
	let equation = input_trim("Enter equation: ");
	let equation: Vec<&str> = equation.split_whitespace().collect();
	
	let (first_num, op, second_num) = (
		equation[0].parse::<i64>().expect("broke"),
		equation[1],
		equation[2].parse::<i64>().expect("broke")
	);

	match op {
		"+" => println!("{} + {} = {}", first_num, second_num, first_num + second_num),
		"-" => println!("{} - {} = {}", first_num, second_num, first_num + second_num),
		"x" => println!("{} x {} = {}", first_num, second_num, first_num * second_num),
		"/" => println!("{} / {} = {}", first_num, second_num, first_num / second_num),
		_ => println!("Not a valid operator")
	}
}
