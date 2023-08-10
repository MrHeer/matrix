use std::f64::consts::PI;

pub fn to_rad(deg: f64) -> f64 {
    deg * PI / 180.
}

pub fn to_deg(rad: f64) -> f64 {
    rad * 180. / PI
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
}
