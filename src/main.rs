use std::collections::HashMap;

use lindenmayer::{progress, separate_stack_strings};

mod lindenmayer;

fn main() {
    let mut rules = HashMap::new();
    rules.insert('X', "F+[[X]-X]-F[-FX]+X".to_string());
    rules.insert('F', "FF".to_string());

    let string = (0..4).fold("-X".to_string(), |acc, _| {
        progress(&acc, &['+', '-', '[', ']'], &rules)
    });
    println!("String: {string}");
    let strings = separate_stack_strings(&string);
    let mut l_test = String::new();
    for string in &strings {
        l_test = "[".to_string() + &l_test + "]" + string;
    }
    println!("{l_test}");
}
