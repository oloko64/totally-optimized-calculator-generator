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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let time = Instant::now();
    let file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&args.file)?;
    let mut writer = FileWriter {
        file: BufWriter::new(file),
    };
    create_header(&mut writer)?;
    create_body(&mut writer, args.max)?;

    println!(
        "Time taken to create file: {:.4}s",
        time.elapsed().as_secs_f32()
    );
    Ok(())
}

fn create_header<T>(file: &mut T) -> Result<(), std::io::Error>
where
    for<'a> T: std::io::Write,
{
    let header = r#"
print("Welcome to the calculator MK I")
num1 = input("Insert the first number: ")
sign = input("Insert the operator (+, -, *, /): ")
num2 = input("Insert the second number: ")
num1 = int(num1)
num2 = int(num2)
"#;
    file.write_all(header.as_bytes())?;
    file.flush()?;
    Ok(())
}

pub fn create_body<T>(writer: &mut T, max: u32) -> Result<(), Box<dyn std::error::Error>>
where
    for<'a> T: std::io::Write + Send,
{
    let (tx, rx) = std::sync::mpsc::sync_channel::<Vec<u8>>(64);

    std::thread::scope(|s| {
        s.spawn(move || {
            for chunk in rx {
                writer.write_all(&chunk).unwrap();
            }

            writer.flush().unwrap();
        });

        let estimated_size = (max as usize + 1) * 110;
        [Add, Sub, Mul, Div].par_iter().for_each(|op| {
            (0..=max).into_par_iter().for_each(|n2| {
                let mut buffer = Vec::with_capacity(estimated_size);
                for n1 in 0..=max {
                    utils::calc_result(n1, n2, op, &mut buffer).unwrap();
                }
                tx.send(buffer).unwrap();
            });
        });

        drop(tx); // Close the channel
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
    // file.flush()?;
    Ok(())
}

struct FileWriter {
    file: BufWriter<fs::File>,
}

impl Write for FileWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.file.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}

// #[bench]
// fn bench_create_body(b: &mut test::Bencher) {
//     b.iter(|| {
//         let buffer = Vec::new();
//         let writer = BufWriter::new(buffer);
//         create_body(writer, 100).unwrap();
//     });
// }
