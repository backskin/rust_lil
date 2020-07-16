#![allow(dead_code)]

trait Animal
{
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

trait Summable<T>{
    fn sum(&self) -> T;
}

pub fn traits(){

    struct Human{
        name: &'static str,
    }

    struct Cat{
        name: &'static str,
    }

    impl Summable<i32> for Vec<i32>{
        fn sum(&self) -> i32 {
            let mut res = 0;
            for x in self {
                res += *x;
            }
            res
        }
    }

    impl Animal for Human {
        fn create(name: &'static str) -> Human {
            Human { name }
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn talk(&self) {
            println!("Hello, I'm {}", self.name);
        }
    }

    impl Animal for Cat{
        fn create(name: &'static str) -> Cat {
            Cat { name }
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn talk(&self) {
            println!("*{}, the cat*: Meow!", self.name);
        }
    }


    let h = Human::create("John");
    h.talk();
    let c = Cat::create("Misty");
    c.talk();
    c.talk();

    let an_int_vec = vec![1,2,3,4,5,6,7,];
    println!("vec sum = {}", an_int_vec.sum());

}

use std::fmt::Debug;
use crate::study_module::while_loop;
use std::borrow::Borrow;

trait Shape {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Triangle {
    side_a:f64,
    side_b:f64,
    side_c:f64,
}

#[derive(Debug)]
struct Square {
    side:f64,
}

impl Shape for Circle{
    fn area(&self) -> f64 {
        self.radius*self.radius*std::f64::consts::PI
    }
}

impl Shape for Triangle{
    fn area(&self) -> f64 {
        let h_per = 0.5 * (self.side_a + self.side_b + self.side_c);
        (h_per * (h_per - self.side_a) * (h_per - self.side_b) * (h_per - self.side_c)).sqrt()
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

pub fn trait_params(){

    // we use '&' for borrowing the address, not the value itself
    // otherwise we would move the value (a Shape object) to the function,
    // this will cause an Error, if we will address the same variable
    // (this variable will be guaranteed uninitialized by that moment)
    fn print_area(shape: &(impl Shape + Debug)){
        println!("The area is {}", shape.area());
    }

    let circle = Circle {radius: 2.0};
    print_area(&circle);

    fn print_shape_area<T: Shape + Debug>(shape: &T){
        println!("shape area = {}", shape.area())
    }

    let sq = Square{ side: 20.0};
    println!("Square, side={}", sq.side);
    print_shape_area(&sq);
    let tri = Triangle{
        side_a: 15.5,
        side_b: 12.2,
        side_c: 18.0,};
    println!("Triangle, a={}, b={}, c={}",
             tri.side_a, tri.side_b, tri.side_c);
    print_shape_area(&tri);

    fn print_area_template<T>(shape: &T) where T: Shape + Debug
    {
        println!("{:?}", shape);
        println!("The mthfkr area is {}", shape.area());
    }

    print_area_template(tri.borrow());
    print_area_template(&circle);
    print_area_template(&circle);

}


