#![allow(dead_code)]
use std::vec;
use std::collections::HashMap;

pub(crate) fn vectors(){
    let mut a = vec::Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);

    a.push(44);

    println!("a = {:?}", a);

    let idx:usize = 1;
    a[idx] = 321;
    let some_idx = 6;
    println!("{}",match a.get(some_idx) {
        Some(res) => *res,
        None => {
            println!("index {} is out of bounds \
            (true index might be between [0..{}])",
                     some_idx, a.len()-1);
            0
    }});

    let true_idx = 2;

    match a.get(true_idx) {
        Some(x) => println!("Element a[{}] = {}", true_idx, x),
        None => println!("Error! No such element with idx={}", true_idx),
    }

    println!("a[{}] = {}",idx, a[idx]);
    println!("a = {:?}", a);
    a.push(77);
    for x in &a {
        print!("{} ", x);
    }
    println!();
    a.push(154);
    println!("{:?}", a);

    let last_elem = match  a.pop() {
        Some(elem) => elem,
        None => 0,
    };
    println!("last elem is {:?}, now a = {:?}", last_elem, a);

}

pub fn hashmaps(){
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    // println!("a square has {} sides", shapes["circle".into()]);

    for (key, value) in &shapes {
        println!("{}: {}", key, value);
    }

    shapes.insert("pentagon".into(), 5);

    println!("{:?}", shapes);

    shapes.entry("circle".into()).or_insert(1);
    {
        let actual = shapes.entry("circle".into());
        println!("{}", actual.or_insert(3));
    }

    println!("{:?}", shapes);

}
