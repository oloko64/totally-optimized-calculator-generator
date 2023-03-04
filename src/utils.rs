pub fn calc_result(n1: u32, n2: u32, op: char) -> String {
    match op {
        '+' => format!("if num1 == {n1} and sign == \"{op}\" and num2 == {n2}:\n    print(\"{n1} {op} {n2} = {res}\")\n", res = n1 + n2),     
        '-' => format!("if num1 == {n1} and sign == \"{op}\" and num2 == {n2}:\n    print(\"{n1} {op} {n2} = {res}\")\n", res = n1 as i32 - n2 as i32),
        '*' => format!("if num1 == {n1} and sign == \"{op}\" and num2 == {n2}:\n    print(\"{n1} {op} {n2} = {res}\")\n", res = n1 * n2),
        '/' => {
            if f64::from(n2) == 0.0 {
                format!("if num1 == {n1} and sign == \"{op}\" and num2 == {n2}:\n    print(\"{n1} {op} {n2} = {res}\")\n", res = 0.0)
            } else {
                format!("if num1 == {n1} and sign == \"{op}\" and num2 == {n2}:\n    print(\"{n1} {op} {n2} = {res:.2}\")\n", res = f64::from(n1) / f64::from(n2))
            }
        }
        _ => format!("if num1 == {n1} and sign == \"{op}\" and num2 == {n2}:\n    print(\"{n1} {op} {n2} = {res}\")\n", res = 0),
    }
}
