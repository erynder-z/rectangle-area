use std::io;

/* #[derive(Debug)] */
struct Rectangle {
    width: u32,
    height: u32,
}

fn get_input(prompt: &str) -> u32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid input. Please enter a valid positive integer.");
                continue;
            }
        }
    }
}

fn main() {
    let width_input = get_input("Enter width (in pixels):");
    let height_input = get_input("Enter height (in pixels):");

    let rect1 = Rectangle {
        width: width_input,
        height: height_input,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
