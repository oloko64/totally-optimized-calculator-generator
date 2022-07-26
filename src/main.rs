mod utils;

use std::fs;

fn main() {
    let file = "calc.py";
    // Don't go over 100, it will crash when you try to run open the generated file.
    // If you use a value of 1000 it will generate a file with +8mi lines and over 200MB.
    let max = 100;

    create_header(file);
    create_body(file, max);
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
