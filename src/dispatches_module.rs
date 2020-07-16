
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

use crate::traits_module::{Shape, Circle, Square, Triangle};

pub fn why_dynamic_dispatch(){
    let shapes:[&dyn Shape; 6] = [
        &Circle{radius: 1.0},
        &Square{side: 3.0},
        &Circle{radius: 2.0},
        &Square{side: 4.0},
        &Triangle{side_a: 1.0, side_b: 1.0, side_c: 1.0},
        &Triangle{side_a: 3.0, side_b: 4.0, side_c: 5.0},
    ];

    for (i, shape) in shapes.iter().enumerate(){
        println!("Shape #{} has area {}", i, shape.area());
    }
}

use crate::traits_module::{Animal, Human, Cat};

enum AnimalEnum {
    //Although, we've got
    //Elements of the same ENUM,
    //the storage of particular
    //Enum element can be virtually
    //anything.
    // As an example, it can be our
    // existing struct as Human and Cat
    Human(Human), Cat(Cat)
}

pub fn vectors_of_objects(){

    let mut animals = Vec::new();
    // animals.push(Human{ name: "John" });
    // animals.push(Cat{ name: "Fluffy" });

    //we're not allowed to add elements
    //like it happens in the example above.

    //Instead, we will use Enum objects
    //as in the following example:

    animals.push(AnimalEnum::Human(
        Human{ name: "John" },
    ));
    animals.push(AnimalEnum::Cat(
        Cat{ name: "Fluffy" },
    ));

    for c in animals{
        match c {
            AnimalEnum::Human(human)=> human.talk(),
            AnimalEnum::Cat(cat) => cat.talk(),
        }
    }

    // Unfortunate side of this implementation is that
    // we have to introduce a brand new Enum, which is
    // annoying, because we already have an abstraction
    // called Animal.

    // Another downside is that to get access to the methods
    // of objects, we have to use 'match pattern' construction.
    // This thing hugely extends our code for the case if
    // we have a lot of subtypes, that implement our
    // trait.

}



