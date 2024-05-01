// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

//to make it work
//#[macro_use]
pub mod macros {
    pub macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}
use macros::my_macro;
fn main() {
    
    my_macro!();
}
