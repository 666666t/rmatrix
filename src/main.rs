use crossterm::{cursor, style, terminal};
use crossterm::{Color, ErrorKind, Terminal, TerminalCursor};

use std::{char, thread, time};

use rand::prelude::*;

struct Trail {
    x: u16,
    y_head: u16,
    y_tail: i32,
    printing: bool,
    last_char: char,
}

impl Trail {
    fn new(x: u16, length: i32) -> Trail {
        Trail {
            x: x,
            y_head: 0,
            y_tail: 0 - length,
            printing: true,
            last_char: '0',
        }
    }

    fn printdown(&mut self, args: &Args, cursor: &TerminalCursor) -> Result<(), ErrorKind> {
        let mut rng = rand::thread_rng();

        cursor.goto(self.x, self.y_head)?;

        if self.y_head >= 1 {
            cursor.goto(self.x, self.y_head - 1)?;
            let line = style(format!("{}", self.last_char))
                .with(args.colorset[rng.gen_range(0, args.colorset.len())]);
            print!("{}", line);
        }
        //Color previous line to random element of set
        //TODO: see if possible to change color without saving last value (get_character?)

        let rnchar: char = *args.charset.choose(&mut rng).unwrap();
        //TODO: Allow custom charsets via argument

        cursor.goto(self.x, self.y_head)?;
        let line = style(format!("{}", rnchar)).with(Color::White);
        print!("{}", line);
        self.last_char = rnchar;
        //Print random character to head location, save character for later update
        //TODO: see last_char draw section

        Ok(())
    }
}

struct Args {
    delay: u64,
    colorset: Vec<Color>,
    spawn_rate: f64,
    tmin: u16,
    tmax: u16,
    charset: Vec<char>,
}

impl Args {
    fn new() -> Args {
        Args {
            delay: 25,
            colorset: vec![Color::Green],
            spawn_rate: 0.01,
            tmin: 5,
            tmax: 25,
            charset: Args::generate_charset(vec![('゠', 'ヿ'), ('０', 'ｚ')]),
        }
    }

    fn generate_charset(ranges: Vec<(char, char)>) -> Vec<char> {
        ranges
            .into_iter()
            .map(|(a, b)| (a as u32, b as u32))
            .map(|(a, b)| a..=b)
            .flatten()
            .map(|a| char::from_u32(a).unwrap())
            .collect::<Vec<char>>()
    }
}


fn main() -> Result<(), ErrorKind> {
    let cursor = cursor();
    let term = terminal();
    let user_args = Args::new();

    drawloop(&cursor, &term, &user_args)?;

    Ok(())
}

fn drawloop(cursor: &TerminalCursor, term: &Terminal, args: &Args) -> Result<(), ErrorKind> {
    let mut trails: Vec<Trail> = Vec::new();
    let mut rng = rand::thread_rng();
    cursor.hide()?;



    loop {
        let mut removals: i32 = 0;
        let (x, y) = term.size()?;

        for i in 0..x - 1 {
            if rng.gen::<f64>() < args.spawn_rate {
                trails.push(Trail::new(
                    i.into(),
                    rng.gen_range(args.tmin, y-args.tmax) as i32,
                ));
            }
        }
        //Iterate across x every loop,
        //each coordinate has a chance (variable spawn_rate) of spawning a new trail each update.
        //Trail length is [tmin, tmax)

        for i in 0..trails.len() {
            let x = trails[i].x;

            if trails[i].y_head == y {
                trails[i].printing = false;
            } else if trails[i].printing {
                trails[i].printdown(&args, &cursor)?;
            }

            if trails[i].y_tail >= 0 && trails[i].y_tail < y as i32 {
                cursor.goto(x, trails[i].y_tail as u16)?;
                print!(" ");
            } else if trails[i].y_tail == y as i32 {
                removals += 1;
            }

            trails[i].y_head += 1;
            trails[i].y_tail += 1;
        }
        //Iterate through trails, updating all coordinates that exist within terminal bounds

        for _ in 0..removals {
            trails.remove(0);
        }

        thread::sleep(time::Duration::from_millis(args.delay));
    }
}
