


#[cfg(tests)]
mod tests {
    use crate::raagas::swars;

    #[test]
    fn test_hertz() {
        let base_hz = swars::Hertz(277.18);
        assert_eq!(base_hz, swars::SA);
    }

    #[test]
    fn test_hertz_sub_op() {
        assert_eq!(swars::Hertz(439.0), swars::SA - swars::Hertz(1.0))
    }

    #[test]
    fn test_from_float() {
        assert_eq!(swars::SA, swars::Hertz::from(440.0))
    }

}
