#![allow(dead_code)]
#![allow(unused_variables)]
use std::fmt::Formatter;

fn how_many(x:i32) -> &'static str{
    match x {
        0 => "no",
        1 | 2 => "one or two",
        3..=5 => "enough",
        7 => "exactly seven",
        9 | 11 => "lots of",
        12 => "a dozen",
        _ if (x % 2 == 0) => "some even amount of",
        _ => "a few",
    }
}

pub fn pattern_matching(){
    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let point = (5,1);

    match point {
        (0,0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y),
        (x, 0) => println!("y axis, x = {}", x),
        (_, y) => println!("(?, {})",y),
        // (x, y ) => println!("point={:?}", point),
    }

    enum Color{
        Red, Green, Blue, RGBColor(u8,u8,u8),
        CMYK{cyan:u8, magenta:u8, yellow:u8, black:u8 },
    }

    // let c:Color = Color::RGBColor(100, 100, 50);
    // let c = Color::RGBColor(255,255,255);
    let c = Color::RGBColor(0,0,0);
    use Color::CMYK;
    // let c = CMYK{cyan: 20, magenta: 165, yellow: 100, black: 255};
    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RGBColor(0, 0, 0) => println!("black"),
        Color::RGBColor(255, 255, 255) => println!("white"),
        Color::RGBColor(r, g, b) => println!("specific: r={}, g={}, b={}", r, g, b),
        CMYK { black: 255, .. } => println!("black (in CMYK)"),
        _ => {}
    }
}

pub fn generics(){

    struct Point<P>{
        x: P,
        y: P,
    }

    struct Line<T>{
        start: Point<T>,
        end: Point<T>,
    }

    impl std::fmt::Debug for Line<i32> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
            print!("Line: start=[{}, {}]; end=[{}, {}]", self.start.x, self.start.y, self.end.x, self.end.y);
            Ok(())
        }
    }

    impl std::fmt::Debug for Line<f32> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
            print!("Line: start=[{}, {}]; end=[{}, {}]", self.start.x, self.start.y, self.end.x, self.end.y);
            Ok(())
        }
    }

    let a = Point { x: 0, y: 0};
    let f = Point {x: 1, y: -1};
    let b = Point { x: 1.2, y: 3.4};
    let d = Point {x: 1.9, y: 5.5};
    let my_line = Line { start: a, end: f};
    let other_line = Line { start: b, end: d };
    println!("{:?}\n{:?}", my_line, other_line);
}

