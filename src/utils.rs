use std::fmt::Write as _;

pub enum NumType {
    Int(i32),
    Float(f64),
}

pub fn get_header_text() -> String {
    "print(\"Welcome to the calculator MK I\")
num1 = input(\"Insert the first number: \")
sign = input(\"Insert the operator (+, -, *, /): \")
num2 = input(\"Insert the second number: \")
num1 = int(num1)
num2 = int(num2)
"
    .to_string()
}

pub fn calc_result(n1: i32, n2: i32, op: &char) -> NumType {
    match op {
        '+' => NumType::Int(n1 + n2),
        '-' => NumType::Int(n1 - n2),
        '*' => NumType::Int(n1 * n2),
        '/' => {
            if n2 as f64 == 0 as f64 {
                NumType::Float(0.0)
            } else {
                NumType::Float(n1 as f64 / n2 as f64)
            }
        }
        _ => NumType::Int(0),
    }
}

pub fn add_to_block(block_string: &mut String, n1: i32, n2: i32, op: &char, res: &NumType) {
    match res {
        NumType::Int(i) => write!(block_string, "if num1 == {n1} and sign == \"{op}\" and num2 == {n2}:\n    print(\"{n1}{op}{n2} = {i}\")\n").unwrap(),
        NumType::Float(f) => write!(block_string, "if num1 == {n1} and sign == \"{op}\" and num2 == {n2}:\n    print(\"{n1}{op}{n2} = {f:.2}\")\n").unwrap(),
    };
}
