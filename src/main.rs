use std::io;

/* #[derive(Debug)] */
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
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

enum UserChoice {
    CalculateArea,
    CheckFitting,
}

fn display_menu() {
    println!("Choose an option:");
    println!("1. Calculate the area of a rectangle");
    println!("2. Check whether a rectangle fits inside another rectangle");
}

fn get_user_choice() -> UserChoice {
    loop {
        let choice = get_input("Enter your choice (1 or 2):");
        match choice {
            1 => return UserChoice::CalculateArea,
            2 => return UserChoice::CheckFitting,
            _ => println!("Invalid choice. Please enter 1 or 2."),
        }
    }
}

fn get_rectangle_input(prompt: &str) -> Rectangle {
    let width = get_input(&format!("{} - Enter width (in pixels):", prompt));
    let height = get_input(&format!("{} - Enter height (in pixels):", prompt));

    Rectangle { width, height }
}

fn main() {
    display_menu();

    match get_user_choice() {
        UserChoice::CalculateArea => {
            let rect = get_rectangle_input("Calculate the area of the rectangle");
            println!(
                "The area of the rectangle is {} square pixels.",
                rect.area()
            );
        }
        UserChoice::CheckFitting => {
            let rect1 = get_rectangle_input("Enter dimensions of the first rectangle");
            let rect2 = get_rectangle_input("Enter dimensions of the second rectangle");

            if rect1.can_hold(&rect2) {
                println!("The first rectangle can hold the second rectangle.");
            } else {
                println!("The first rectangle cannot hold the second rectangle.");
            }
        }
    }
}
