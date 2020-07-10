#![allow(dead_code)]
use std::mem;

struct Point{
    x: f64,
    y: f64,
}
impl Point {
    fn new() -> Point {
        Point {
            x: 1.0,
            y: 1.0,
        }
    }
}

pub fn stack_and_heap(){
    let point_1 = Point::new();
    let pp = Box::new(Point::new());

    println!("p1 takes up {} bytes", mem::size_of_val(&point_1));
    println!("pp takes up {} bytes", mem::size_of_val(&pp));
}

fn say_hello(){
    println!("hello");
}

pub fn closures(){
    let sh = say_hello;
    sh();
    let far_to_cel = |far_t:f32| (far_t - 32.0) * (5.0 / 8.0);
    let fff_1 = 82.0;

    let mut my_const = 2591.12;

    let decr = |value: &mut f32, dec_v: f32| *value -= dec_v;
    let plus_my_const = |x: f32| x + my_const;

    println!("{} Farenheit is {} Celcius", fff_1, far_to_cel(fff_1));
    let x = 150.98;
    println!("x={}; x+my_const(which is {})={}",
             x, my_const, plus_my_const(x));

    //what i cannot do:
    let borrow_const = &mut my_const;
    *borrow_const += 10.55;
    // let my_const = 999.99;
    decr(&mut my_const, 111.111);
    // println!("{}",plus_my_const(5.5));
    let f = 12.44;
    let plus_my_const_ref = |x:& f32| x + my_const;
    println!("{}+{} = {}", f, my_const,plus_my_const_ref(&f));
}


pub fn fundamental_data_types(){
    let x: i32;
    x = 12;
    let y = 0;
    println! ("x is {}", x);
    println! ("y-1 is {}; size = {} bytes", y-1, mem::size_of_val(&y));
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, on {}-bit OS",
             z, size_of_z, size_of_z*8);

    let d = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));
    let e:f32 = 2.5;
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
    let f = 4 > 0;
    println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));
}

pub fn operators(){
    let mut a = 2+3*4;
    println!("{}", a);
    a = a+1;
    a -= 2;
    println!("a = {}", a);

    let remainder = a % 3;
    println!("rem-r of {} / {} = {}", a, 3, remainder);

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);
    //bitwise (logical operations)
    let c = 1 | 2;
    println!("2 | 2 = {}", c); // | OR, & AND, ^ XOR, ! NOR
    let d = 4;
    let two_to_10 = d << 10;
    println!("{}*(2^10) = {}", d, two_to_10);
    let f = 1_000_000;
    let ten_to_one = f >> 4;
    println!("{}/(2^4) = {}",f, ten_to_one);
    let x = 7;
    let x_is_5 = x == 5;
    println!("is x 5? {}", x_is_5);
}

pub fn scope_and_shadowing(){
    let a = 123;
    println!("{}", a);
    let a = 222;
    println!("{}", a);
    {
        let b = 456;
        println!("inside b = {}", b);
        let a = 101;
        println!("inside a = {}", a);
    }
    let b = 789;
    println!("outside a = {}", a);
    println!("outside b = {}", b);
}


