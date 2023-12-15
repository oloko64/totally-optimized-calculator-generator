pub enum Operators {
    Add,
    Sub,
    Mul,
    Div,
}

pub fn calc_result(n1: u32, n2: u32, op: &Operators) -> String {
    match op {
        Operators::Add => format!("if num1 == {n1} and sign == \"+\" and num2 == {n2}:\n    print(\"{n1} + {n2} = {res}\")\n", res = n1 + n2),     
        Operators::Sub => format!("if num1 == {n1} and sign == \"-\" and num2 == {n2}:\n    print(\"{n1} - {n2} = {res}\")\n", res = n1 as i32 - n2 as i32),
        Operators::Mul => format!("if num1 == {n1} and sign == \"*\" and num2 == {n2}:\n    print(\"{n1} * {n2} = {res}\")\n", res = n1 * n2),
        Operators::Div if f64::from(n1) == 0.0 => format!("if num1 == {n1} and sign == \"/\" and num2 == {n2}:\n    print(\"{n1} / {n2} = {res}\")\n", res = 0.0),
        Operators::Div => format!("if num1 == {n1} and sign == \"/\" and num2 == {n2}:\n    print(\"{n1} / {n2} = {res:.2}\")\n", res = f64::from(n1) / f64::from(n2))
    }
}
