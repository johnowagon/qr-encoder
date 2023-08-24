extern crate qr_encoder;

#[cfg(test)]
mod tests {
    use qr_encoder::codes::smallest_cap;
    use qr_encoder::codes::ErrorCorrectionLevel;

    #[test]
    fn determine_cap1() {
        let res: String = String::from("HELLO WORLD");
        assert_eq!(smallest_cap(ErrorCorrectionLevel::Q ,res), (1, 16));
    }

    #[test]
    fn determine_cap2() {
        let res: String = String::from("HELLO THERE WORLD");
        assert_eq!(smallest_cap(ErrorCorrectionLevel::Q ,res), (2, 29));
    }

    #[test]
    fn determine_cap3() {
        let res: String = String::from("1234 5654 543490");
        assert_eq!(smallest_cap(ErrorCorrectionLevel::Q ,res), (1, 16));
    }

    #[test]
    fn determine_cap4() {
        let res: String = String::from("314159265431234567890987654321"); //30
        assert_eq!(smallest_cap(ErrorCorrectionLevel::L ,res), (1, 41));
    }

    #[test]
    fn determine_cap5() {
        let res: String = String::from("107128224107128224107128224107128224107128224107128224107128224107128224107128224107128224107128224107128224107128224107128224"); //126
        assert_eq!(smallest_cap(ErrorCorrectionLevel::M ,res), (4, 149));
    }

    #[test]
    fn determine_cap6() {
        let res: String = String::from("é, á, í, ó, and ú"); //17
        assert_eq!(smallest_cap(ErrorCorrectionLevel::Q ,res), (2, 20));
    }
}