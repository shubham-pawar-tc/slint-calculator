slint::include_modules!();

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let ui = AppWindow::new().unwrap();

    let expression = Rc::new(RefCell::new(String::new()));

    // ── ADD CHARACTER ────────────────────────────────────────────────────────
    {
        let expression = expression.clone();
        let ui_handle = ui.as_weak();

        ui.on_add_number(move |value| {
            let mut expr = expression.borrow_mut();

            if value == "⌫" {
                // Backspace: remove last character
                expr.pop();
            } else {
                expr.push_str(&value);
            }

            if let Some(ui) = ui_handle.upgrade() {
                ui.set_display_text(expr.clone().into());
            }
        });
    }

    // ── CLEAR ────────────────────────────────────────────────────────────────
    {
        let expression = expression.clone();
        let ui_handle = ui.as_weak();

        ui.on_clear(move || {
            expression.borrow_mut().clear();
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_display_text("".into());
            }
        });
    }

    // ── CALCULATE ────────────────────────────────────────────────────────────
    {
        let expression = expression.clone();
        let ui_handle = ui.as_weak();

        ui.on_calculate(move || {
            let expr = expression.borrow().clone();
            let result = simple_eval(&expr);

            if let Some(ui) = ui_handle.upgrade() {
                ui.set_display_text(result.clone().into());
            }

            *expression.borrow_mut() = result;
        });
    }

    ui.run().unwrap();
}

/// Evaluate a simple arithmetic expression supporting +, -, *, /
fn simple_eval(expr: &str) -> String {
    let expr = expr.trim();

    // Tokenise: split on operators while keeping them
    // We do a simple left-to-right single-operator parse for now.
    // Find the last + or - (lowest precedence), then * or /.

    // Try each operator in reverse precedence order
    for op in ['+', '-', '*', '/'] {
        if let Some(pos) = expr.rfind(op) {
            // Make sure it's not the very first character (unary minus etc.)
            if pos == 0 { continue; }

            let left = expr[..pos].trim();
            let right = expr[pos + op.len_utf8()..].trim();

            let a = left.parse::<f64>();
            let b = right.parse::<f64>();

            if let (Ok(a), Ok(b)) = (a, b) {
                let result = match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => {
                        if b == 0.0 { return "Div/0".to_string(); }
                        a / b
                    }
                    _ => return "Error".to_string(),
                };

                // Format: show integer if no fractional part
                if result.fract() == 0.0 && result.abs() < 1e12 {
                    return (result as i64).to_string();
                } else {
                    return format!("{:.6}", result)
                        .trim_end_matches('0')
                        .trim_end_matches('.')
                        .to_string();
                }
            }
        }
    }

    "Error".to_string()
}