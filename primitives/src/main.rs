fn main() {
    // Variables can be type annoted
    let logical: bool = true;

    let a_float: f64; // Regular annotation
    let an_integer = 5i32; //suffix annotation

    // or default will be used
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    // A type can also be inferred from context
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // Error! the type of variable cannot be changed
    // mutable = true;

    // Variables can be overwritten by shadowing
    let mutable = true;
}
