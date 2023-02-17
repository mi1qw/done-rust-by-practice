fn main() {
    main1();
    main2();
    main3();
    main4();
    main5();
    main6();
}

// Make it work
use std::mem::size_of_val;
fn main1() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);

    println!("Success!");
}

// Make it work
fn main2() {
    let c1 = '中';
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}

// Make println! work
fn main3() {
    let _f: bool = false;

    let t = false;
    if !t {
        println!("Success!");
    }
}

// Make it work
fn main4() {
    let f = false;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}

// Make it work, don't modify `implicitly_ret_unit` !
fn main5() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}

// Modify `4` in assert to make it work
fn main6() {
    let unit: () = ();
    assert_eq!(size_of_val(&unit), 0);

    println!("Success!");
}
