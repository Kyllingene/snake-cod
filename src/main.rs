use std::time::Instant;
use std::{process::exit, time::Duration};

use rand::{thread_rng, Rng};

use cod::*;
use device_query::{DeviceQuery, DeviceState, Keycode};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    None,
    Up,
    Left,
    Down,
    Right,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Pos {
    pub x: u32,
    pub y: u32,
}

struct Food {
    pub pos: Pos,

    max: Pos,
}

impl Food {
    pub fn randomize(max_x: u32, max_y: u32) -> Self {
        Self {
            pos: Pos {
                x: thread_rng().gen_range(1..=max_x),
                y: thread_rng().gen_range(1..=max_y),
            },

            max: Pos { x: max_x, y: max_y },
        }
    }

    pub fn new(max_x: u32, max_y: u32) -> Self {
        Food::randomize(max_x, max_y)
    }

    pub fn eat(&mut self) {
        let new = Food::randomize(self.max.x, self.max.y);
        self.pos = new.pos;
    }
}

struct Snake {
    pub head: Pos,
    pub tail: Vec<Pos>,
    pub dir: Direction,

    max: Pos,
}

impl Snake {
    pub fn new(max_x: u32, max_y: u32) -> Self {
        Self {
            head: Pos { x: 1, y: 1 },
            tail: Vec::new(),
            max: Pos { x: max_x, y: max_y },
            dir: Direction::None,
        }
    }

    pub fn update(&mut self) {
        if !self.tail.is_empty() {
            self.tail.pop();
            self.tail.insert(0, self.head);
        }

        match self.dir {
            Direction::Up => self.head.y -= 1,
            Direction::Left => self.head.x -= 1,
            Direction::Down => self.head.y += 1,
            Direction::Right => self.head.x += 1,
            Direction::None => {}
        }

        if self.head.x > self.max.x {
            self.head.x = 1;
        } else if self.head.x < 1 {
            self.head.x = self.max.x;
        }

        if self.head.y > self.max.y {
            self.head.y = 1;
        } else if self.head.y < 1 {
            self.head.y = self.max.y;
        }

        if self.tail.contains(&self.head) {
            self.lose();
        }
    }

    pub fn aim(&mut self, dir: Direction) {
        self.dir = dir;
    }

    pub fn eat(&mut self) {
        self.tail.insert(0, self.head);
    }

    pub fn lose(&self) {
        clear();
        home();

        println!("Game Over!\n\n  Score: {}", self.tail.len());
        exit(0);
    }
}

struct Timer {
    pub duration: Duration,

    start: Instant,
    complete: bool,
    loopable: bool,
}

impl Timer {
    pub fn new(duration: f32, loopable: bool) -> Self {
        Self {
            duration: Duration::from_secs_f32(duration),

            start: Instant::now(),
            complete: duration != 0.,
            loopable,
        }
    }

    pub fn poll(&mut self) -> bool {
        if self.complete {
            if self.loopable {
                self.start = Instant::now();
                self.complete = false;

                return false;
            } else {
                return true;
            }
        }

        self.complete = self.start.elapsed() >= self.duration;
        self.complete
    }
}

fn main() {
    let mut food = Food::new(13, 10);
    let mut snake = Snake::new(13, 10);

    let keyboard = DeviceState::new();
    let mut timer = Timer::new(0.3, true);

    loop {
        clear();

        if timer.poll() {
            snake.update();

            if snake.head == food.pos {
                snake.eat();
                food.eat();
            }

            color_fg(6);
            rect('+', 1, 1, 15, 12);
            text(format!("Score: {}", snake.tail.len()), 1, 13);

            color_fg(11);
            pixel('@', food.pos.x + 1, food.pos.y + 1);

            color_fg(2);
            for &p in &snake.tail {
                pixel('#', p.x + 1, p.y + 1);
            }

            color_fg(10);
            pixel('O', snake.head.x + 1, snake.head.y + 1);

            bot();
        }

        let keys: Vec<Keycode> = keyboard.get_keys();
        for key in keys {
            match key {
                Keycode::W | Keycode::Up => snake.aim(Direction::Up),
                Keycode::A | Keycode::Left => snake.aim(Direction::Left),
                Keycode::S | Keycode::Down => snake.aim(Direction::Down),
                Keycode::D | Keycode::Right => snake.aim(Direction::Right),
                Keycode::Q | Keycode::Escape => snake.lose(),
                _ => {}
            }
        }

        sleep(0.02);
    }
}
