// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref temp) => println!("Co-ordinates are {},{} ", temp.x, temp.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
