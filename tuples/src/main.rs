mod tup;

// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // let can be used to bind the member pairs of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn hello_world() {
    println!("Hello world");
}

// The following struct is for the activity
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

// fn transpose(mat: Matrix) -> Matrix {
//     let matrix = mat;
//     (matrix.0, matrix.2, matrix.1, matrix.3);
// }

fn main() {
    // A tupple with a bunch of different types
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // values can be access from the tupple using long indexing
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tupple second value: {}", long_tuple.1);

    // Tuples can be tupple numbers
    let tuple_of_tupples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tupples);

    // But long tupples cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // Calling the hello world function
    hello_world();
    tup::tup_fn();

    // tuples can be desctructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 1.3, 1.4);
    println!("Matrix:\n{:?}", matrix);
    // println!("Transpose:\n{:?}", reverse(matrix))
}
