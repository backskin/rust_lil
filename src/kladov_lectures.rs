#![allow(dead_code)]

fn use_x<T: std::fmt::Display>(x: T){
    println!("{}", x);
}

pub fn intro() {
    fn foo() {
        let x = Box::new(92.1);
        let r: &f32 = &x;
        use_x(r);
    }

    foo();

    // let r: &i32;
    // {
    //     let x = Box::new(92);
    //     r = &x; // - this is not allowed (variable 'x' does not live long enough)
    // } // - by that time, the x will be cleaned up
    // // use_x(r); // COMPILER ERROR - forbidden query for a data, that 'r' might have,
    // // 'cause it doesn't has it by that moment.

    //another example
    let a: &i32 = &55;

    {
        let b = 79;
        // a = &b;
        let a = &b; // - shadowing will keep original data in outer scope
        println!("inner a = {}", *a);
    }

    println!("outer a = {}", *a);


    // use_x(a); - still COMPILER ERROR:
    // error[E0597]: `b` does not live long enough

    let spam = Box::new(96);
    let eggs = spam;
    println!("{}", eggs);  // ERROR - borrow of moved value: `spam`

    // fn drop<T>(_value: T){
    //
    // }

    let x = Box::new(1);
    let y = Box::new(2);
    drop(x);
    drop(y);

    // aphine's types
    // let x: T = foo() in
    // if condition { fun_one(x);} else {fun_two(x)}
    // if condition_with_x { y } else {z }

    //now - how should you work with the memory in Rust
    let x = String::from("data"); // data's memory allocating
    let y = &x; // immutable reference to x
    // (but not the reference to a data itself)
    println!("{}", x); // ok, 'cause the x still holds the link
    // to a memory, allocated for our data
    println!("{}", y); // also ok, we retrieving data from
    // reference-to-a-reference-to-a-data
    fn print_vec_sorted(xs: &mut Vec<u8>) {
        xs.sort();
        for x in xs {
            println!("{}", x);
        }
    }
    let mut xs = vec![1, 2, 3];
    print_vec_sorted(&mut xs);  // ok
    print_vec_sorted(&mut xs);  // ok
    // My thoughts about current progress:
    // the greatest disadvantage of Rust language
    // is that one man cannot use it correctly.
    // even the example up above had to be rewritten
    // due to syntax and object types issues
    // R.I.P RUST (soon)
    let _x = ();
    println!("{}",  'f');
}