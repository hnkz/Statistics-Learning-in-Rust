pub fn average(values: &Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;
    for val in values {
        sum += *val;
    }

    sum / values.len() as f64
}