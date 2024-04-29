// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
    //println!("yes!{}",word);
    //println!("{}",word);
}

fn is_a_color_word(attempt: &String) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}