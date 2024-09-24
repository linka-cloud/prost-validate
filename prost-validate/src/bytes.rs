pub trait ValidateBytes {
    fn contains(&self, needle: &[u8]) -> bool;
}

impl ValidateBytes for Vec<u8> {
    fn contains(&self, needle: &[u8]) -> bool {
        let len = needle.len();
        if len == 0 {
            return true;
        }
        self.windows(len).any(move |sub_slice| sub_slice == needle)
    }
}

impl ValidateBytes for &Vec<u8> {
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
        assert!(ValidateBytes::contains(&haystack, &needle));
        assert!(haystack.contains(&needle));
    }
}
