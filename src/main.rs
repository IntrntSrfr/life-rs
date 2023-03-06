use rand::prelude::*;
use std::{thread, time};
use std::borrow::Cow;
use std::fs::File;
use gif::{Encoder, Frame, Repeat};

fn main() {
    let mut g = Game::new(16, 16, true).unwrap();
    g.run(250, false, 250);
    g.export(16, 16).unwrap();
}

#[derive(Debug)]
struct Game {
    grid: Grid,
    buf_grid: Grid,
    history: Vec<Grid>,
    rng: ThreadRng,
}

impl Game {
    fn new(h: i32, w: i32, wrap: bool) -> Result<Self, String> {
        Ok(Game {
            grid: Grid::new(h, w, wrap)?.randomize().to_owned(),
            buf_grid: Grid::new(h, w, wrap)?,
            history: Vec::new(),
            rng: thread_rng(),
        })
    }

    fn run(&mut self, iterations: i32, display: bool, delay: u64) {
        for _ in 0..iterations {
            if display { self.grid.display() }
            self.history.push(self.buf_grid.clone());
            self.step();
            if display && delay != 0 { thread::sleep(time::Duration::from_millis(delay)) }
        }
    }

    fn step(&mut self) {
        self.grid.step(&mut self.buf_grid);
        self.grid = self.buf_grid.clone();
    }

    fn export(&self, h: u16, w: u16) -> Result<(), String> {
        if self.history.len() <= 0 {
           return Err("no frames to export".to_string()); 
        }
        let palette =&[0xFF, 0xFF, 0xFF, 0, 0, 0];
        let mut f = File::create("./out.gif").unwrap();
        let mut encoder = Encoder::new(&mut f, w, h, palette).unwrap();
        encoder.set_repeat(Repeat::Infinite).unwrap();

        for g in &self.history{
            let mut frame = Frame::default();
            frame.delay = 25;
            frame.width = w;
            frame.height = h;
            frame.buffer = Cow::from_iter(g.clone().data.into_iter());
            encoder.write_frame(&frame).unwrap()
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Grid {
    data: Vec<u8>,
    height: i32,
    width: i32,
    wrap: bool
}

impl Grid {
    fn new(h: i32, w: i32, wrap: bool) -> Result<Self, String> {
        if h < 16 || w < 16 {
            return Err("grid must be at least 16x16".to_string());
        }
        return Ok(Grid {
            data: std::iter::repeat(0).take((h * w) as usize).collect(),
            height: h,
            width: w,
            wrap
        });
    }

    fn randomize(&mut self) -> &mut Self {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set(x, y, if random::<f32>() < 0.5 { 0 } else { 1 })
            }
        }
        return self;
    }

    fn step(&mut self, dst: &mut Grid) {
        for y in 0..self.height {
            for x in 0..self.width {
                let n = self.neighbours(x, y);
                let at = self.at(x, y);
                if at == 1 && (n != 2 && n != 3) {
                    dst.set(x, y, 0)
                }
                if at == 0 && n == 3 {
                    dst.set(x, y, 1)
                }
            }
        }
    }

    fn get_wrapped_pos(&self, x: i32, y: i32) -> (i32, i32) {
        return (x.rem_euclid(self.width), y.rem_euclid(self.height));
    }

    fn at(&self, mut x: i32, mut y: i32) -> u8 {
        if !self.wrap && (x < 0 || x >= self.width || y < 0 || y >= self.height ) {
            return 0;
        }
        (x, y) = self.get_wrapped_pos(x, y);

        self.data[(y * self.width + x) as usize]
    }

    fn set(&mut self, mut x: i32, mut y: i32, value: u8) {
        if !self.wrap && (x < 0 || x >= self.width || y < 0 || y >= self.height) {
            return;
        }
        (x, y) = self.get_wrapped_pos(x, y);
        self.data[(y * self.width + x) as usize] = value
    }

    fn neighbours(&self, x: i32, y: i32) -> i32 {
        let mut count = 0;
        for dy in -1..2 {
            for dx in -1..2 {
                if dy == 0 && dx == 0 {
                    continue
                }
                count += self.at(x + dx, y + dy) as i32
            }
        }
        return count;
    }

    fn display(&self) {
        let mut out = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.at(x, y) == 1 {
                    out.push_str("\u{001b}[34m# ")
                } else {
                    out.push_str("\u{001b}[31m. ")
                }
            }
            out.push('\n')
        }
        out.push_str("\u{001B}[0m"); // reset color
        println!("{}", out)
    }
}
