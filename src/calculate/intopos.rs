pub (crate) fn get_postfix(input: &str) -> String {
    let input = input.to_owned() + ")";

    let mut postfix = String::new();
    let mut stack: Vec<char> = Vec::new();

    for chr in input.chars() { 
        if chr.is_ascii_digit() || chr == '.' {
            postfix.push(chr);
            continue 
        }
        if !postfix.ends_with(' ') { postfix.push(' ') }
        stack_append(&mut stack, &mut postfix, chr)
    }

    postfix
}

fn stack_append(stack: &mut Vec<char>, postfix: &mut String, operator: char) {
    match operator {
        ')' => while let Some(x) = stack.pop() { if x == '(' { break } postfix.push(x) }
        '^' | '(' => stack.push(operator),
        '+' | '-' => {
            displace_priority(stack, postfix, "+-/*^");
            stack.push(operator)
        },
        '*' | '/' => {
            displace_priority(stack, postfix, "/*^");
            stack.push(operator)
        },
        _ => {},
    }
}

fn displace_priority( stack: &mut Vec<char>, postfix: &mut String, priority: &str) {
    while let Some(x) = stack.last() {
        if priority.contains(*x) {
            postfix.push(stack.pop().unwrap());
        } else { break }
    }
}
