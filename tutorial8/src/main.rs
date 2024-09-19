fn main() {
    println!("Hello, world!");
    add_numbers(20, 30);

    let number = {
        let x = 3;
        x + 1
    };

    let result = add_numbers(8, 3);

    println!("{}", result);

    println!("{}", number);
}

fn add_numbers(x: i32, y: i32) -> i32 {
    let result = x + y;

    if result > 10 {
        return result - 10;
    }

    result
}
