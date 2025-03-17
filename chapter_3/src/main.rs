fn main() {
    /*
    Data Types

    Primitives

    | Length   | Signed  | Unsigned |
    |----------|---------|----------|
    | 8-bit    | i8      | u8       |
    | 16-bit   | i16     | u16      |
    | 32-bit   | i32     | u32      |
    | 64-bit   | i64     | u64      |
    | 128-bit  | i128    | u128     |
    | arch     | isize   | usize    |

    These can be written in base-10, binary, octal, or byte. (u8 only)

    | Number literals | Example     |
    |-----------------|-------------|
    | Decimal         | 98_222      |
    | Hex             | 0xff        |
    | Octal           | 0o77        |
    | Binary          | 0b1111_0000 |
    | Byte (u8 only)  | b'A'        |

    Float is 32 or 64 bit. Default is f64, as it's usually just as fast on
    modern architecture.
    */
    let eight_bit: u8 = u8::MAX;
    let sixteen_bit: u16 = u16::MAX;
    let thirty_two: u32 = u32::MAX;
    let sixty_four: u64 = u64::MAX;
    let one_twenty_eight: u128 = u128::MAX;
    let arch: usize = usize::MAX;

    println!("8-bit unsigned max: {eight_bit}.");
    println!("16-bit unsigned max: {sixteen_bit}.");
    println!("32-bit unsigned max: {thirty_two}.");
    println!("64-bit unsigned max: {sixty_four}.");
    println!("128-bit unsigned max: {one_twenty_eight}.");
    println!("Arch unsigned max: {arch}.");

    // Compound Data Types
    /*
    Tuples
    - Fixed length
    - Heterogeneous
    - Fixed size
    - Comma separated list of values
    - Accessed via destructuring or dot notation.
    */
    let tup: (i32, char, bool) = (42, 'X', true);
    let (int, x, is_true) = tup;
    // Could also use tup.0, tup.1, tup.2

    println!("Int: {int}.");
    println!("X: {x}");
    println!("is_true: {is_true}");

    /*
    Arrays
    - Fixed length
    - Homogenous
    - Fixed size
    - CSL
    - Accessed via bracket notation
    - Can be pre-filled with same value
    */
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [0, 1, 2, 3, 4];
    let b = [0; 5]; // Creates a pre-plotted array filled with zeroes.

    for month in months {
        println!("{month}");
    }
    for element in a {
        println!("{element}");
    }
    for element in b {
        println!("{element}");
    }

    /*
    Control Flow

    3 kinds of loops:
    - loop
    - while
    - for

    Can use labels to assist with controlled breaks and continues within nested loops.
    */
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    /*
    For loops

    Handled similarly to most languages.
    Range very useful for iterating a set number of times.
    Range is NOT inclusive.
    */
    for number in (1..4).rev() { // .rev() reverses the range.
        println!("{number}!");
    }
    println!("LIFTOFF!");
}
