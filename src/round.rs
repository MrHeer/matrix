pub fn round_factory(precision: usize) -> impl Fn(f64) -> f64 {
    let factor = 10.0_f64.powi(precision as i32);
    move |x: f64| (x * factor).round() / factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round() {
        let round = round_factory(3);
        assert_eq!(round(5.33983), 5.34);
        assert_eq!(round(5.9836), 5.984);
    }
}
