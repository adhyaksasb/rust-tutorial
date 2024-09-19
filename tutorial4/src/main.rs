fn main() {
    // Scalar Data Types

    // let x: i32 = -282;

    // let y: u32 = 972;

    // let z: f64 = 43.2;

    // let is_x: bool = true;

    // let letter: char = ';';

    // Compound Data Types

    let mut tup = (1, true, 's');

    tup = (10, false, 'c');
    // let tup2: (i8, bool, char) = (1, true, 's');

    println!("{:#?}", tup);

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];

    arr[4] = 3;

    println!("{}", arr[4]);

    println!("{} + {}", tup.0, arr[4]);

    let a: u8 = 4;
    let b: i32 = a.into(); // let b: i32 = x as i32;

    println!("{}", b);
}
