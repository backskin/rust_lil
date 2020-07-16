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

pub(crate) trait Shape {
    fn area(&self) -> f64;
}

#[derive(Debug)]
pub(crate) struct Circle {
    pub(crate) radius: f64,
}

#[derive(Debug)]
pub(crate) struct Triangle {
    pub(crate) side_a:f64,
    pub(crate) side_b:f64,
    pub(crate) side_c:f64,
}

#[derive(Debug)]
pub(crate) struct Square {
    pub(crate) side:f64,
}

impl Shape for Circle{
    fn area(&self) -> f64 {
        self.radius*self.radius
            *std::f64::consts::PI
    }
}

impl Shape for Triangle{
    fn area(&self) -> f64 {
        let h_per = 0.5*
            (self.side_a + self.side_b + self.side_c);
        (h_per
            *(h_per - self.side_a)
            *(h_per - self.side_b)
            *(h_per - self.side_c)
        ).sqrt()
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

pub fn trait_params(){

    // we use '&' for borrowing the address, not the value itself
    // otherwise we would move the entire value
    // (a Shape object) to the function -
    // this will cause a compilation error,
    // when we will refer to the same variable
    // (this variable will be guaranteed uninitialized by that moment)

    //Three ways to use traits in function's parameters

    //First - litteraly use it as parameter type
    fn print_area(shape: &(impl Shape + Debug)){
        println!("The area is {}", shape.area());
    }

    let circle = Circle {radius: 2.0};
    print_area(&circle);

    //Second - use it as generic's constraint
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

    //Third - using a 'where' operator
    fn print_area_template<T>(shape: &T) where T: Shape + Debug
    {
        println!("{:?}", shape);
        println!("The mthfkr area is {}", shape.area());
    }

    print_area_template(tri.borrow());
    print_area_template(&circle);
    // now we can refer to the same variable without a fear
    // of losing its value
    print_area_template(&circle);

}

struct Person{
    name: String,
}

impl Person{
    // fn new(name: &str) -> Person{
    //     Person {name: name.to_string()}
    // }

    // now we specify the template of type
    // to be convertible _into_ the String struct.
    // (basically, 'Into' is just another trait)

    // fn new<S: Into<String>>(name: S)-> Person {
    //     Person {
    //         name: name.into(),
    //     }
    // }

    //and, of course, there is another/alternative way
    // to implement 'new' function with the 'Into' trait:

    fn new<S>(name: S) -> Person
    where S: Into<String>{
        Person { name: name.into(), }
    }
}

pub fn into_the_into(){

    //Into
    // Most of the code for this exercise is up above

    let _john = Person::new("John");
    let name = "Jane Watson".to_string();
    let _jane = Person::new(name);
}

struct Creature {
    name: String,
}

impl Creature {
    fn new<T: Into<String>>(name: T) -> Creature {
        Creature { name: name.into(), }
    }

}
// next stop is where we implement Drop trait to
// specify where and how exactly our value will be
// cleaned up

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} is dead", self.name);
    }
}

pub fn drops(){

    let gargoyle = Creature::new("Jeff");
    println!("game proceeds");
    // we do nothing here, yet by the end of the scope,
    // the .drop method will be executed, so we should
    // see our exclamation, implemented inside
    // the 'Drop trait' .drop method above

    // furthermore, if we will execute .drop explicitly,
    // the compiler will report an error:
    // "error[E0040]: explicit use of destructor method"
    // gargoyle.drop();

    // Even so we're allowed to call a global drop() function
    // to keep object's destruction event in touch
    // (to definitely know, when the data of variable gonna be cleaned up)
    drop(gargoyle);

    // after this step, compiler won't allow us to use
    // our variable 'gargoyle' anymore.
    // "error[E0382]: borrow of moved value: `gargoyle`"
    // println!("{}",gargoyle.name);

    //In most cases, however, it is a rather unnecessary
    // thing to have in your code (we're talking about Drop trait)
}

use std::ops::{Add, AddAssign, Neg, Sub,};
use std::cmp::{PartialEq};

#[derive(Debug, Copy, Clone)]
struct Complex<T>{
    re: T,
    im: T,
}

impl <T> Complex<T>{
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> { re, im }
    }
}

// impl Add for Complex<i32>{
//     type Output = Complex<i32>;
//
//     fn add(self, rhs: Self) -> Self::Output {
//        Complex {
//            re : self.re + rhs.re,
//            im: self.im + rhs.im,
//        }
//     }
// }

// Looks like we need to make
// this Add-operator to be a generic one
// as well as a Complex struct already is.


impl<T> Add for Complex<T>
    where T: Add<Output = T>
{
    type Output = Complex<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re : self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T> AddAssign for Complex<T>
    where T: AddAssign<T>
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> Neg for Complex<T>
    where T: Neg<Output=T>
{
    type Output = Complex<T>;
    fn neg(self) -> Self::Output {
        Complex{
            re: -self.re,
            im: -self.im,
        }
    }
}

impl<T> Sub for Complex<T>
    where T: Sub<Output=T> + Neg<Output=T> + Add<Output=T>
{
    type Output = Complex<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        self+-rhs
    }
}

impl<T> PartialEq for Complex<T>
    where T: PartialEq
{
    fn eq(&self, rhs: &Self) -> bool {
        self.re == rhs.re && self.im == rhs.im
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, )]
struct Shmomplex<Y>{
    re: Y,
    im: Y,
}

impl<SomeType> Shmomplex<SomeType>{
    fn new(re: SomeType, im: SomeType) -> Shmomplex<SomeType> {
        Shmomplex{
            re, im
        }
    }
}

impl<T> Add for Shmomplex<T>
    where T: Add<Output=T>{
    type Output = Shmomplex<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Shmomplex { re: self.re + rhs.re, im: self.im + rhs.im, }
    }
}

impl<T> Sub for Shmomplex<T>
    where T: Sub<Output=T>{
    type Output = Shmomplex<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Shmomplex{ re:self.re - rhs.re, im: self.im - rhs.im, }
    }
}

impl<T> Neg for Shmomplex<T>
    where T: Neg<Output=T>{
    type Output = Shmomplex<T>;

    fn neg(self) -> Self::Output {
        Shmomplex{ re: -self.re, im: -self.im, }
    }
}

impl<T> AddAssign for Shmomplex<T>
    where T: AddAssign<T>{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

pub fn operator_overloading(){
    let a = Complex::new(1, 2);
    let b = Complex::new(1, 2);
    let res_1 = a+b;
    println!("sum = {:?}", res_1);
    let c = Complex::new(1.0, 2.0);
    let d = Complex::new(1.5, 2.5);
    let res_2 = c-d;
    println!("sum = {:?}; c={:?}, d={:?}", res_2, c, d);
    println!("a == b ? - {}; c == d ? - {}; a == c ? - no idea",
    a == b, c == d);

    // Also we actually don't always need to implement
    // Equality traits by a hand.
    // Instead, we might just ask compiler to derive
    // the implementation automatically

    let mut shm_a = Shmomplex::new(1.8, 5.8);
    let shm_b = Shmomplex::new(2.7, 1.2);
    println!("Shmomplex:\na = {:?};\nb = {:?};\na+b = {:?};\
    \na-b = {:?};\na+-b = {:?}.",
    shm_a,shm_b,shm_a+shm_b,shm_a-shm_b,shm_a+-shm_b);
    shm_a += shm_b;
    println!("after a+=b, a = {:?}", shm_a);
}


