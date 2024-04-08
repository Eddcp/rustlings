// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let cat = ("Furry McFurson", 3.5);
    let tup : (&str, f32) = cat;

    let name = tup.0;
    let age = tup.1;

    println!("{} is {} years old.", name, age);
}
