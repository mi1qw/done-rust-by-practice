fn main() {
    main1();
    main1_1();
    main1_2();
    main1_3();
    main1_4();
    main2();
    main3();
    main4();
    main4_1();
    main5();
    main6();
    main7();
    main8();
    main9();
    main9_1();
}


fn main1() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = &x;
    println!("{},{}", x, *y);
}

fn main1_1() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}", x, y);
}

fn main1_2() {
    // Use as many approaches as you can to make it work
    let x = "hello, world";
    let y = x;
    println!("{},{}", x, y);
}

fn main1_3() {
    // Use as many approaches as you can to make it work
    const STR: &str = "hello, world";
    let x = STR;
    let y = x;
    println!("{},{}", x, y);
}

fn main1_4() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = x.trim();
    // let y = x.as_str();
    println!("{},{}", x, y);
}


// Don't modify code in main!
fn main2() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}


fn main3() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // Convert String to Vec
    let _s = s.as_bytes();
    s
}


// Fix the error without removing code line
fn main4() {
    let s = String::from("hello, world");

    print_str(&s);

    println!("{}", s);
}

fn print_str(s: &String) {
    println!("{}", s)
}

// Fix the error without removing code line
fn main4_1() {
    let s = String::from("hello, world");

    print_str_(s.clone());

    println!("{}", s);
}

fn print_str_(s: String) {
    println!("{}", s)
}


// Don't use clone ,use copy instead
fn main5() {
    let x = (1, 2, (), "hello");
    let y = x.clone();
    println!("{:?}, {:?}", x, y);
}


// Mutability can be changed when ownership is transferred.
fn main6() {
    let s = String::from("hello, ");

    // Modify this line only !
    let mut s1 = s;

    s1.push_str("world");

    println!("Success!");
}


fn main7() {
    let x = Box::new(5);

    // Implement this line, dont change other lines!
    let mut y = Box::new(0);
    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}


// Partial move


fn main8() {
    let t = (String::from("hello"), String::from("world"));

    let _s = t.0;

    // Modify this line only, don't use `_s`
    println!("{:?}", t.1);
}


fn main9() {
    let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (ref s1, ref s2) = t;
    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}


fn main9_1() {
    let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (s1, s2) = t.clone();
    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}