mod utils;

use std::fs;

fn main() {
    // You can set the file name here.
    let file = "calc.py";

    // You can set the maximum number that the calculator can process here, but I don't recommend you to go over 100, it will crash when you try to open the generated file. :)
    // If you use a value of 1000 it will generate a file with +8mi lines and around 200MB.
    let maximum_number = 100;


    // ##############################################################################
    // ############################## DON'T EDIT ####################################
    // ##############################################################################
    create_header(file);
    create_body(file, maximum_number);
}

fn create_header(file: &str) {
    fs::write(file, utils::get_header_text()).expect("Unable to write to file");
}

fn create_body(file: &str, max: i32) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true) // This is needed to append to file
        .open(file)
        .unwrap();

    for op in ['+', '-', '*', '/'].iter() {
        let mut n2 = 0;
        while n2 <= max {
            for (n1, _) in (0..max + 1).enumerate() {
                let res: utils::NumType = utils::calc_result(n1.try_into().unwrap(), n2, op);
                utils::write_format_line(&mut file, n1.try_into().unwrap(), n2, op, &res);
            }
            n2 += 1;
        }
    }
    drop(file);
}
