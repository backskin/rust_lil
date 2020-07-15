#![allow(dead_code)]

pub fn strings(){
    // utf-8
    let s: &'static str = "Hello there!";
    // let s = "abc";
    // let letter = s[0];

    println!("{}", s);
    for letter in s.chars().rev(){
        print!("{}", letter);
    }
    println!();

    if let Some(first_char) = s.chars().nth(0){
        println!("first letter is {}", first_char);
    }

    //heap
    //String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('f' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a+=1;
    }
    println!("{}", letters);
    // &str <> String
    let _u:&str = &letters;

    //Concatenation of str and String:
    //cannot use letters + &letters;
    //first realization
    // let mut betters: &str = letters.as_str();
    let _z = letters.to_owned() + &letters;
    println!("{}", _z);
    //second one
    let mut _z: String = letters.clone();
    _z = _z + &letters;
    // second with reversing the text
    let z: String = letters.chars().rev().collect::<String>() + &letters;
    // //or
    // let mut z: String = letters.chars().rev().collect();
    // //and at last
    // z = z + &letters;
    println!("{}", z);
    //conversion from static string (literal) to String
    let mut abc = "abc".to_string();
    abc.remove(0);
    abc.push_str("def");
    println!("{}", abc.repeat(3).replace("def", "father"));
}

pub fn str_formatting(){
    // oh! it's so easy!
    let name = "Backskin".to_string();
    let greeting = format!("hi, I'm {}", name);
    println!("{}", greeting);

    let hello = "hello" ;
    let rust = "rust";
    let hello_rust = format!("{}, {}!", hello, rust);
    println!("{}", hello_rust);

    let run = "run";
    let forest = "forest";
    let rfr = format!("{0}, {1}, {0}!", run, forest);

    println!("{}", rfr);

    let response = format!("the name's {last}. {first} {last}.",
    first = "James", last = "Bond");
    println!("{}", response);

    let mixed = format!("{1}, {}, {0}, {}, {data}",
                        "alpha", "beta", data = "gamma",);

    println!("{}", mixed);
}