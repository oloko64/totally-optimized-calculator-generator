mod utils;

use rayon::prelude::*;
use std::env;
use std::error::Error;
use std::fs;
use std::io::{BufWriter, Write};
use std::sync::Mutex;
use std::time::Instant;

fn main() {
    let time = Instant::now();
    // You can set the file name here.
    let file = "calc.py";

    // You can set the maximum number that the calculator can process here, but I don't recommend you to go over 100, it will crash when you try to open the generated file. :)
    // If you use a value of 1000 it will generate a file with +8mi lines and around 200MB.
    let default_maximum_number: u32 = 100;

    // ##############################################################################
    // ############################## DON'T EDIT ####################################
    // ##############################################################################
    let args = env::args().collect::<Vec<_>>();
    let maximum_number = if let Ok(arg) = convert_max_to_u32(&args) {
        println!("\nUsing the maximum number of \"{arg}\"\n");
        arg
    } else {
        println!(
                     "\nThe maximum number is not a valid number, using the default value of: \"{default_maximum_number}\"\n"
                 );
        default_maximum_number
    };
    create_header(file);
    create_body(file, maximum_number);
    println!(
        "Time taken to create file: {:.4}s",
        time.elapsed().as_secs_f32()
    );
}

fn convert_max_to_u32(args: &Vec<String>) -> Result<u32, Box<dyn Error>> {
    if args.len() < 2 {
        return Err(From::from("The maximum number is not specified"));
    }
    Ok(args[1].parse::<u32>()?)
}

fn create_header(file: &str) {
    let header = r#"
print("Welcome to the calculator MK I")
num1 = input("Insert the first number: ")
sign = input("Insert the operator (+, -, *, /): ")
num2 = input("Insert the second number: ")
num1 = int(num1)
num2 = int(num2)
"#;
    fs::write(file, header).expect("Unable to write to file");
}

fn create_body(file: &str, max: u32) {
    let file = Mutex::new(BufWriter::new(
        fs::OpenOptions::new()
            .write(true)
            .append(true) // This is needed to append to file
            .open(file)
            .unwrap(),
    ));

    ['+', '-', '*', '/'].par_iter().for_each(|op| {
        for n2 in 0..=max {
            for n1 in 0..=max {
                let res = utils::calc_result(n1, n2, *op);
                file.lock().unwrap().write_all(res.as_bytes()).unwrap();
            }
        }
        file.lock().unwrap().flush().unwrap();
    });
}
