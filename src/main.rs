#![allow(unused_imports)]

mod hard_methods_module;
mod methods_module;
mod kladov_lectures;
mod string_module;
mod collections_module;
mod study_module_two;
mod study_module;

use crate::study_module::{stack_and_heap, scope_and_shadowing};
use crate::study_module::{if_statement, while_loop, for_loop};
use crate::study_module::{match_tutorial, combination_lock, structures};
use crate::study_module::{enumerations, unions, options, arrays, slices, tuples};
use crate::study_module_two::{pattern_matching, generics};
use crate::collections_module::{vectors, hashmaps};
use crate::string_module::{strings, str_formatting};
use crate::kladov_lectures::intro;
use crate::methods_module::{functions, methods, closures, high_order_functions};
use crate::hard_methods_module::traits;

fn main () {
    println!("Main Function");
    // closures();
    // scope_and_shadowing();
    // stack_and_heap();
    // if_statement();
    // while_loop();
    // for_loop();
    // match_tutorial();
    // combination_lock();
    // structures();
    // enumerations();
    // unions();
    // options();
    // arrays();
    // slices();
    // tuples();
    // pattern_matching();
    // generics();
    // vectors();
    // hashmaps();
    // strings();
    // intro();
    // str_formatting();
    // functions();
    // methods();
    // closures();
    // high_order_functions();
    traits();
}