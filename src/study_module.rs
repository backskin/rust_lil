#![allow(dead_code)]
use std::mem;
use std::io::stdin;

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
    let p1 = Point::new();
    let p2 = Box::new(Point::new());

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("pp takes up {} bytes", mem::size_of_val(&p2));

    let mut p3 = *p2;
    p3.x = 50.4;
    println!("{}", p3.x);
}

pub fn if_statement(){
    let temperature = 8.0;
    if temperature > 10 as f64 {
        if temperature > 15 as f64 {
            if temperature > 25 as f64 {
                println!("IT'S HOT!");
            } else {
                println!("It's kinda warm");
            }
        } else {
            println!("It's barely warm :/");
        }
    } else {
        println!("Brr... It's cold")
    }
}

pub fn while_loop(){
    let mut x = 1;
    while x < 1000{
        x *= 2;

        if x == 64 { continue; }

        println!("x = {}", x);
    }
    println!("X is DONE!");

    let mut y = 1;
    loop{
        y *= 2;
        println!("y = {}", y);

        if y == 1 << 10 { break;}
    }
    println!("Y is also DONE!");
}

pub fn for_loop(){
    for x in 1..11 {
        if x == 6 { continue; }
        if x == 8 { break; }
        println!("x = {}",x);
    }

    for (pos, y) in (30..41).enumerate(){
        println!("{}: {}", pos+1, y);
    }
}

pub fn match_tutorial(){

    let country_code = 1000;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown",
        _ => "invalid",
    };

    println!("the country with code {} is {}", country_code, country);
}

pub fn combination_lock(){

    enum State {
        Locked,
        Failed,
        Unlocked,
    }

    let code = String::from("1234");
    let mut state = State::Locked;
    // 123
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        // 3
                        entry.push_str(&input.trim_end());
                        // 12
                    }

                    Err(_) => {
                        println!("I dunno how to handle this error...");
                        continue;
                    }
                }

                if entry == code {
                    state = State::Unlocked;
                    continue;
                }

                if !code.starts_with(&entry) {
                    // "1234".starts_with("125")
                    state = State::Failed;
                }
            }

            State::Failed => {
                println!("FAILED");
                entry.clear(); // ""
                state = State::Locked;
                continue;
            }

            State::Unlocked => {
                println!("UNLOCKED!");
                return;
            }
        }
    }
}

pub fn structures(){
    struct Point(f64, f64);

    let p1 = Point(25.9, 55.11);

    println!("point p1: x = {}, y = {}", p1.0, p1.1);

    struct NameAge(String, u16);

    let human1 = NameAge(String::from("Peter"), 25);

    let human2 = NameAge(String::from("Olga"), 33);

    println!("first man is {}, he is {} old\nfirst woman is {}, she is {} old",
             human1.0, human1.1, human2.0, human2.1);

    struct Line {
        start: Point,
        end: Point,
    }

    let my_line = Line{start: Point(1.5,2.0), end:Point(4.0, 7.5)};

    println!("my line is [{}, {}, {}, {}]",
             my_line.start.0, my_line.start.1,
             my_line.end.0, my_line.end.1);
}

pub fn enumerations(){
    enum Color{
        Red, Green, Blue, RGBColor(u8,u8,u8),
        CMYK{cyan:u8, magenta:u8, yellow:u8, black:u8 },
    }

    // let c:Color = Color::RGBColor(100, 100, 50);
    // let c = Color::RGBColor(255,255,255);
    use Color::CMYK;
    let c = CMYK{cyan: 20, magenta: 165, yellow: 100, black: 251};
    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RGBColor(0,0,0) => println!("black"),
        Color::RGBColor(255,255,255) => println!("white"),
        Color::RGBColor(r,g,b) => println!("specific: r={}, g={}, b={}", r,g,b),
        CMYK {cyan:_, magenta:_, yellow:_, black: 255} => println!("black (in CMYK)"),
        _ => {}
    }
}

pub fn unions(){
    union IntOrFloat{
        i:i32,
        f: f32,
    }

    fn process_value(iof: IntOrFloat){
        unsafe {
            match iof {
                IntOrFloat { i: 42} => println!("meaning of life value"),
                IntOrFloat { f} => println!("value = {}", f),
            }
        }
    }

    let mut  iof_int = IntOrFloat{ i:128 };
    iof_int.i = 234;

    let value = unsafe {iof_int.i};
    println!("unsafe value(union)={}", value);
    process_value(IntOrFloat{i:42});
    process_value(IntOrFloat{f: 42 as f32});
    process_value(IntOrFloat{i: 100});
    process_value(IntOrFloat{ f: 10.1010});
}

pub fn options(){
    let x = 3.0;
    let y = 0.0;

    // Option -> Some(v) | None
    let result =
    if y != 0.0 { Some (x/y)} else {println!("y=0.0"); None};
    match result {
        Some(z) => println!("{}/{}={}", x,y,z),
        None => println!("cannot divide by zero"),
    }

    if let Some(zz) = result{
        println!("result = {}", zz);
    } else {
        println!("result is None");
    }
}

pub fn arrays() {


    let mut a:[i32; 5] = [1,2,3,4,5];
    a[0] = 321;
    let f_and_l = |ar: [i32; 5]|  println!("fst={}, lst={}", ar[0], ar[ar.len()-1]);
    println!("a has {} elements, first is {}", a.len(), a[0]);

    println!("{:?}", a);
    f_and_l(a);
    a[4] = 444;
    f_and_l(a);
    a[0] = 10;
    f_and_l(a);
    let val: f32 = 1.1999999999999999;
    let b = [6u8; 10];
    let c = [val; 10];
    for i in 0..b.len(){
        println!("{}", b[i]);
    }

    println!("b took up {} bytes", std::mem::size_of_val(&b));
    println!("c took up {} bytes", std::mem::size_of_val(&c));

    let mtx:[[f32;3];3] = [
        [1.000, 0.000, 0.2],
        [0.0, 2.0, 1.505],
        [5.5, 7.77, 8.42],
    ];
    println!("{:?}", mtx);
    println!("diagonals:");
    for i in 0..mtx.len(){
        for j in 0..mtx[0].len(){
            if i == j {
                println!("mtx({};{}) = {}", i,j, mtx[i][j]);
            }
        }
    }

}

pub fn slices(){
    fn use_slice( slice: &mut[i32]){
        println!("slice = {:?}", slice);
        println!("first elem = {}, len = {}", slice[0], slice.len());
        slice[0] = 4321;
        println!("we changed slice[0] to {}", slice[0]);
    }

    let mut data = [1,2,3,4,5];
    println!("data = {:?}", data);
    use_slice(&mut data[1..4]);
    println!("now data = {:?}", data);
}

pub fn tuples(){

    fn sum_and_product(x:i32, y:i32) -> (i32, i32){
        (x+y, x*y)
    }

    let x = 3;
    let y = 4;
    let sp = sum_and_product(x,y);

    println!("sum and product of x={}, y={} is: {:?}",x,y,sp);
    println!("{}, {}", sp.0, sp.1);
    println!("{0}+{1}={2}, {0}*{1}={3}", x, y, sp.0, sp.1);
    let (a,b) = sp;
    println!("a={}, b={}",a,b);
    let sp2 = sum_and_product(10, 12);
    let combo = (sp, sp2);
    println!("{:?}", combo);
    println!("last element = {}", (combo.1).1);

    let ((c,d),(e,f)) = combo;
    println!("c={}, d={}, e={}, f={}", c,d,e,f);

    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);
    let meaning = (42,); //tuple of single element
    println!("{:?}", meaning);
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