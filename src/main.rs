use std::fmt;

fn main() {
    comments();
    printing();
    primitives();
    strings();
    vectors_and_arrays();
    tuples();
    destructuring();
    types();
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

fn strings() {
    // Typed string
    let str: &str = "this is a string";
    println!("{}", str);

    // A heap-allocated string
    let s: String = "hello world".to_string();
    println!("{}", s);
}

#[allow(unused_variables)]
fn vectors_and_arrays() {
    let four_ints: [i32;4] = [1, 2, 3, 4];
    // four_ints.push(5);

    // Dynamic array (vector)
    let mut ints: Vec<i32> = vec![1, 2, 3, 4];
    ints.push(5);

    // Immutable view
    let slice: &[i32] = &ints;
    println!("{:?}", &slice);
}

fn tuples() {
    let x: (i32, &str, f64) = (42, "42", 42.24);
    println!("{:?}", x);
}

fn destructuring() {
    let x: (i32, &str, f64) = (42, "42", 42.24);
    let (a, b, c) = x;
    println!("{} {} {}", a, b, c);
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn types() {
    // Struct
    struct Point {
        x: i32,
        y: i32
    }

    let point: Point = Point { x: 0, y: 0 };
    println!("x: {}, y: {}", point.x, point.y);

    // Struct with unnamed fields (tuple struct)
    struct Point2(i32, i32);
    let point2: Point2 = Point2(1, 2);
    println!("{} {}", point2.0, point2.1);

    // Enum
    enum Direction {
        Up, Down, Left, Right
    }

    let up: Direction = Direction::Up;

    // Generics
    struct Foo<T> { bar: T };
    // let foo: Foo = Foo { bar: 42 };
}