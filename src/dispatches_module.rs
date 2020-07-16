
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



pub fn static_dispatches(){

    let a = 123;
    let b = "a string".to_string();

    //we can define a function for a struct
    // in different ways!
    //essentially, we get concrete implementation
    // for any structs, that implement 'Printable' trait
    //this is what they call 'Static Dispatch'
    fn print_it<T: Printable>(z: &T){
        println!("{}", z.format());
    }
    // println!("{}", a.format());
    // println!("{}", b.format());
    print_it(&a);
    print_it(&b);
}

pub fn dynamic_dispatches(){

    let a = 123;
    let b = "a string".to_string();

    /// there is happening a 'Dynamic Dispatch'
    /// this function has to see,
    /// the type of 'z' argument,
    /// and call trait methods
    /// depending on the type
    /// 'z' really is
    fn print_it_too(z: &dyn Printable){

        println!("{}", z.format());

    }
    // it seems, that this function should
    // behave itself in the same way as the one above.
    // But that's not true, in general

    //Whenever this function gets called,
    //It may be called with a printable.
    //But we losing the information about
    //exact type of the parameter-object.
    //So, when we pass an argument to
    //this function, inside its body
    //we (as a dev) have no information,
    // what type, or, what
    // kind of struct the 'z' object really is.
    print_it_too(&a);
    print_it_too(&b);
}



