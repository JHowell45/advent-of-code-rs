use std::ops::{Add, Mul, Sub};

pub enum ButtonType {
    A,
    B
}

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