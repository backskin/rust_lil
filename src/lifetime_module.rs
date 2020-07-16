pub fn ownership(){
    //v 'OWNS' a vector-object
    //right after the assignment
    let v = vec![1,3,5];
    // in this case below,
    // v2 copies pointer to the memory effectively
    // that means that v2 now 'owns' the object
    // compiler understands this situation and
    // prevents you from previous variable
    // exploitation
    let v2 = v;
    // There is only one highlander! (one at one moment, regardless of a number of threads) :)
    // println!("{:?}", v);
    // error[E0382]: borrow of moved value: `v`

    // let foo = |v:Vec<i32>| ();
    // foo(v2);

    //This example, somehow, still works correctly.
    //Because, instead of owning a simple object i32-type,
    //u2 gets a copy of it.
    //Assign operator doesn't move an object if this object is
    // a value-type one
    let u = 1;
    let u2 = u;
    println!("u = {}", u);

    //Let's modify this expression:
    // let u = Box::new(1);
    // let u2 = u;
    // println!("u = {}", u);
    // See? Now we can't get to the value of 'u'
    // 'cause it's been moved to a 'u2' variable
    // error[E0382]: borrow of moved value: `u`

    //If you have a pointer to something on a heap,
    //then as you reassign it to a different variable
    //the old variable gets invalidated

    //One of the solution is to simply move the value back
    //to code:
    let print_vector = |x:Vec<i32>| -> Vec<i32>
    {
        println!("{:?}", x);
        x
    };

    let vv = print_vector(v2);
    println!("{}", vv.get(0).unwrap());

    //We can do the same action with the old variable-owner
    // if this variable is mutable:
    let mut a = vec![55, 11];
    a = print_vector(a);
    println!("{:?}", a);
}

//---BORROWING

pub fn borrowing(){
    let print_vector = |x:&Vec<i32>|
        {
            //& (reference) allows us to just borrow
            // a vector for a while and don't
            // move its data to another variable
            println!("{:?}", x);
            //still, we cannot do anything modifiable
            //with it, because our reference 'x'
            // is not mutable
        };

    let v = vec![1,2,3];
    print_vector(&v);
    println!("v[0] = {}", v[0]);

    let mut a = 40;
    let b = &mut a;
    //the 'star' (*) allows to
    // get access to the data behind
    // the reference
    *b += 20;
    //b has essentially borrowed 'a' value and modified it.
    println!("a = {}", a);
}

//---LIFETIME

struct Person{
    name: String,
}

struct Company{
    name: String,
    //as we see, compiler needs
    // the lifetime of a 'ceo' object
    // to be specified.
    // In other words, it relies on us (programmers)
    // to make all objects inside this structure
    // to live as long as structure does.
    ceo: &Person,
}

pub fn lifetimes(){
    let boss = Person{ name: "Elon Musk".to_string() };
    let tesla = Company{ name: "Tesla".to_string(), ceo: &boss };
}