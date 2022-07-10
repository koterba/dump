use std::io::{stdout, Write};
use std::time::Duration;
use std::thread;

mod physics;
use physics::{get_y, Model};

const SIZE: usize = 100;

fn new_grid() -> Vec<Vec<char>> {
    vec![vec![' '; SIZE+1]; SIZE+1]
}

fn show_grid(grid: &Vec<Vec<char>>) {
    clear();
    for row in grid {
        for element in row {
            print!("{element} ")
        }
        println!();
    }
    thread::sleep(Duration::from_millis(10));
}

fn main() {
    let model = Model {
        vel: 40.0,
        angle: 60.0,
        grav: 9.8,
    };
    let mut grid = new_grid();

    for x in 0..SIZE {
        
        let y = get_y(x as f64, &model).round() as usize;
        grid[SIZE - y][x] = '*';
        show_grid(&grid);
        
    }
}

fn clear() {
    print!("{}c", 27 as char);
    stdout().flush().expect("Could not flush stdout")
}