slint::include_modules!();

use std::cell::RefCell;
use std::rc::Rc;

pub fn run_app() {
    let ui = AppWindow::new().unwrap();
    let expression = Rc::new(RefCell::new(String::new()));

    {
        let expression = expression.clone();
        let ui_handle = ui.as_weak();
        ui.on_add_number(move |value| {
            let mut expr = expression.borrow_mut();
            if value == "⌫" { expr.pop(); } else { expr.push_str(&value); }
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_display_text(expr.clone().into());
            }
        });
    }

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

fn simple_eval(expr: &str) -> String {
    let expr = expr.trim();
    for op in ['+', '-', '*', '/'] {
        if let Some(pos) = expr.rfind(op) {
            if pos == 0 { continue; }
            let left = expr[..pos].trim();
            let right = expr[pos + op.len_utf8()..].trim();
            if let (Ok(a), Ok(b)) = (left.parse::<f64>(), right.parse::<f64>()) {
                let result = match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => { if b == 0.0 { return "Div/0".to_string(); } a / b }
                    _ => return "Error".to_string(),
                };
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

#[cfg(target_os = "android")]
#[no_mangle]
extern "C" fn android_main(app: i_slint_backend_android_activity::android_activity::AndroidApp) {
    slint::android::init(app).unwrap();
    run_app();  // ✅ now visible because run_app is defined above in same file
}