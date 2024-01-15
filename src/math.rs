use std::f64::consts::PI;

const NO_NONZERO_ELEMENTS_FOUND: &str = "No nonzero elements found.";
const TOLERANCE: f64 = 1e-10;

pub fn to_rad(deg: f64) -> f64 {
    deg * PI / 180.
}

pub fn to_deg(rad: f64) -> f64 {
    rad * 180. / PI
}

pub fn eq_with_tolerance(value: f64, target: f64, tolerance: Option<f64>) -> bool {
    let tolerance = tolerance.unwrap_or(TOLERANCE);
    (value - target).abs() < tolerance
}

pub fn eq(value: f64, target: f64) -> bool {
    eq_with_tolerance(value, target, None)
}

pub fn ne_with_tolerance(value: f64, target: f64, tolerance: Option<f64>) -> bool {
    !eq_with_tolerance(value, target, tolerance)
}

pub fn ne(value: f64, target: f64) -> bool {
    !eq(value, target)
}

pub fn is_zero_with_tolerance(value: f64, tolerance: Option<f64>) -> bool {
    eq_with_tolerance(value, 0., tolerance)
}

pub fn is_zero(value: f64) -> bool {
    is_zero_with_tolerance(value, None)
}

pub fn first_nonzero_index(iter: impl IntoIterator<Item = f64>) -> Result<usize, String> {
    for (index, item) in iter.into_iter().enumerate() {
        if ne(item, 0.) {
            return Ok(index);
        }
    }
    Err(String::from(NO_NONZERO_ELEMENTS_FOUND))
}

#[cfg(test)]
mod tests {
    use crate::math::*;

    #[test]
    fn deg_to_rad() {
        assert_eq!(to_rad(90.), PI / 2.);
    }

    #[test]
    fn rad_to_deg() {
        assert_eq!(to_deg(2. * PI), 360.);
    }

    #[test]
    fn eq_case() {
        assert!(eq(0., 0.));
        assert!(!eq(0.1, 0.3));
        assert!(eq_with_tolerance(0.23, 0.24, Some(0.01)));
    }

    #[test]
    fn ne_case() {
        assert!(!ne(0., 0.));
        assert!(ne(0.1, 0.3));
        assert!(!ne_with_tolerance(0.23, 0.24, Some(0.01)));
    }

    #[test]
    fn is_zero_case() {
        assert!(is_zero(0.));
        assert!(!is_zero(0.1));
        assert!(!is_zero(1e-10));
        assert!(is_zero(1e-11));
        assert!(is_zero_with_tolerance(0.01, Some(0.1)));
    }

    #[test]
    fn first_nonzero_index_case() {
        assert_eq!(first_nonzero_index([0., 0., 2., 3.]), Ok(2));
        assert_eq!(
            first_nonzero_index([0., 0., 0.]),
            Err(String::from(NO_NONZERO_ELEMENTS_FOUND))
        );
    }
}
