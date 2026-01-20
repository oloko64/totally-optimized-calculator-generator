pub enum Operators {
    Add,
    Sub,
    Mul,
    Div,
}

pub fn calc_result(
    n1: u32,
    n2: u32,
    op: &Operators,
    buff: &mut impl std::io::Write,
) -> std::io::Result<()> {
    let mut int_buff_n1 = itoa::Buffer::new();
    let mut int_buff_n2 = itoa::Buffer::new();

    let itoa_n1 = int_buff_n1.format(n1);
    let itoa_n2 = int_buff_n2.format(n2);

    match op {
        Operators::Add => {
            let mut int_buff_res = itoa::Buffer::new();
            let res = int_buff_res.format(n1 + n2);
            write!(
                buff,
                "if num1 == {itoa_n1} and sign == \"+\" and num2 == {itoa_n2}:\n    print(\"{itoa_n1} + {itoa_n2} = {res}\")\n",
            )?;
        }
        Operators::Sub => {
            let mut int_buff_res = itoa::Buffer::new();
            let res = int_buff_res.format(n1 as i32 - n2 as i32);

            write!(
                buff,
                "if num1 == {itoa_n1} and sign == \"-\" and num2 == {itoa_n2}:\n    print(\"{itoa_n1} - {itoa_n2} = {res}\")\n",
            )?;
        }
        Operators::Mul => {
            let mut int_buff_res = itoa::Buffer::new();
            let res = int_buff_res.format(n1 * n2);

            write!(
                buff,
                "if num1 == {itoa_n1} and sign == \"*\" and num2 == {itoa_n2}:\n    print(\"{itoa_n1} * {itoa_n2} = {res}\")\n",
            )?;
        }
        Operators::Div => {
            if f64::from(n2) == 0.0 {
                let res = "0.0";
                write!(
                    buff,
                    "if num1 == {itoa_n1} and sign == \"/\" and num2 == {itoa_n2}:\n    print(\"{itoa_n1} / {itoa_n2} = {res}\")\n",
                )?;
            } else {
                let mut float_buff_res = ryu::Buffer::new();
                let res = float_buff_res.format(f64::from(n1) / f64::from(n2));

                write!(
                    buff,
                    "if num1 == {itoa_n1} and sign == \"/\" and num2 == {itoa_n2}:\n    print(\"{itoa_n1} / {itoa_n2} = {res}\")\n",
                )?;
            }
        }
    }

    Ok(())
}
