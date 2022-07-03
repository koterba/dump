use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, poll}, 
    terminal
};

use std::io::{stdout, Write};


enum SnakeDir {
    Up,
    Down,
    Left,
    Right,
    Still
}


struct CleanUp;
struct Keypress;
struct Board;

struct Snake {
    segments: Vec<Segment>,
    direction: SnakeDir
}

struct Segment {
    symbol: char,
    y: usize,
    x: usize
}


impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode")
    }
}

impl Keypress {
    fn get_key(&self) -> SnakeDir {
        if poll(std::time::Duration::from_millis(10)).expect("could not run 'poll'") {
            if let Event::Key(event) = event::read().expect("o") /* modify */ { 
                match event {
                    KeyEvent {
                        code: KeyCode::Char('w'),
                        modifiers: event::KeyModifiers::NONE,
                    } => SnakeDir::Up,
                    KeyEvent {
                        code: KeyCode::Char('s'),
                        modifiers: event::KeyModifiers::NONE,
                    } => SnakeDir::Down,
                    KeyEvent {
                        code: KeyCode::Char('a'),
                        modifiers: event::KeyModifiers::NONE,
                    } => SnakeDir::Left,
                    KeyEvent {
                        code: KeyCode::Char('d'),
                        modifiers: event::KeyModifiers::NONE,
                    } => SnakeDir::Right,
                    _ => SnakeDir::Still
                }
            } else {
                SnakeDir::Still
            }
        } else {
            SnakeDir::Still
        }
    }
}

impl Board {
    fn display_snake(&self, segments: &Vec<Segment>) {
        //clear screen
        print!("{}c", 27 as char);
        stdout().flush().expect("Could not flush stdout");
        // create a fresh board so the snake can be placed on top
        let mut board = vec![vec![' '; 15]; 15];
        // go over snake tail, and replace board with tail segments
        for segment in segments {
            board[segment.y][segment.x] = segment.symbol.clone();
        }
        
        for row in board {
            for element in row {
                print!("{element} ");
            }
            println!();
        }
    }
}

impl Snake {
    fn new() -> Self {
        Snake {
            segments: vec![
                Segment{symbol: '*', y: 5, x: 5},
                Segment{symbol: '*', y: 5, x: 6},
                Segment{symbol: '*', y: 5, x: 7},
                Segment{symbol: '*', y: 5, x: 8},
            ],
            // set default direction to be left
            direction: SnakeDir::Left
        }
    }
    
    fn update_snake(&mut self) {
        let mut prev_location = (self.segments[0].y.clone(), self.segments[0].x.clone());
        match self.direction {
            SnakeDir::Up => self.segments[0].y -= 1,
            SnakeDir::Down => self.segments[0].y += 1,
            SnakeDir::Left => self.segments[0].x -= 1,
            SnakeDir::Right => self.segments[0].x += 1,
            _ => {}
        }

        for (index, segment) in self.segments.iter_mut().enumerate() {
            if index == 0 {
                continue
            }
            let temp_prev_location = (segment.y.clone(), segment.x.clone());
            segment.y = prev_location.0;
            segment.x = prev_location.1;
            prev_location = (temp_prev_location.0, temp_prev_location.1)
        }
    }
}

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    let keypress = Keypress;
    terminal::enable_raw_mode()?;
    let mut snake = Snake::new();
    let board = Board;
    loop {
        match keypress.get_key() {
            SnakeDir::Up => snake.direction = SnakeDir::Up,
            SnakeDir::Down => snake.direction = SnakeDir::Down,
            SnakeDir::Left => snake.direction = SnakeDir::Left,
            SnakeDir::Right => snake.direction = SnakeDir::Right,
            SnakeDir::Still => {}
        }
        println!("WAITED FOR BOARD");
        snake.update_snake();
        board.display_snake(&snake.segments);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }


    keypress.get_key();
    println!("Hello, world!");

    Ok(())
}
