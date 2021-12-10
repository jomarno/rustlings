// macros1.rs
// Make me compile! Execute `rustlings hint macros1` for hints :)

// // I AM NOT DONE

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // SOLUTION: Added exclamation mark in following line
    my_macro!();
}
