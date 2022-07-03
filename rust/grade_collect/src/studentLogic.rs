use crate::Student;
use crate::{input, i32_input};

pub fn add_student(student_list: &mut Vec<Student>) {
	student_list.push(
		Student {
			name: input("Enter name of student: "),
			grade: i32_input("Enter students grade: ")
		}
	)
}

pub fn highest_grade(student_list: &Vec<Student>) -> &Student {
	let mut highest = &student_list[0];

	for student in student_list {
		if student.grade > highest.grade {
			highest = &student
		}
	}

	highest
}

pub fn lowest_grade(student_list: &Vec<Student>) -> &Student {
	let mut lowest = &student_list[0];

	for student in student_list {
		if student.grade < lowest.grade {
			lowest = &student
		}
	}

	lowest
}
