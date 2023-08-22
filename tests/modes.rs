extern crate qr_encoder;

#[cfg(test)]
mod tests {
    use qr_encoder::modes::analyze_mode;
    use qr_encoder::modes::Mode::*;

    #[test]
    fn determine_mode1() {
        let res: String = String::from("Test encoding function!");
        assert_eq!(analyze_mode(res), Byte);
    }

    #[test]
    fn determine_mode2() {
        let res: String = String::from("1234567890");
        assert_eq!(analyze_mode(res), Numeric);
    }

    #[test]
    fn determine_mode3() {
        let res: String = String::from("ñòóôõöÖøùúûüÆÊÿ");
        assert_eq!(analyze_mode(res), Byte);
    }

    #[test]
    fn determine_mode4() {
        let res: String = String::from("this is a test! ÿ");
        assert_eq!(analyze_mode(res), Byte);
    }

    #[test]
    fn determine_mode5() {
        let res: String = String::from("THIS SHOULD BE ALPHANUMBER");
        assert_eq!(analyze_mode(res), AlphaNumeric);
    }
}