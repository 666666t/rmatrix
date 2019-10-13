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
//Fields and Constructor of falling trails

fn main() -> Result<(), ErrorKind> {
    let cursor = cursor();
    let term = terminal();

    cursor.hide()?;

    let delay = 45;
    let colorset: Vec<Color> = vec![Color::Green];
    let spawn_rate: f64 = 0.01;
    //Values intended to use arguments go here

    let mut trails: Vec<Trail> = Vec::new();
    let mut rng = rand::thread_rng();

    loop {
        let mut removal_flags: Vec<usize> = Vec::new();
        //Indexes of the trail are fed here to be removed at the end of each draw

        let (x, y) = term.size()?;
        for i in 0..x - 1 {
            if rng.gen::<f64>() < spawn_rate {
                trails.push(Trail::new(i.into(), rng.gen_range(5, y - 10)));
            }
        }
        //Iterate across x every loop,
        //each coordinate has a chance (variable spawn_rate) of spawning a new trail each update.
        //TODO: consider trail length as argument field too?

        for i in 0..trails.len() {
            let head_y = trails[i].y_head;
            let mut tail_y: i32 = -1;
            if trails[i].y_tail <= head_y {
                tail_y = trails[i].y_head as i32 - trails[i].y_tail as i32;
            }
            let x = trails[i].x;
            //Initial lets to avoid repeatedly calling vector element
            //Note that majority of if statements are to avoid panic from invalid coordinates.

            if head_y == y {
                trails[i].printing = false;
            }
            //Mark path to only be cleaned up by tail.

            if trails[i].printing {
                if head_y >= 1 {
                    cursor.goto(x, head_y - 1)?;
                    let line = style(format!("{}", trails[i].last_char))
                        .with(colorset[rng.gen_range(0, colorset.len())]);
                    print!("{}", line);
                }
                //Color previous line to random element of set
                //TODO: see if possible to change color without saving last value (get_character?)

                let rnchara = char::from_u32(rng.gen_range(65296, 65371)).unwrap();
                let rncharb = char::from_u32(rng.gen_range(12449, 12542)).unwrap();
                let rnchar: char;
                if rng.gen::<f64>() < 0.5 {
                    rnchar = rnchara;
                } else {
                    rnchar = rncharb;
                }
                //Random Character Generation
                //Current Set A: [65296,65370]: Fullwidth Letters+Numbers
                //Current Set B: [12449, 12541]: Fullwidth Katakana
                //TODO: Improve selection for multiple character ranges (vector of chars?)
                //TODO: Allow custom charsets via argument

                cursor.goto(x, head_y)?;
                let line = style(format!("{}", rnchar)).with(Color::White);
                print!("{}", line);
                trails[i].last_char = rnchar;
                //Print random character to head location, save character for later update
                //TODO: see last_char draw section
                //TODO?: relegate this printing to struct method
            }

            trails[i].y_head += 1;

            if tail_y != -1 {
                cursor.goto(x, tail_y as u16)?;
                print!(" ");
            }
            //Tail clearing of lines

            if (trails[i].y_head as i32 - trails[i].y_tail as i32) == y as i32 {
                removal_flags.push(i);
            }
            //Buffer indices of completed lines to be removed after loop
            //Immediate removal would require modification of i
            //TODO: Possibly Unnecessary, as lines should finish in order of creation
        }

        removal_flags.sort();
        removal_flags.reverse();
        for i in 0..removal_flags.len() {
            trails.remove(i);
        }
        //Remove Flagged lines
        //TODO: Possible modification to simply clear index 0 of trails n times,
        //where n is the amount of lines finished.

        thread::sleep(time::Duration::from_millis(delay));
    }
}
