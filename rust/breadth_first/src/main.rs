use std::io::{Write, stdout};
use std::{thread, time};

fn main() {
    let mut board = vec![vec![' ';30];30];
    let start = (0, 0);
    board[start.0][start.1] = '0'; // starting piece
    board[29][29] = '*'; // ending piece

    let mut queue = Vec::new();
    queue.push((start.0, start.1));

    while queue.len() > 0 {
        let (row, col) = queue.pop().unwrap();

        if row < 0 || col < 0 || row > board.len()-1 || col > board[0].len()-1 || board[row][col] == '.' {
            continue
        }

        if board[row][col] == '*' {
            break
        }
        board[row][col] = '.';

        queue.push((row+1, col)); // down
        if row != 0 {queue.push((row-1, col))} // up
        queue.push((row, col+1)); // right
        if col != 0 {queue.push((row, col-1))} // left

        clear();
        display_board(&board);
        thread::sleep(time::Duration::from_millis(2));
    }
}

fn display_board(board: &Vec<Vec<char>>) {
    for row in board {
        for element in row {
            print!("{element} ")
        }
        println!("");
    }
}

fn clear() {
    print!("{}c", 27 as char);
    stdout().flush().expect("Could not flush stdout");
}
