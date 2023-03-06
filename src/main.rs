use rand::prelude::*;
use std::{thread, time};

fn main() {
    let mut g = Game::new(16, 32, true).unwrap();
    g.run(4);
    //println!("{}", (-1i32).rem_euclid(10));
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

    fn run(&mut self, iterations: i32) {
        for _ in 0..iterations {
            self.grid.display();
            self.step();
            self.history.push(self.buf_grid.clone());
            self.grid = self.buf_grid.clone();
            thread::sleep(time::Duration::from_millis(500));
        }
    }

    fn step(&mut self) {
        //self.grid = self.buf_grid.clone()
        self.grid.step(&mut self.buf_grid)
    }

    fn export(&mut self) {}
}

#[derive(Debug, Clone)]
struct Grid {
    data: Vec<bool>,
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
            data: std::iter::repeat(false).take((h * w) as usize).collect(),
            height: h,
            width: w,
            wrap
        });
    }

    fn randomize(&mut self) -> &mut Self {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set(x, y, random::<f32>() < 0.5)
            }
        }
        return self;
    }

    fn step(&mut self, dst: &mut Grid) {
        for y in 0..self.height {
            for x in 0..self.width {
                let n = self.neighbours(x, y);
                let at = self.at(x, y);
                if at && (n != 2 && n != 3) {
                    dst.set(x, y, false)
                }
                if !at && n == 3 {
                    dst.set(x, y, true)
                }
            }
        }
    }

    fn get_wrapped_pos(&self, x: i32, y: i32) -> (i32, i32) {
        return (x.rem_euclid(self.width), y.rem_euclid(self.height));
    }

    fn at(&self, mut x: i32, mut y: i32) -> bool {
        if !self.wrap && (x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32) {
            return false;
        }
        (x, y) = self.get_wrapped_pos(x, y);

        self.data[(y * self.width + x) as usize]
    }

    fn set(&mut self, mut x: i32, mut y: i32, value: bool) {
        if !self.wrap && (x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32) {
            return;
        }
        (x, y) = self.get_wrapped_pos(x, y);
        self.data[(y * self.width + x) as usize] = value
    }

    fn neighbours(&self, x: i32, y: i32) -> i32 {
        let mut count = 0;
        for dy in -1..2 {
            for dx in -1..2 {
                if dy == dx {
                    continue;
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
                if self.at(x, y) {
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
