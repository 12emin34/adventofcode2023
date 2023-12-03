use std::io;

mod part2;
mod part1;

fn main() {
    println!("Which part would you like to run?");
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Something has gone catastrophically wrong");

    match choice.trim() {
        "1" => part1::main(),
        "2" => part2::main(),
        _ => println!("Enter 1 or 2"),
    }
}
