fn main() {
    // Let Variable
    let x = 4;
    // let mut x = 4;
    println!("x is: {}", x);

    {
        let x = x - 2;
        println!("x is: {}", x);
    }

    let x = x + 1;

    // x = 5;
    println!("x is: {}", x);

    let x = "hello";
    println!("x is: {}", x);

    // Const Variable
    const SECONDS_IN_MINUTE: u32 = 60;
    // const SECONDS_IN_MINUTE: u32 = 100;

    println!("const: {}", SECONDS_IN_MINUTE);
}
