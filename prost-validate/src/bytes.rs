pub trait ValidateBytesExt {
    fn contains(&self, needle: &[u8]) -> bool;
}

impl ValidateBytesExt for Vec<u8> {
    fn contains(&self, needle: &[u8]) -> bool {
        let len = needle.len();
        if len == 0 {
            return true;
        }
        self.windows(len).any(move |sub_slice| sub_slice == needle)
    }
}

impl ValidateBytesExt for &Vec<u8> {
    fn contains(&self, needle: &[u8]) -> bool {
        let len = needle.len();
        if len == 0 {
            return true;
        }
        self.windows(len).any(move |sub_slice| sub_slice == needle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let haystack = vec![1, 2, 3, 4, 5];
        let needle = vec![3, 4];
        assert!(haystack.contains(&needle));
    }
}
