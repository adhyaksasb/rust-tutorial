fn main() {
    let cond = 2 as f32 <= 2.2;

    let cond2 = false || !cond;

    let food = "fruit";

    if food == "cookie" {
        println!("I like cookies too");
    } else if food == "fruit" {
        println!("That sounds healthy!");
    } else {
        println!("Oh, that's too bad!");
    }

    match food {
        "cookie" => println!("Yummy Cookie"),
        "fruit" => println!("Boooring"),
        _ => println!("That's too bad!"),
    }

    println!("{}", cond2);
}
