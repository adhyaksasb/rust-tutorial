use std::io;

fn main() {
    // explicit types (255.0f64, 255.0_f64, 255.0 as f64)

    // let x = (i32::MAX as i64) + 1; // 0 - 255
    // let y = 10_i32; // -128 -127

    // let z = x / y as i64;

    // println!("{}", z);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let int_input: i64 = input.trim().parse().unwrap();

    println!("{}", int_input + 2);
}
