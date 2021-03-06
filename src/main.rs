#![allow(unused_imports)]
#![allow(dead_code)]

mod circular_module;
mod lifetime_module;
mod dispatches_module;
mod traits_module;
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
use crate::traits_module::{traits, trait_params, into_the_into, drops, operator_overloading};
use crate::dispatches_module::{static_dispatches, dynamic_dispatches, why_dynamic_dispatch, vectors_of_objects};
use crate::lifetime_module::{ownership, borrowing, lifetimes, lifetime_in_structures, rc_demo, arc_demo, mutex_demo};

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
    // traits();
    // trait_params();
    // into_the_into();
    // drops();
    // operator_overloading();
    // static_dispatches();
    // dynamic_dispatches();
    // why_dynamic_dispatch();
    // vectors_of_objects();
    // ownership();
    // borrowing();
    // lifetimes();
    // lifetime_in_structures();
    // rc_demo();
    // arc_demo();
    mutex_demo();
}