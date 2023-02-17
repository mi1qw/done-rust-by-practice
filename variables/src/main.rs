// Fix the error below with least amount of modification to the code
fn main() {
    let x: i32; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !
    x = 5;
    y = 0;
    assert_eq!(x, 5);
    assert_eq!(y, 0);
    println!("Success!");

    // Fill the blanks in the code to make it compile
    let mut x: i32 = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");

    // Fix the error below with least amount of modification
    let x: i32 = 10;
    let y: i32 = 0;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);

    // Fix the error with the use of define_x

    println!("{}, world", define_x());

    fn define_x() -> &'static str {
        let x = "hello";
        &x[..]
    }
    main1();
    main2();
    main3();
    main4();
    main5();
}

// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn main1() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}

// Remove a line in the code to make it compile
fn main2() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    // let x = x;
    x += 3;

    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!";

    println!("Success!");
}

// Warning: unused variable: `x`
fn main3() {
    let x = 1;
    println!("{}", x);
}

// Fix the error below with least amount of modification
fn main4() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}


// Introduced in Rust 1.59: You can now use tuple, slice,
// and struct patterns as the left-hand side
// of an assignment.
fn main5() {
    let (mut x, mut y);
    (x, y) = (3, 4);
    println!("{} {}", x, y);
    [x, y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x, y], [1, 2]);

    println!("Success!");
}
