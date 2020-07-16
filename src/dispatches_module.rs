
trait Printable{
    fn format(&self) -> String;
}

impl Printable for i32
{
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String{
    fn format(&self) -> String {
        format!("string: {}", *self)
    }
}

//we can define a function for a struct
// in different ways!

//essentially, we get concrete implementation
// for any structs, that implement 'Printable' trait
fn print_it<T: Printable>(z: T){
    println!("{}", z.format());
}

pub fn static_dispatches(){

    let a = 123;
    let b = "a string".to_string();

    // println!("{}", a.format());
    // println!("{}", b.format());
    print_it(a);
    print_it(b);
}