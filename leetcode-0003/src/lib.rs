use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let (mut lo, mut hi, mut res) = (0, 0, 0);
        let mut set = HashSet::new();
        while hi < bytes.len() {
            if !set.insert(bytes[hi]) {
                res = if hi - lo > res { hi - lo } else { res };
                while bytes[lo] != bytes[hi] {
                    set.remove(&bytes[lo]);
                    lo += 1;
                }
                lo += 1;
            }
            hi += 1;
        }
        res = if hi - lo > res { hi - lo } else { res };
        res as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::length_of_longest_substring("tmmzuxt".to_string()), 5);
    }
}