mod betterInput;
mod studentLogic;

use betterInput::{i32_input, input};
use studentLogic::{add_student, highest_grade, lowest_grade};

#[derive(Debug)]
pub struct Student {
    name: String,
    grade: i32,
}

fn main() {
    clear();
    let mut student_list: Vec<Student> = Vec::new();
    loop {
        let user_choice = i32_input("Enter a number to pick:\n\n1) add student\n2) view students\n3) view highest grade\n4) view lowest grade\n\n> ");
        clear();

        if user_choice == 1 {
            add_student(&mut student_list);
            clear();
        } else if user_choice == 2 {
            println!("{:#?}", student_list);
        } else if user_choice == 3 {
            println!("{:#?}", highest_grade(&student_list));
        } else if user_choice == 4 {
        	println!("{:#?}", lowest_grade(&student_list));
        }
    }
}

fn clear() {
    print!("{}c", 27 as char);
}
