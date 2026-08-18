use std::io::{self, Write};

fn main() {
    println!("=== Rust Calculator ===");

    loop {
        print!("> ");

        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            println!("Goodbye!");
            break; //
        }

        match parse_and_evaluate(input) {
            Ok(result) => println!("{}", result),

            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn parse_and_evaluate(expr: &str) -> Result<f64, String> {
    let operators = ['+', '-', '*', '/', '%'];

    for op in &operators {
        if let Some(pos) = find_operator(expr, *op) {
            let left_str = expr[..pos].trim();
            let right_str = expr[pos + 1..].trim();

            let a: f64 = left_str
                .parse()
                .map_err(|_| format!("Invalid number: '{}'", left_str))?;
            let b: f64 = right_str
                .parse()
                .map_err(|_| format!("Invalid number: '{}'", right_str))?;

            return match op {
                '+' => Ok(a + b),   // addition
                '-' => Ok(a - b),   // subtration
                '*' => Ok(a * b),   // multiplication
                '/' => {
                    if b == 0.0 {
                        Err("0".to_string())
                    } else {
                        Ok(a / b)
                    }
                }
                '%' => {
                    if b == 0.0 {
                        Err("0".to_string())
                    } else {
                        Ok(a % b)   // Example: 10 % 3 = 1 (the remainder after dividing)
                    }
                }
                _ => Err("Unknown operator".to_string()),
            };
        }
    }

    Err(format!(
            "Invalid expression: '{}'. Use format: a+b, a-b, a*b, a/b, a%b",
            expr
    ))
}

// This helper function scans a string for a specific operator character
// and returns its position (index) in the string, or None if it isn't there.
//
// The tricky part: the minus sign '-' can mean TWO different things:
//   1. Subtraction:    "10-3"  (we WANT to find this)
//   2. Negative sign: "-5=3"   (we want to SKIP this)
// This function handles that distinction.
fn find_operator(expr: &str, op: char) -> Option<usize> {
    let chars: Vec<char> = expr.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if c == op {
            if op == '-' && i == 0 {
                continue;
            }

            // Another special case: if '-' comes right after another operator
            // (e.g. "5*-3"), it's making the RIGHT number negative, not subtracting.
            if op == '-' && i > 0 {
                let prev = chars[i -1];
                if "+-*/%".contains(prev) {
                    continue;
                }
            }

            return Some(i);
        }
    }

    None
}
