use std::fmt;

fn main() {
    comments();
    printing();
    primitives();
}

fn comments() {
    // This is a line comment

    /*
    This is a block comment
    */

    /* This too,
     * is a block comment
     */
}

fn printing() {
    let greeting = "Hello";
    let target = "world";
    println!("{0} {1}", greeting, target);

    println!("{greeting} {target}", greeting=greeting, target=target);

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);

    // Create a structure named `Structure` which contains an `i32`.
    struct Structure(i32);

    impl fmt::Display for Structure {
        // This trait requires `fmt` with this exact signature.
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Write strictly the first element into the supplied output
            // stream: `f`. Returns `fmt::Result` which indicates whether the
            // operation succeeded or failed. Note that `write!` uses syntax which
            // is very similar to `println!`.
            write!(f, "{}", self.0)
        }
    }

    println!("This struct `{}` will print", Structure(3));
}

fn primitives() {
    // Typed constant
    // Should always be uppercase and typed
    const CONSTANT: bool = true;
    println!("{}", CONSTANT);

    // Untyped variable
    let mutable = true;
    println!("{}", mutable);

    // Typed variable
    let typed_mutable: bool = false;
    println!("{}", typed_mutable);

    // Float
    let float1 = 1.0435; // f64
    let float2 = 55;     // i32
    println!("{}", float1);
    println!("{}", float2);

    let float3: f64 = 2.643;
    println!("{}", float3);

    // Integer
    // Suffix annotation
    let integer1 = 5i32;
    println!("{}", integer1);

    // Inferred type
    let mut inferred_type = 10;
    inferred_type = 4294967296i64;
    println!("{}", inferred_type);

    // Mutable variables
    let mut mutable = 12;
    mutable = 10;
    println!("{}", mutable); // 10

    // Type cannot be changed later
    // mutable = false;

    // Shadowing
    let mutable = false;
    println!("{}", mutable); // false
}