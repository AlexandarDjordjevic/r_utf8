#[path = "../src/lib.rs"] mod utf8;


#[cfg(test)]
mod tests {
    use crate::utf8;

  
    #[test]
    fn test_ascii_to_unicode() {
        let expected : Vec<utf8::CodePoint> = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64];
        let result = utf8::ascii_to_unicode("Hello World");
        assert_eq!(expected, result);
    }
}
