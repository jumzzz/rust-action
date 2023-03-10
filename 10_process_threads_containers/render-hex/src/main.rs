use std::env;

use svg::node::element::path::{Command, Data, Position};
use svg::node::element::{Path, Rectangle};
use svg::Document;

use crate::Operation::{
    Forward,
    Noop,
    Home,
    TurnLeft,
    TurnRight
};

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





fn main() {
    println!("Hello, world!");
}
