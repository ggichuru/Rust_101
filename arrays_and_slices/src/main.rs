use std::mem;

/*
    This function borrows a slice
*/
fn analyze_slice(slice: &[i32]) {
    println!("the first element of the slice is {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // fixed size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All the elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("\n\t\nthe first element of the Array is {}", xs[0]);
    println!("the second element of the Array is {}\n", xs[1]);

    // `len` returns the count of items in the Array
    println!("number of elements in array is {}\n", xs.len());

    // arrays are stack allocated
    println!("array occupies {} bytes\n", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("\tBorrow the whole array as a slice");
    analyze_slice(&xs);

    /*
        Slices can point to a section of an array
        They are of the form [starting_index..ending_index]
        starting_index is the first position in the slice
        ending index is one more than the last position in the slice
    */
    println!("\n\t Borrow a section of the arraya as a slice");
    analyze_slice(&ys[1..4]);

    // Out of bound indexing causes compile error
    // println!("{}", xs[6])
}
