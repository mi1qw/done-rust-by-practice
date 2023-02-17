fn main() {
    main1();
    main2();
    main3();
    main4();
    main5();
    main6();
    main7();
    main8();
    main9();
    main10();
    main11();
}

// Remove something to make it work
fn main1() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;

    let z: i32 = 10; // Type of z ?

    println!("Success!");
}

// Fill the blank
fn main2() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}

// Tips: If we don't explicitly assign a type to a variable,
// then the compiler will infer one for us.
// Modify `assert_eq!` to make it work
fn main3() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

// Fill the blanks to make it work
fn main4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}

// Fix errors and panics to make it work
fn main5() {
    let v1 = 247_u8 + 8;
    let v2 = i8::checked_add(119, 8).unwrap();
    println!("{},{}", v1, v2);
}

// Modify `assert!` to make it work
fn main6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    println!("{}", v);
    assert!(v == 1597);

    println!("Success!");
}

// Fill the blank to make it work
fn main7() {
    let x = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

fn main8() {
    assert_eq!(0.1_f32 + 0.2_f32, 0.3_f32);

    println!("Success!");
}

//Two goals:
// 1. Modify assert! to make it work
// 2. Make println! output: 97 - 122
fn main9() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }
    assert_eq!(sum, -5);

    for c in 'a'..='z' {
        println!("{}", c as u8);
    }
}

// Fill the blanks
use std::ops::{Range, RangeInclusive};
fn main10() {
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}

// Fill the blanks and fix the errors
fn main11() {
    // Integer addition
    assert_eq!(1u32 + 2, 3);

    // Integer subtraction
    assert_eq!(1i32 - 2, -1);
    assert_eq!(1i8 - 2, -1);

    assert_eq!(3 * 50, 150);

    assert_eq!(9.6 / 3.2, 3_f32); // error ! make it work

    assert_eq!(24 % 5, 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert_eq!(!true, false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {:b}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
