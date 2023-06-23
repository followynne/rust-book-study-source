use std::io;

mod fibonacci;
mod lesson_code;
mod temp_converter;
mod christmas;

fn main() {
    loop {
        println!("1 for the lesson code, 2 for fahreneit to celsius, 3 for celsius to fahr, 4 for fibonacci, 5 christmas carol, 6 to exit");

        let mut result = String::new();

        io::stdin()
            .read_line(&mut result)
            .expect("failed to read line");

        let result: i32 = match result.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match result {
            1 => lesson_code::lesson_code(),
            2 => temp_converter::from_fahreneit_to_celsius(15.0, false),
            3 => temp_converter::from_fahreneit_to_celsius(15.0, true),
            4 => println!("fibonacci: {}", fibonacci::fibonacci(97)),
            5 => christmas::carol(),
            6 => break,
            _ => continue,
        }
    }

    println!("bye!");
}
