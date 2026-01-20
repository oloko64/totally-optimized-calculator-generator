mod utils;

use clap::Parser;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::fs;
use std::io::{BufWriter, Write};
use std::time::Instant;
use utils::Operators::{Add, Div, Mul, Sub};

/// This is a calculator that generates a python file that can calculate any number from 0 to the maximum number you specify.
#[derive(Parser)]
#[command(version)]
struct Args {
    /// The file name to be generated.
    #[arg(short, long, default_value = "calc.py")]
    file: String,

    // On my machine, using 3000 takes about 1.1s to generate the 2.7GB file.
    /// The maximum number that the calculator can process. (avoid using over 3000)
    #[arg(short, long, default_value = "100")]
    max: u32,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let time = Instant::now();
    create_header(&args.file)?;
    create_body(args.file, args.max)?;
    println!(
        "Time taken to create file: {:.4}s",
        time.elapsed().as_secs_f32()
    );
    Ok(())
}

fn create_header<T: AsRef<str>>(file: T) -> Result<(), std::io::Error> {
    let header = r#"
print("Welcome to the calculator MK I")
num1 = input("Insert the first number: ")
sign = input("Insert the operator (+, -, *, /): ")
num2 = input("Insert the second number: ")
num1 = int(num1)
num2 = int(num2)
"#;
    fs::write(file.as_ref(), header)?;
    Ok(())
}

fn create_body<T>(file: T, max: u32) -> Result<(), std::io::Error>
where
    T: AsRef<str>,
{
    let mut file = BufWriter::new(
        fs::OpenOptions::new()
            .append(true) // This is needed to append to file
            .open(file.as_ref())?,
    );

    let arc_file = std::sync::Mutex::new(&mut file);
    [Add, Sub, Mul, Div].par_iter().for_each(|op| {
        (0..=max).into_par_iter().for_each(|n2| {
            let mut buffer = String::new();
            for n1 in 0..=max {
                let res = utils::calc_result(n1, n2, op);
                buffer.push_str(&res);
            }
            let mut file = arc_file.lock().unwrap();
            file.write_all(buffer.as_bytes()).unwrap();
        });
    });

    // Old code without parallelism
    // for op in [Add, Sub, Mul, Div] {
    //     for n2 in 0..=max {
    //         for n1 in 0..=max {
    //             let res = utils::calc_result(n1, n2, &op);
    //             file.write_all(res.as_bytes())?;
    //         }
    //     }
    // }
    file.flush()?;
    Ok(())
}
