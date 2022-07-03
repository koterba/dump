use std::{thread, time};
use std::io::{Write, stdout};

fn main() {
    let mut bar = String::new();
    let mut going_up = true;
    loop {
        print!("{}c", 27 as char); // clear
        print_bar(&bar);

        if bar.len() < 1 {
            going_up = true
        } else if bar.len() > 9 {
            going_up = false
        }

        if going_up {
            bar.push('=')
        } else {
            bar.pop();
        }
        
        thread::sleep(time::Duration::from_millis(500));
    }
}

fn print_bar(bar: &String) {
    print!("<|"); // start
    print!("{}", bar); // actual bar
    for _ in 1..(11 - bar.len()) {print!(" ")} // calculates spaces from bar to end
    print!("|> {}%", get_percentage(bar)); // end
    stdout().flush();
}

fn get_percentage(bar: &String) -> usize { bar.len() * 10 }
