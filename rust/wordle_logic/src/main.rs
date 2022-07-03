use colored::Colorize;

fn main() {
    let target_word = String::from("fruit");
    let guess = String::from("friad");

    let mut result = String::new();

    for i in 0..(target_word.len()) {
    
        let iter_guess_letter = guess.chars().nth(i).unwrap().to_string();
        let iter_target_letter = target_word.chars().nth(i).unwrap().to_string();
        
        if iter_guess_letter == iter_target_letter {
            result.push_str(&iter_guess_letter.green())
        } else if target_word.contains(&iter_guess_letter) {
            result.push_str(&iter_guess_letter.yellow())
        } else {
            result.push_str(&iter_guess_letter)//guess.chars().nth(i).unwrap())
        }
    }

    println!("{result}");

    println!("{}", "blue".blue());
}
