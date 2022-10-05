mod utils;

use rayon::prelude::*;
use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;
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
    let maximum_number = match convert_max_to_u32(&args) {
        Ok(arg) => {
            println!("\nUsing the maximum number of \"{}\"\n", arg);
            arg
        }
        Err(_) => {
            println!(
                "\nThe maximum number is not a valid number, using the default value of: \"{}\"\n",
                default_maximum_number
            );
            default_maximum_number
        }
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
    fs::write(file, utils::get_header_text()).expect("Unable to write to file");
}

fn create_body(file: &str, max: u32) {
    let file = fs::OpenOptions::new()
        .write(true)
        .append(true) // This is needed to append to file
        .open(file)
        .unwrap();

    ['+', '-', '*', '/'].par_iter().for_each(|op| {
        let mut block_string = String::new();
        (0..=max).into_iter().for_each(|n2| {
            (0..max + 1).into_iter().enumerate().for_each(|(n1, _)| {
                let res: utils::NumType =
                    utils::calc_result(n1.try_into().unwrap(), n2.try_into().unwrap(), op);
                utils::add_to_block(&mut block_string, n1.try_into().unwrap(), n2, op, &res);
            });
        });
        write!(&file, "{}", block_string).expect("Unable to write to file");
    });
    drop(file);
}
