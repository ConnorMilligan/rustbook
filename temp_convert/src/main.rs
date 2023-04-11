use std::io;

fn main() {
    println!("Welcome to the temp converter!");

    loop {
        println!("Would you like to convert from (C)elcius or (F)ahrenheit? (Q)uit.");
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: char = match choice.trim().parse() {
            Ok(opt) => opt,
            Err(_) => continue,
        };

        match choice {
            'C' => convert_to_f(),
            'F' => convert_to_c(),
            'Q' => break,
            _ => println!("Incorrect input, is it capitalized?")
        }

    }
    println!("Goodbye!");
}

fn convert_to_c() {
    println!(" ~ Converting to C ~ ");
    loop {
        println!("Please enter the degrees in F: ");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: Option<i32> = match choice.trim().parse() {
            Ok(opt) => Some(opt),
            Err(_) => None,
        };

        if choice.is_some() {
            println!("{} C is {} in F", choice.unwrap(), (choice.unwrap()-32)*5/9);
            break;
        } else {
            println!("Invalid input!");
        }
        
    }
}

fn convert_to_f() {
    println!(" ~ Converting to F ~ ");
    loop {
        println!("Please enter the degrees in C: ");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: Option<i32> = match choice.trim().parse() {
            Ok(opt) => Some(opt),
            Err(_) => None,
        };

        if choice.is_some() {
            println!("{} F is {} in C", choice.unwrap(), choice.unwrap()*9/5+32);
            break;
        } else {
            println!("Invalid input!");
        }
        
    }
}