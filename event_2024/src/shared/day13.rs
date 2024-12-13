use std::ops::{Add, Div, Mul, Sub};

use itertools::min;
use regex::Regex;

#[derive(Debug)]
pub enum ButtonType {
    A,
    B,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn prize_point(text: &str) -> Self {
        let re = Regex::new(r"(X=(?<x>\d+), Y=(?<y>\d+))").unwrap();
        let caps = re.captures(&text).unwrap();
        Self {
            x: caps.name("x").unwrap().as_str().parse::<usize>().unwrap(),
            y: caps.name("y").unwrap().as_str().parse::<usize>().unwrap(),
        }
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Mul<usize> for Point {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div for Point {
    type Output = Option<usize>;

    fn div(self, rhs: Self) -> Self::Output {
        vec![self.x / rhs.x, self.y / rhs.y].iter().min().copied()
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
    pub button_type: ButtonType,
    pub point: Point,
    pub token_price: usize,
}

impl Button {
    pub fn from_string(text: &str, btype: ButtonType) -> Self {
        let re = Regex::new(r"(X(?<x_sign>.)(?<x>\d+), Y(?<y_sign>.)(?<y>\d+))").unwrap();
        let caps = re.captures(&text).unwrap();
        let _ = caps.name("x_sign").unwrap();
        let _ = caps.name("y_sign").unwrap();
        let x = caps.name("x").unwrap().as_str().parse::<usize>().unwrap();
        let y = caps.name("y").unwrap().as_str().parse::<usize>().unwrap();

        Self {
            point: Point::new(x, y),
            token_price: match btype {
                ButtonType::A => 3,
                ButtonType::B => 1,
            },
            button_type: btype,
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
    pub fn from_string(text: &str) -> Self {
        let mut a: Option<Button> = None;
        let mut b: Option<Button> = None;
        let mut prize_point: Option<Point> = None;

        for line in text.lines().into_iter() {
            if line.contains("Button A") {
                a = Some(Button::from_string(line, ButtonType::A));
            } else if line.contains("Button B") {
                b = Some(Button::from_string(line, ButtonType::B));
            } else if line.contains("Prize") {
                prize_point = Some(Point::prize_point(&line));
            } else {
            }
        }

        if a.is_none() || b.is_none() || prize_point.is_none() {
            panic!("Failed to parse!!");
        }

        Self {
            a: a.unwrap(),
            b: b.unwrap(),
            prize: prize_point.unwrap(),
        }
    }

    pub fn least_tokens(&self) -> Option<usize> {
        let a_count: usize = 0;
        let b_count: usize = (self.prize / self.b.point).unwrap();
        
        let x_weight = if self.a.point.x < self.b.point.x { ButtonType::B } else { ButtonType::A };
        let y_weight = if self.a.point.y < self.b.point.y { ButtonType::B } else { ButtonType::A };

        let mut current: Point = self.b.point * b_count;


        println!("A: {:?}", self.a);
        println!("B: {:?}", self.b);
        println!("Prize: {:?}", self.prize);
        println!("Current: {current:?}");



        Some(0)
    }
}