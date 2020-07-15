#![allow(dead_code)]

fn pri_val<T: std::fmt::Display>(x: T) {
    println!("val = {}", x);
}

fn incr_by_ten (x: &mut i32) -> &mut i32{
    *x += 10;
    x
}

fn product (x: &i32, y:&i32) -> i32{
    *x * *y
}



pub fn functions(){
    let my_x = &mut 54.1;
    pri_val(*my_x);
    *my_x += 1.1;
    pri_val(*my_x);
    *my_x -= 51.7;
    pri_val(*my_x);
    let z = &mut 10;
    println!("now z = {}", z);
    incr_by_ten(z);
    println!("now z = {}", z);
    incr_by_ten(z);
    println!("now z = {}", z);
    let x = &4;
    let y = &16;
    println!("product of {} and {} is {}", x, y, product(x, y));
}


pub fn methods(){
    struct Point(f64, f64);
    struct Line{
        start: Point,
        end: Point,
    };

    impl Line{
        fn len(&self) -> f64{
            let dx = self.start.0 - self.end.0;
            let dy = self.start.1 - self.end.1;
            (dx*dx + dy*dy).sqrt()

        }
    }

    let my_line = Line{start: Point(0.0,1.0), end: Point(1.0, 5.0)};
    println!("line len = {}", my_line.len());
}


pub fn closures(){

    fn say_hello(){
        println!("hello");
    }

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

pub fn high_order_functions(){
    let mut sum = 0;
    for i in 0.. {
        sum += 1;
        if i > 10 { break; }
    }

    fn is_even(x: u32) -> bool {
        x % 2 == 0
    }

    let limit: u32 = 500;
    sum = 0;



    let above_limit = |y| y > limit;
    fn greater_than (limit: u32)
        -> impl Fn(u32) -> bool
    {
        move |y| y > limit
    }

    for i in 0.. {
        let isq = i*i;

        if above_limit(isq) {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }

    println!("loop sum = {}", sum);

    let sum2 = (0..).map(|x| x*x)
        .take_while(|&x| x < limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("hof sum = {}", sum2);
}