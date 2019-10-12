use crossterm::{cursor, style, terminal};
use crossterm::{Color, ErrorKind};

use std::{char, thread, time};

use rand::prelude::*;

struct Trail {
    x: u16,
    y_head: u16,
    y_tail: u16,
    printing: bool,
    last_char: char,
}

impl Trail {
    fn new(x: u16, length: u16) -> Trail {
        Trail {
            x: x,
            y_head: 0,
            y_tail: length,
            printing: true,
            last_char: '0',
        }
    }
}

fn main() -> Result<(), ErrorKind> {
    let cursor = cursor();
    let term = terminal();

    cursor.hide()?;

    let mut trails: Vec<Trail> = Vec::new();
    let mut rng = rand::thread_rng();

    loop {
        let mut removal_flags: Vec<usize> = Vec::new();
        let (x, y) = term.size()?;
        for i in 0..x {
            if rng.gen::<f64>() > 0.99 {
                trails.push(Trail::new(i.into(), rng.gen_range(5, y - 10)));
            }
        }

        for i in 0..trails.len() {
            let head_y = trails[i].y_head;
            let mut tail_y: i32 = -1;
            if trails[i].y_tail <= head_y {
                tail_y = trails[i].y_head as i32 - trails[i].y_tail as i32;
            }
            let x = trails[i].x;

            if head_y == y {
                trails[i].printing = false;
            }

            if trails[i].printing {
                if head_y >= 1 {
                    cursor.goto(x, head_y - 1)?;
                    let line = style(format!("{}", trails[i].last_char)).with(Color::Green);
                    print!("{}", line);
                }
                cursor.goto(x, head_y)?;
                let rnchar = char::from_u32(rng.gen_range(33, 127)).unwrap();
                let line = style(format!("{}", rnchar)).with(Color::White);
                print!("{}", line);
                trails[i].last_char=rnchar;
            }

            trails[i].y_head += 1;

            if tail_y != -1 {
                cursor.goto(x, (tail_y % 65535) as u16)?;
                print!(" ");
            }

            if (trails[i].y_head as i32 - trails[i].y_tail as i32) == y as i32 {
                removal_flags.push(i);
            }
        }

        removal_flags.sort();
        removal_flags.reverse();
        for i in 0..removal_flags.len() {
            trails.remove(i);
        }
        
        thread::sleep(time::Duration::from_millis(30));
    }
}
