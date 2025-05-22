pub fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    }
    else {
        Some(a / b)
    }
}