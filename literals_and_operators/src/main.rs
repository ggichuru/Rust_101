fn main() {
    //Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("4 - 6 = {}", 4i32 - 6);

    // Shortcircuiting boolean logic
    println!("true AND false = {}", true && false);
    println!("true OR false={}", true || false);
    println!("NOT true = {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Using underscore for readability
    println!("One million is written as {}", 1_000_000u32);
}

/*
    Integers 1, floats 1.2, characters 'a', strings "abc", booleans true and the unit type () can be expressed using literals.
    Integers can, alternatively, be expressed using hexadecimal, octal or binary notation using these prefixes respectively: 0x, 0o or 0b.
    Underscores can be inserted in numeric literals to improve readability, e.g. 1_000 is the same as 1000, and 0.000_001 is the same as 0.000001.
*/
