// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[rustfmt::skip]
#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
        println!("Look at this other macro: {}", $x);
    }*
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
