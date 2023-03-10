use std::env;

use svg::node::element::path::{Command, Data, Position};
use svg::node::element::{Path, Rectangle};
use svg::Document;


/// # Note on using `use crate`
///  Without `use crate`, we have to declare enums more explicitly like
/// `Operation::Forward` etc.
/// 
/// With `use crate`, we can simply declare `Forward` etc.
use crate::Operation::{
    Forward,
    Noop,
    Home,
    TurnLeft,
    TurnRight
};

/// # Note on using `use crate`
///  Without `use crate`, we have to declare enums more explicitly like
/// `Orientation::North` etc.
/// 
/// With `use crate`, we can simply call `North`
use crate::Orientation::{
    North,
    South,
    East,
    West
};

const WIDTH: isize = 400;
const HEIGHT: isize = WIDTH;

const HOME_Y: isize = HEIGHT / 2;
const HOME_X: isize = WIDTH / 2;

const STROKE_WIDTH: usize = 5;

#[derive(Debug, Clone, Copy)]
enum Orientation {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Forward(isize),
    Noop(u8),
    Home,
    TurnLeft,
    TurnRight
}

#[derive(Debug)]
struct Artist {
    x: isize,
    y: isize,
    heading: Orientation,
}

impl Artist {
    fn new() -> Artist {
        Artist {
            heading: North,
            x: HOME_X,
            y: HOME_Y,
        }
    }

    fn home(&mut self) {
        self.x = HOME_X;
        self.y = HOME_Y;
    }

    fn forward(&mut self, distance: isize) {
        match self.heading {
            North => self.y += distance,
            South => self.y -= distance,
            West => self.x += distance,
            East => self.x -= distance, 
        }
    }

    fn turn_right(&mut self) {
        self.heading = match self.heading {
            North => East,
            South => West,
            West => North,
            East => South, 
        }
    }

    fn turn_left(&mut self) {
        self.heading = match self.heading {
            North => West,
            South => East,
            West => South, 
            East => North, 
        }
    }

    fn wrap(&mut self) {
        if self.x < 0 {
            self.x = HOME_X;
            self.heading = West;
        } else if self.x > WIDTH {
            self.x = HOME_X;
            self.heading = East;
        }

        if self.y < 0 {
            self.y = HOME_Y;
            self.heading = North;
        } else if self.y > HEIGHT {
            self.y = HOME_Y;
            self.heading = South; 
        }
    }
}

fn parse(input: &str) -> Vec<Operation> {
    let mut steps = Vec::<Operation>::new();
    for byte in input.bytes() {
        let step = match byte {
            _ => Noop(byte),
        };
    }
    steps
}



fn main() {
    println!("Hello, world!");
}
