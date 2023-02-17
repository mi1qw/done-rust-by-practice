fn main() {
    main0();
    main1();
    main1_1();
    main2();
    main3();
}


fn main0() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}


// Make it work with two ways
fn main1() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);

    println!("Success!");
}

fn main1_1() {
    let v = {
        let mut x = 1;
        x += 2
    };

    assert_eq!(v, ());

    println!("Success!");
}


fn main2() {
    let v = {
        let x = 3;
        x
    };

    assert_eq!(v, 3);

    println!("Success!");
}


fn main3() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}