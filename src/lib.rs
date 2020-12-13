

pub type CodePoint = u32;

pub fn ascii_to_unicode(input: &str) -> Vec<CodePoint> {

    let mut result: Vec<CodePoint> = Vec::new();
    for u8_value in input.as_bytes() {
        result.push(*u8_value as CodePoint)
    }
    result
}
