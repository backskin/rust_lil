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
    println!("u = {}; u2 = {}", u, u2);

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

struct Company<'z>{
    name: String,
    //as we see, compiler needs
    // the lifetime of a 'ceo' object
    // to be specified.
    // In other words, it relies on us (programmers)
    // to make all objects inside this structure
    // to live as long as structure does.
    ceo: &'z Person,
    // when we assign the same lifetime mark to the inside object
    // by this ('z) proclamation, thus we ask a compiler to track
    // the state of this variable and to not admit it to be cleaned up,
    // until the 'Company' struct's object disappears from the scope.
}

impl Person{
    fn get_ref_name(&self) -> &String{
        &self.name
    }
}

pub fn lifetimes(){
    // let boss = Person{ name: "Elon Musk".to_string() };
    // let tesla = Company{ name: "Tesla".to_string(), ceo: &boss };
    let mut z: &String;

    // Lifetime elision example:
    {
        let p = Person {
            name: String::from("John"),
        };
        z = p.get_ref_name();
        println!("{}", *z);
    }
    // Here we cannot access the name, because the object 'p'
    // does not live long enough

    // println!("name={}", *z);

    // That's why compiler won't let us to
    // build app with these code lines

    //To avoid this issue, we may clarify the lifetime
    //of a method's argument:
    impl Person {
        fn get_ref_name_2<'a>(&'a self) -> &'a String{
            &self.name
        }
    }
    let name = String::from("Mike");
    let p = &Person{name};
    z = p.get_ref_name_2();
    // let pointer = tesla.ceo;
    // pointer.get_ref_name_2();
    println!("name={}", z);
}

struct Document<'a>{
    name: &'a str,
    info: &'a str,
}

impl<'a> Document<'a>{
    fn show(&self){
        println!("Doc: {};\n> {}", self.name, self.info);
    }
}

pub fn lifetime_in_structures() {
    let doc1 = Document { name: "Report - word 2007",
        info: "Well, the years start coming\n\
        And they don't stop coming\n\
        Fed to the rules and I hit the ground running\n\
        Didn't make sense not to live for fun\n\
        Your brain gets smart, but your head gets dumb\n\
        So much to do, so much to see,\n\
        So what's wrong with taking the backstreets?\n\
        You'll never know if you don't go\n\
        You'll never shine if you don't glow." };

    doc1.show();
}

struct Shmerson{
    name: Rc<String>,
}

impl Shmerson {
    fn new(name: Rc<String>) -> Shmerson{
        Shmerson{ name }
    }

    fn greet(&self){
        println!("Hi, my name is {}", self.name);
    }
}

use std::rc::Rc;

pub fn rc_demo(){

    fn how_many_pointers(rc: &Rc<String>){
        println!("Name = {}, it has {} strong pointers",
                 &rc, Rc::strong_count(rc));
    }

    let guy_name = Rc::new("John".to_string());
    how_many_pointers(&guy_name);
    {
        let guy = Shmerson::new(guy_name.clone());
        how_many_pointers(&guy_name);
        guy.greet();
        how_many_pointers(&guy_name);
    }
    how_many_pointers(&guy_name);
    println!("NAME = {}",guy_name);
    how_many_pointers(&guy_name);
}

use std::thread;
use std::sync::{Mutex, Arc};
use std::any::Any;
use std::borrow::Borrow;


struct Zhmerson {
    name: Arc<String>,
}

impl Zhmerson {
    //why state is a string? why not enum?
    fn new(name: Arc<String>) -> Zhmerson{
        Zhmerson{ name }
    }

    fn greet(&self){
        println!("Hi, my name is {}", self.name);
    }
}

pub fn arc_demo(){
    //When we want to pass a variable not just to man y var_names, but
    // throughout a several threads, we must use an 'Arc' pointer,
    // which is thread-safe
    let guy_name = Arc::new("John".to_string());
    let guy = Zhmerson::new(guy_name.clone());

    let t = thread::spawn(move || {
        guy.greet();
    });
    println!("Name = {}", guy_name);

    t.join().unwrap();
    //Author confessed that he cheated here,
}

struct Hrenorson{
    name: Arc<String>,
    state: Arc<Mutex<String>>,
}

impl Hrenorson{
    //why state is a string? why not enum?
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Hrenorson{
        Hrenorson{ name, state }
    }

    fn greet(&mut self){
        let mut state = self.state.lock().unwrap();

        println!("Hi, my name is {} and I am {}.", self.name, state);

        state.clear();
        state.push_str("excited");

        // self.state.clear();
        // self.state.push_str("excited");
    }
}

pub fn mutex_demo(){
    //When we want to pass a variable not just to many var_names, but
    // throughout a several threads, we must use an 'Arc' pointer,
    // which is thread-safe
    let guy_name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));
    let mut guy = Hrenorson::new(
        guy_name.clone(),
        state.clone());

    let t = thread::spawn(|| { guy.borrow().greet(); });

    println!("Name = {}, State = {}", guy_name, state.lock().unwrap().as_str());
    //we moved the 'guy' to another thread,
    //so that's why we're not allowed to use it
    // here
    // guy.greet();

    match  t.join() {
        Ok(_) => {
            guy.greet();
        },
        Err(_) => {},
    };
}

