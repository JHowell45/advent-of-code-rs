use std::ops::{Add, Mul, Sub};

#[derive(Debug)]
pub enum ButtonType {
    A,
    B
}

#[derive(Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Mul<usize> for Point {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[derive(Debug)]
pub struct Button {
    button_type: ButtonType,
    point: Point,
    token_price: usize
}

impl Button {
    pub fn button_a(x: usize, y: usize) -> Self {
        Self {
            button_type: ButtonType::A,
            point: Point::new(x, y),
            token_price: 3
        }
    }

    pub fn button_b(x: usize, y: usize) -> Self {
        Self {
            button_type: ButtonType::B,
            point: Point::new(x, y),
            token_price: 1
        }
    }
}

#[derive(Debug)]
pub struct ClawMachine {
    a: Button,
    b: Button,
    prize: Point,
}

impl ClawMachine {
    pub fn new(button_a_x: usize, button_a_y: usize, button_b_x: usize, button_b_y: usize, prize_x: usize, prize_y: usize) {
        Self {
            a: Button::button_a(button_a_x, button_a_y),
            b: Button::button_b(button_b_x, button_b_y),
            prize: Point::new(prize_x, prize_y),
        }
    }
}