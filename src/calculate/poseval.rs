pub(crate) fn evaluate(input: &str) -> f32 {
    let postfix = input.to_owned();
    let mut stack: Vec<f32> = Vec::new();
    let mut digits = String::new();
    let mut is_num = false;

    for chr in postfix.clone().chars() {
        if chr.is_ascii_digit() || chr == '.' {
            digits.push(chr);
            is_num = true;
            continue
        }
        if chr == ' ' { 
            if is_num { 
                stack.push(digits.parse().unwrap()); 
                (digits, is_num) = ("".to_string(), false)
            }
            continue
        }
        calculate(&mut stack, chr)
    }
    stack.last().unwrap().to_owned()
}

fn calculate(stack: &mut Vec<f32>, operator: char) {
    let (num2, num1) = (stack.pop().unwrap() , stack.pop().unwrap());

    stack.push( match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        '^' => num1.powf(num2),
        _ => panic!("Invalid Operator")
    })
}
